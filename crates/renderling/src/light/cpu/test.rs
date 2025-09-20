//! Tests of the lighting system.

use glam::{Vec3, Vec4, Vec4Swizzles};

use spirv_std::num_traits::Zero;

use crate::{
    bvol::BoundingBox,
    camera::Camera,
    color::linear_xfer_vec4,
    context::Context,
    geometry::Vertex,
    light::{shader::SpotLightCalculation, LightTiling, LightTilingConfig},
    math::GpuRng,
    primitive::{shader::PrimitivePbrVertexInfo, Primitive},
    stage::Stage,
    test::BlockOnFuture,
    transform::shader::TransformDescriptor,
};

use super::*;

#[test]
/// Ensures that a spot light can determine if a point lies inside or outside its cone
/// of emission.
fn spot_one_calc() {
    let (doc, _, _) = gltf::import(
        crate::test::workspace_dir()
            .join("gltf")
            .join("spot_one.glb"),
    )
    .unwrap();
    let light = doc.lights().unwrap().next().unwrap();
    let spot = if let gltf::khr_lights_punctual::Kind::Spot {
        inner_cone_angle,
        outer_cone_angle,
    } = light.kind()
    {
        (inner_cone_angle, outer_cone_angle)
    } else {
        panic!("not a spot light");
    };
    log::info!("spot: {spot:#?}");

    let light_node = doc.nodes().find(|node| node.light().is_some()).unwrap();
    let parent_transform = TransformDescriptor::from(light_node.transform());
    log::info!("parent_transform: {parent_transform:#?}");

    let spot_descriptor = SpotLightDescriptor {
        position: Vec3::ZERO,
        direction: Vec3::NEG_Z,
        inner_cutoff: spot.0,
        outer_cutoff: spot.1,
        color: Vec3::from(light.color()).extend(1.0),
        intensity: light.intensity(),
    };

    let specific_points = [
        (Vec3::ZERO, true, true, Some(1.0)),
        (Vec3::new(0.5, 0.0, 0.0), false, true, None),
        (Vec3::new(0.5, 0.0, 0.5), false, false, None),
        (Vec3::new(1.0, 0.0, 0.0), false, false, Some(0.0)),
    ];
    for (i, (point, inside_inner, inside_outer, maybe_contribution)) in
        specific_points.into_iter().enumerate()
    {
        log::info!("{i} descriptor: {spot_descriptor:#?}");
        let spot_calc = SpotLightCalculation::new(spot_descriptor, parent_transform.into(), point);
        log::info!("{i} spot_calc@{point}:\n{spot_calc:#?}");
        assert_eq!(
            (inside_inner, inside_outer),
            (
                spot_calc.fragment_is_inside_inner_cone,
                spot_calc.fragment_is_inside_outer_cone
            ),
        );
        if let Some(expected_contribution) = maybe_contribution {
            assert_eq!(expected_contribution, spot_calc.contribution);
        }
    }
}

#[test]
/// Ensures that a spot light illuminates only the objects within its cone of
/// emission.
fn spot_one_frame() {
    let m = 32.0;
    let (w, h) = (16.0f32 * m, 9.0 * m);
    let ctx = Context::headless(w as u32, h as u32).block();
    let stage = ctx.new_stage().with_msaa_sample_count(4);
    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("spot_one.glb"),
        )
        .unwrap();
    let camera = doc.cameras.first().unwrap();
    camera
        .as_ref()
        .set_projection(crate::camera::perspective(w, h));
    stage.use_camera(camera);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().block().unwrap();
    img_diff::assert_img_eq("light/spot_lights/one.png", img);
    frame.present();
}

#[test]
/// Test the spot lights.
///
/// This should render a cube with two spot lights illuminating a spot on two
/// of its sides.
fn spot_lights() {
    let w = 800.0;
    let h = 800.0;
    let ctx = Context::headless(w as u32, h as u32).block();
    let stage = ctx
        .new_stage()
        .with_lighting(true)
        .with_msaa_sample_count(4);

    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("spot_lights.glb"),
        )
        .unwrap();
    let camera = doc.cameras.first().unwrap();
    camera
        .as_ref()
        .set_projection(crate::camera::perspective(w, h));
    stage.use_camera(camera);

    let down_light = doc.lights.first().unwrap();
    log::info!("down_light: {:#?}", down_light.as_spot().unwrap());

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().block().unwrap();
    img_diff::assert_img_eq("light/spot_lights/frame.png", img);
    frame.present();
}

#[test]
fn light_tiling_light_bounds() {
    let magnification = 8;
    let w = 16.0 * 2.0f32.powi(magnification);
    let h = 9.0 * 2.0f32.powi(magnification);
    let ctx = Context::headless(w as u32, h as u32).block();
    let stage = ctx.new_stage().with_msaa_sample_count(4);
    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();
    let camera = doc.cameras.first().unwrap();

    stage.use_camera(camera);

    let _lights = crate::test::make_two_directional_light_setup(&stage);

    // Here we only want to render the bounding boxes of the renderlets,
    // so mark the renderlets themeselves invisible
    doc.renderlets_iter().for_each(|r| {
        r.set_visible(false);
    });

    let colors = [0x6DE1D2FF, 0xFFD63AFF, 0x6DE1D2FF, 0xF75A5AFF].map(|albedo_factor| {
        stage.new_material().with_albedo_factor({
            let mut color = crate::math::hex_to_vec4(albedo_factor);
            linear_xfer_vec4(&mut color);
            color
        })
    });
    let mut resources = vec![];
    for (i, node) in doc.nodes.iter().enumerate() {
        if node.mesh.is_none() {
            continue;
        }
        let transform = Mat4::from(node.transform.global_descriptor());
        if let Some(mesh_index) = node.mesh {
            log::info!("mesh: {}", node.name.as_deref().unwrap_or("unknown"));
            let mesh = &doc.meshes[mesh_index];
            for prim in mesh.primitives.iter() {
                let (min, max) = prim.bounding_box;
                let min = transform.transform_point3(min);
                let max = transform.transform_point3(max);
                let bb = BoundingBox::from_min_max(min, max);
                if bb.half_extent.min_element().is_zero() {
                    log::warn!("bounding box is not a volume, skipping");
                    continue;
                }
                log::info!("min: {min}, max: {max}");
                resources.push(
                    stage
                        .new_primitive()
                        .with_vertices(
                            stage.new_vertices(
                                bb.get_mesh().map(|(p, n)| {
                                    Vertex::default().with_position(p).with_normal(n)
                                }),
                            ),
                        )
                        .with_material(&colors[i % colors.len()]),
                );
            }
        }
    }

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().block().unwrap();
    img_diff::save("light/tiling/bounds.png", img);
    frame.present();
}

fn gen_vec3(prng: &mut GpuRng) -> Vec3 {
    let x = prng.gen_f32(-120.0, 120.0);
    let y = prng.gen_f32(0.0, 80.0);
    let z = prng.gen_f32(-120.0, 120.0);
    Vec3::new(x, y, z)
}

struct GeneratedLight {
    _unused_transform: Transform,
    _light: AnalyticalLight<PointLight>,
    mesh_renderlet: Primitive,
}

fn gen_light(stage: &Stage, prng: &mut GpuRng, bounding_boxes: &[BoundingBox]) -> GeneratedLight {
    let mut position = gen_vec3(prng);
    while bounding_boxes.iter().any(|bb| bb.contains_point(position)) {
        position = gen_vec3(prng);
    }
    assert!(!position.x.is_nan());
    assert!(!position.y.is_nan());
    assert!(!position.z.is_nan());

    let color = Vec4::new(
        prng.gen_f32(0.0, 1.0),
        prng.gen_f32(0.0, 1.0),
        prng.gen_f32(0.0, 1.0),
        1.0,
    );

    let scale = prng.gen_f32(0.1, 1.0);

    let light_bb = BoundingBox {
        center: Vec3::ZERO,
        half_extent: Vec3::new(scale, scale, scale) * 0.5,
    };

    let _unused_transform = stage.new_transform().with_translation(position);
    let vertices = stage.new_vertices(
        light_bb
            .get_mesh()
            .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
    );
    let material = stage
        .new_material()
        .with_albedo_factor(color)
        .with_has_lighting(false)
        .with_emissive_factor(color.xyz())
        .with_emissive_strength_multiplier(100.0);
    let mesh_renderlet = stage
        .new_primitive()
        .with_vertices(vertices)
        .with_material(material);
    let _light = {
        // suffix the actual analytical light
        let intensity = scale * 100.0;
        stage
            .new_point_light()
            .with_position(position)
            .with_color(color)
            .with_intensity(intensity)
    };

    GeneratedLight {
        _unused_transform,
        _light,
        mesh_renderlet,
    }
}

fn size() -> UVec2 {
    UVec2::new(
        (10.0 * 2.0f32.powi(8)) as u32,
        (9.0 * 2.0f32.powi(8)) as u32,
    )
}

fn make_camera(stage: &Stage) -> Camera {
    let size = size();
    let eye = Vec3::new(250.0, 200.0, 250.0);
    let target = Vec3::ZERO;
    log::info!("make_camera: forward {}", (target - eye).normalize());
    stage.new_camera().with_projection_and_view(
        Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_4,
            size.x as f32 / size.y as f32,
            50.0,
            600.0,
        ),
        Mat4::look_at_rh(eye, target, Vec3::Y),
    )
}

/// Ensures that `LightTile`s are cleared by the clear_tiles shader.
#[test]
fn clear_tiles_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let s = 256;
    let depth_texture_size = UVec2::splat(s);
    let ctx = Context::headless(s, s).block();
    let stage = ctx.new_stage();
    let lighting: &Lighting = stage.as_ref();
    let tiling_config = LightTilingConfig::default();
    let tiling = LightTiling::new_hybrid(lighting, false, depth_texture_size, tiling_config);
    let desc = tiling.tiling_descriptor.get();
    let tile_dimensions = desc.tile_grid_size();

    // Write to the tiles to ensure we know the starting state, that way we can
    // ensure each step of tiling is correct.
    {
        let mut rng = GpuRng::new(0);
        let max_distance = UVec2::ZERO.manhattan_distance(tile_dimensions) as f32;
        for i in 0..tiling.tiles().len() {
            tiling.tiles().modify(i, |item| {
                let x = i as u32 % tile_dimensions.x;
                let y = i as u32 / tile_dimensions.x;
                let tile_coord = UVec2::new(x, y);
                let distance = tile_coord.manhattan_distance(tile_dimensions) as f32;
                // This should produce an image where pixels get darker towards the lower right corner.
                let min = distance / max_distance;
                // This should produce an image where pixels get darker towards the upper left corner.
                let max = 1.0 - distance / max_distance;

                item.depth_min = crate::light::shader::quantize_depth_f32_to_u32(min);
                item.depth_max = crate::light::shader::quantize_depth_f32_to_u32(max);

                // This should produce an image that looks like noise
                item.next_light_index = rng.gen_u32(0, 32);
            });
        }
        let _ = lighting.commit();
        ctx.get_device().poll(wgpu::PollType::Wait).unwrap();

        let (mins, maxs, lights) = futures_lite::future::block_on(tiling.read_images(lighting));
        img_diff::assert_img_eq("light/tiling/clear_tiles/1-mins.png", mins);
        img_diff::assert_img_eq("light/tiling/clear_tiles/1-maxs.png", maxs);
        img_diff::assert_img_eq("light/tiling/clear_tiles/1-lights.png", lights);
    }

    // Run the clear_tiles shader to ensure that the tiles are cleared.
    {
        tiling.prepare(lighting, depth_texture_size);
        let stage_commit_result = stage.commit();
        let bindgroup = tiling.get_bindgroup(
            ctx.get_device(),
            &stage_commit_result.geometry_buffer,
            &stage_commit_result.lighting_buffer,
            &stage.depth_texture.read().unwrap(),
        );
        let label = Some("light-tiling-clear-tiles-test");
        let mut encoder = ctx
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            tiling.clear_tiles(&mut encoder, bindgroup.as_ref(), depth_texture_size);
        }
        ctx.runtime().queue.submit(Some(encoder.finish()));

        let (mins, maxs, lights) = futures_lite::future::block_on(tiling.read_images(lighting));
        img_diff::assert_img_eq("light/tiling/clear_tiles/2-mins.png", mins);
        img_diff::assert_img_eq("light/tiling/clear_tiles/2-maxs.png", maxs);
        img_diff::assert_img_eq("light/tiling/clear_tiles/2-lights.png", lights);
    }
}

#[test]
fn min_max_depth_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let s = 256;
    let depth_texture_size = UVec2::splat(s);
    let ctx = Context::headless(s, s).block();
    let stage = ctx.new_stage();
    let _doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();
    let camera = make_camera(&stage);
    stage.use_camera(camera);
    snapshot(
        &ctx,
        &stage,
        "light/tiling/min_max_depth/1-scene.png",
        false,
    );

    let lighting = &stage.lighting;
    let tiling = LightTiling::new_hybrid(lighting, false, depth_texture_size, Default::default());
    tiling.prepare(lighting, depth_texture_size);

    let stage_commit_result = stage.commit();
    let bindgroup = tiling.get_bindgroup(
        ctx.get_device(),
        &stage_commit_result.geometry_buffer,
        &stage_commit_result.lighting_buffer,
        &stage.depth_texture.read().unwrap(),
    );
    let label = Some("light-tiling-min-max-depth-test");

    // Clear the tiles, which is verified in `clear_tiles_sanity`, then assert the min/max depth
    {
        let mut encoder = ctx
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            tiling.clear_tiles(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            tiling.compute_min_max_depth(&mut encoder, bindgroup.as_ref(), depth_texture_size);
        }
        ctx.runtime().queue.submit(Some(encoder.finish()));
        let (mins, maxs, _lights) = futures_lite::future::block_on(tiling.read_images(lighting));
        img_diff::assert_img_eq("light/tiling/min_max_depth/2-mins.png", mins);
        img_diff::assert_img_eq("light/tiling/min_max_depth/2-maxs.png", maxs);
    }
}

#[test]
fn light_bins_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let s = 256;
    let depth_texture_size = UVec2::splat(s);
    let ctx = Context::headless(s, s).block();
    let stage = ctx.new_stage();
    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();
    let camera = make_camera(&stage);
    stage.use_camera(camera);
    snapshot(&ctx, &stage, "light/tiling/bins/1-scene.png", false);

    let lighting = &stage.lighting;
    let tiling = LightTiling::new_hybrid(lighting, false, depth_texture_size, Default::default());
    assert_eq!(
        tiling.tiling_descriptor.get().tiles_array,
        tiling.tiles().array()
    );
    tiling.prepare(lighting, depth_texture_size);

    let stage_commit_result = stage.commit();
    let bindgroup = tiling.get_bindgroup(
        ctx.get_device(),
        &stage_commit_result.geometry_buffer,
        &stage_commit_result.lighting_buffer,
        &stage.depth_texture.read().unwrap(),
    );
    let label = Some("light-tiling-min-max-depth-test");

    {
        {
            let mut encoder = ctx
                .get_device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            tiling.clear_tiles(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            tiling.compute_min_max_depth(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            tiling.compute_bins(&mut encoder, bindgroup.as_ref(), depth_texture_size);
            ctx.runtime().queue.submit(Some(encoder.finish()));
        }
        let (mut mins, mut maxs, mut lights) =
            futures_lite::future::block_on(tiling.read_images(lighting));
        img_diff::normalize_gray_img(&mut mins);
        img_diff::normalize_gray_img(&mut maxs);
        img_diff::normalize_gray_img(&mut lights);
        img_diff::assert_img_eq("light/tiling/bins/2-mins.png", mins);
        img_diff::assert_img_eq("light/tiling/bins/2-maxs.png", maxs);
        img_diff::assert_img_eq("light/tiling/bins/2-lights.png", lights);
    }
    let directional_light = doc.lights.first().unwrap();
    let tiles = futures_lite::future::block_on(tiling.read_tiles(lighting));
    for tile in tiles.into_iter() {
        let light_bin =
            futures_lite::future::block_on(lighting.light_slab.read_array(tile.lights_array))
                .unwrap();
        // Assert either the light is the correct one, or we're using the zero frustum optimization
        // discussed in <http://renderling.xyz/articles/live/light_tiling.html#zero-volume-frustum-optimization>
        if tile.depth_min != tile.depth_max {
            assert_eq!(light_bin[0], directional_light.id());
            assert_eq!(light_bin[1], Id::NONE);
        } else {
            assert_eq!(0, tile.next_light_index);
            assert_eq!(light_bin[0], Id::NONE);
        }
    }
}

// Ensures point lights are being binned properly.
#[test]
fn light_bins_point() {
    let ctx = Context::headless(256, 256).block();
    let stage = ctx
        .new_stage()
        .with_msaa_sample_count(1)
        .with_bloom_mix_strength(0.08);
    let mut doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("pedestal.glb"),
        )
        .unwrap();

    doc.materials
        .get_mut(0)
        .unwrap()
        .set_albedo_factor(Vec4::ONE)
        .set_roughness_factor(1.0)
        .set_metallic_factor(0.0);

    let camera = doc.cameras.first().unwrap();
    let view = Mat4::look_at_rh(Vec3::new(-7.0, 5.0, 7.0), Vec3::ZERO, Vec3::Y);
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_6, 1.0, 0.1, 15.0);
    camera.camera.set_projection_and_view(proj, view);

    let _point_light = stage
        .new_point_light()
        .with_position(Vec3::new(1.1, 1.0, 1.1))
        .with_color(Vec4::ONE)
        .with_intensity(5.0);
    snapshot(
        &ctx,
        &stage,
        "light/tiling/light_bins_point/1-scene.png",
        true,
    );

    let tiling = stage.new_light_tiling(LightTilingConfig {
        max_lights_per_tile: 2,
        ..Default::default()
    });
    tiling.run(&stage);

    let (_mins, _maxs, lights) = futures_lite::future::block_on(tiling.read_images(stage.as_ref()));
    img_diff::save("light/tiling/light_bins_point/2-lights.png", lights);

    let mut found = 0;
    for tile in futures_lite::future::block_on(tiling.read_tiles(stage.as_ref())) {
        if tile.depth_min != tile.depth_max && found < 3 {
            found += 1;
            log::info!("tile: {tile:#?}");
        }
    }
}

fn tiling_e2e_sanity_with(
    tile_size: u32,
    max_lights_per_tile: u32,
    i: u32,
    minimum_illuminance: f32,
    save_images: bool,
) {
    println!(
        "tiling e2e with tile_size:{tile_size}, \
        max_lights_per_tile:{max_lights_per_tile}, \
        minimum_illuminance: {minimum_illuminance}"
    );
    let size = size();
    let ctx = Context::headless(size.x, size.y).block();
    let stage = ctx
        .new_stage()
        .with_bloom(true)
        .with_bloom_mix_strength(0.5)
        .with_msaa_sample_count(1);

    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();

    let camera = make_camera(&stage);
    stage.use_camera(camera);

    let _ = stage.lighting.commit();

    let moonlight = doc.lights.first().unwrap();
    let _shadow = {
        let sm = stage
            .new_shadow_map(moonlight, UVec2::splat(1024), 0.1, 256.0)
            .unwrap();
        sm.shadowmap_descriptor.modify(|d| {
            d.bias_min = 0.0;
            d.bias_max = 0.0;
            d.pcf_samples = 2;
        });
        sm.update(&stage, doc.renderlets_iter()).unwrap();
        sm
    };

    let mut bounding_boxes = vec![];
    for node in doc.nodes.iter() {
        if node.mesh.is_none() {
            continue;
        }
        let transform = Mat4::from(node.transform.global_descriptor());
        if let Some(mesh_index) = node.mesh {
            let mesh = &doc.meshes[mesh_index];
            for prim in mesh.primitives.iter() {
                let (min, max) = prim.bounding_box;
                let min = transform.transform_point3(min);
                let max = transform.transform_point3(max);
                let bb = BoundingBox::from_min_max(min, max);
                if bb.half_extent.min_element().is_zero() {
                    continue;
                }
                bounding_boxes.push(bb);
            }
        }
    }

    let mut prng = crate::math::GpuRng::new(666);
    let mut lights: Vec<GeneratedLight> = vec![];

    for _ in 0..MAX_LIGHTS {
        lights.push(gen_light(&stage, &mut prng, &bounding_boxes));
    }
    println!("created lights");

    // Remove the light meshes
    for generated_light in lights.iter() {
        stage.remove_primitive(&generated_light.mesh_renderlet);
    }
    snapshot(
        &ctx,
        &stage,
        "light/tiling/e2e/4-scene-no-tiling.png",
        false,
    );

    let config = LightTilingConfig {
        tile_size,
        max_lights_per_tile,
        minimum_illuminance,
    };
    println!("generating rendering with config: {config:#?}");
    let tiling = stage.new_light_tiling(config);
    tiling.run(&stage);
    snapshot(
        &ctx,
        &stage,
        &format!("light/tiling/e2e/6-scene-{tile_size}-{max_lights_per_tile}-lights-{i}-{minimum_illuminance}-min-lux.png"),
        save_images
    );

    #[cfg(feature = "light-tiling-stats")]
    {
        use stats::*;
        // Stats
        let mut stats = LightTilingStats::default();
        for number_of_lights_exponent in 2..=MAX_LIGHTS.ilog2() {
            let number_of_lights = 2usize.pow(number_of_lights_exponent);
            let mut run = LightTilingStatsRun {
                number_of_lights,
                iterations: vec![],
            };

            for (i, generated_light) in lights.iter().enumerate() {
                stage.remove_light(&generated_light._light);
                if i < number_of_lights {
                    stage.add_light(&generated_light._light);
                }
            }

            const NUM_RUNS: usize = 2;
            for (i, with_tiling) in (0..NUM_RUNS).zip([true, false].iter().cycle()) {
                log::info!(
                    "{number_of_lights} {i} {} running",
                    if true { "tiling" } else { "non-tiling" }
                );
                if *with_tiling {
                    tiling.run(&stage);
                }
                stage.lighting.lighting_descriptor.modify(|desc| {
                    desc.light_tiling_descriptor_id = if *with_tiling {
                        tiling.tiling_descriptor.id()
                    } else {
                        Id::NONE
                    }
                });
                let start = std::time::Instant::now();
                let frame = ctx.get_next_frame().unwrap();
                stage.render(&frame.view());
                frame.present();
                ctx.get_device().poll(wgpu::PollType::Wait).unwrap();
                let duration = start.elapsed();
                run.iterations.push((*with_tiling, duration));
            }
            stats.runs.push(run);
        }
        plot(stats, &format!("frame-time-{tile_size}-{max_lights_per_tile}-lights-{i}-{minimum_illuminance}-min-lux"));
    }
}

#[test]
/// Test the light tiling feature, end to end.
fn tiling_e2e_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let config = LightTilingConfig::default();
    tiling_e2e_sanity_with(
        config.tile_size,
        config.max_lights_per_tile,
        0,
        config.minimum_illuminance,
        false,
    );

    #[cfg(feature = "light-tiling-stats")]
    {
        let tile_sizes = [4, 8, 16];
        let max_lights_per_tile = [16, 32, 64, 128, 256];
        let minimum_illuminance_lux = [0.05, 0.1, 0.3, 1.0, 2.0];
        for tile_size in tile_sizes {
            for max_lights_per_tile in max_lights_per_tile {
                for (i, minimum_illuminance) in minimum_illuminance_lux.iter().enumerate() {
                    tiling_e2e_sanity_with(
                        tile_size,
                        max_lights_per_tile,
                        i as u32,
                        *minimum_illuminance,
                        true,
                    )
                }
            }
        }
    }
}

fn snapshot(ctx: &crate::context::Context, stage: &Stage, path: &str, save: bool) {
    let frame = ctx.get_next_frame().unwrap();
    let start = std::time::Instant::now();
    stage.render(&frame.view());
    let elapsed = start.elapsed();
    log::info!("shapshot: {}s '{path}'", elapsed.as_secs_f32());
    let img = frame.read_image().block().unwrap();
    if save {
        img_diff::save(path, img);
    } else {
        img_diff::assert_img_eq(path, img);
    }
    frame.present();
}

const MAX_LIGHTS: usize = 2usize.pow(10);

#[cfg(feature = "light-tiling-stats")]
mod stats {
    #![allow(dead_code)]
    use core::time::Duration;

    use plotters::{
        chart::{ChartBuilder, SeriesLabelPosition},
        prelude::{BitMapBackend, Circle, EmptyElement, IntoDrawingArea, PathElement},
        series::{LineSeries, PointSeries},
        style::{Color, IntoFont, ShapeStyle},
    };

    pub struct LightTilingStatsRun {
        pub number_of_lights: usize,
        pub iterations: Vec<(bool, Duration)>,
    }

    impl LightTilingStatsRun {
        fn avg_frame_time(&self, with_tiling: bool) -> f32 {
            let total: Duration = self
                .iterations
                .iter()
                .filter_map(|(had_tiling, dur)| {
                    if *had_tiling == with_tiling {
                        Some(dur)
                    } else {
                        None
                    }
                })
                .sum();
            total.as_secs_f32() / self.iterations.len() as f32
        }
    }

    #[derive(Default)]
    pub struct LightTilingStats {
        pub runs: Vec<LightTilingStatsRun>,
    }

    pub fn plot(stats: LightTilingStats, filename: &str) {
        let path = crate::test::workspace_dir()
            .join(format!("test_output/light/tiling/e2e/{filename}.png"));
        let root_drawing_area = BitMapBackend::new(&path, (800, 600)).into_drawing_area();
        root_drawing_area.fill(&plotters::style::WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root_drawing_area)
            .caption(
                "Renderling lighting frame time",
                ("sans-serif", 50).into_font(),
            )
            .margin(30)
            .margin_right(100)
            .margin_left(60)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                0..super::MAX_LIGHTS + 1,
                0.0..stats
                    .runs
                    .iter()
                    .flat_map(|r| [r.avg_frame_time(true), r.avg_frame_time(false)])
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or_default(),
            )
            .unwrap();
        fn y_fmt(coord: &f32) -> String {
            let fps = 1.0 / coord;
            format!("{coord}s ({fps:.2} fps)")
        }
        chart
            .configure_mesh()
            .x_desc("number of lights")
            .y_label_formatter(&y_fmt)
            .draw()
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                stats
                    .runs
                    .iter()
                    .map(|r| (r.number_of_lights, r.avg_frame_time(false))),
                plotters::style::RED,
            ))
            .unwrap()
            .label("without tiling")
            .legend(|(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], plotters::style::RED.filled())
            });
        chart
            .draw_series(LineSeries::new(
                stats
                    .runs
                    .iter()
                    .map(|r| (r.number_of_lights, r.avg_frame_time(true))),
                plotters::style::BLUE,
            ))
            .unwrap()
            .label("with tiling")
            .legend(|(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], plotters::style::BLUE.filled())
            });
        chart
            .draw_series(PointSeries::of_element(
                stats
                    .runs
                    .iter()
                    .map(|r| (r.number_of_lights, r.avg_frame_time(false))),
                5,
                ShapeStyle::from(&plotters::style::RED).filled(),
                &|(num_lights, seconds_per_frame), size, style| {
                    EmptyElement::at((num_lights, seconds_per_frame))
                        + Circle::new((0, 0), size, style)
                },
            ))
            .unwrap();
        chart
            .draw_series(PointSeries::of_element(
                stats
                    .runs
                    .iter()
                    .map(|r| (r.number_of_lights, r.avg_frame_time(true))),
                5,
                ShapeStyle::from(&plotters::style::BLUE).filled(),
                &|(num_lights, seconds_per_frame), size, style| {
                    EmptyElement::at((num_lights, seconds_per_frame))
                        + Circle::new((0, 0), size, style)
                },
            ))
            .unwrap();

        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperLeft)
            .margin(20)
            .label_font(("sans-serif", 20))
            .draw()
            .unwrap();
        root_drawing_area.present().unwrap();
    }
}

#[test]
/// For all light types that have a position:
///
/// Ensures that a light with a translated position renders the same
/// as a light at the origin that has a linked `NestedTransform` applied with
/// that same translation.
///
/// In other words, light w/ nested transform is the same as light with
/// that same transform pre-applied.
fn pedestal() {
    let ctx = crate::context::Context::headless(256, 256).block();
    let stage = ctx
        .new_stage()
        .with_lighting(false)
        .with_msaa_sample_count(4)
        .with_bloom_mix_strength(0.08);
    let mut doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("pedestal.glb"),
        )
        .unwrap();

    doc.materials
        .get_mut(0)
        .unwrap()
        .set_albedo_factor(Vec4::ONE)
        .set_roughness_factor(1.0)
        .set_metallic_factor(0.0);

    let camera = doc.cameras.first().unwrap();
    camera.camera.set_projection_and_view(
        Mat4::perspective_rh(std::f32::consts::FRAC_PI_6, 1.0, 0.1, 15.0),
        Mat4::look_at_rh(Vec3::new(-7.0, 5.0, 7.0), Vec3::ZERO, Vec3::Y),
    );

    let color = {
        // let mut c = hex_to_vec4(0xEEDF7AFF);
        // linear_xfer_vec4(&mut c);
        // c
        Vec4::ONE
    };
    let position = Vec3::new(1.1, 1.0, 1.1);
    stage.set_has_lighting(true);

    let mut dir_infos = vec![];
    {
        log::info!("adding dir light");
        let dir_light = stage
            .new_directional_light()
            .with_direction(-position)
            .with_color(color)
            .with_intensity(5.0);
        snapshot(&ctx, &stage, "light/pedestal/directional.png", false);

        let geometry_slab =
            futures_lite::future::block_on(stage.geometry.slab_allocator().read(..)).unwrap();

        let renderlet = doc.renderlets_iter().next().unwrap();
        log::info!("renderlet: {:#?}", renderlet.descriptor());

        for vertex_index in 0..renderlet.descriptor().vertices_array.len() {
            let mut info = PrimitivePbrVertexInfo::default();
            crate::primitive::shader::primitive_vertex(
                renderlet.id(),
                vertex_index as u32,
                &geometry_slab,
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut info,
            );

            dir_infos.push(info);
        }
        stage.remove_light(&dir_light);
    }
    assert_eq!(0, stage.lighting.lights().len());

    // Point lights
    {
        log::info!("adding point light with pre-applied position");
        let point_light = stage
            .new_point_light()
            .with_position(position)
            .with_color(color)
            .with_intensity(5.0);
        snapshot(&ctx, &stage, "light/pedestal/point.png", false);
        stage.remove_light(&point_light);
    }

    {
        log::info!("adding point light with nested transform");
        let transform = stage.new_nested_transform();
        transform.set_local_translation(position);

        let point_light = stage
            .new_point_light()
            .with_position(Vec3::ZERO)
            .with_color(color)
            .with_intensity(5.0);
        point_light.link_node_transform(&transform);

        snapshot(&ctx, &stage, "light/pedestal/point.png", false);
        stage.remove_light(&point_light);
    }

    {
        log::info!("adding spot light with pre-applied position");
        let spot = stage
            .new_spot_light()
            .with_position(position)
            .with_direction(-position)
            .with_color(color)
            .with_intensity(5.0)
            .with_inner_cutoff(core::f32::consts::PI / 5.0)
            .with_outer_cutoff(core::f32::consts::PI / 4.0);
        snapshot(&ctx, &stage, "light/pedestal/spot.png", false);

        let geometry_slab =
            futures_lite::future::block_on(stage.geometry.slab_allocator().read(..)).unwrap();

        let renderlet = doc.renderlets_iter().next().unwrap();
        log::info!("renderlet: {:#?}", renderlet.descriptor());
        let mut spot_infos = vec![];

        for vertex_index in 0..renderlet.descriptor().vertices_array.len() {
            let mut info = PrimitivePbrVertexInfo::default();
            crate::primitive::shader::primitive_vertex(
                renderlet.id(),
                vertex_index as u32,
                &geometry_slab,
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut Default::default(),
                &mut info,
            );
            spot_infos.push(info);
        }

        // assert that the output of the vertex shader is the same for the first renderlet,
        // regardless of the lighting
        pretty_assertions::assert_eq!(dir_infos, spot_infos);
        stage.remove_light(&spot);
    }

    {
        log::info!("adding spot light with node position");

        let node_transform = stage.new_nested_transform();
        node_transform.set_local_translation(position);

        let spot = stage
            .new_spot_light()
            .with_position(Vec3::ZERO)
            .with_direction(-position)
            .with_color(color)
            .with_intensity(5.0)
            .with_inner_cutoff(core::f32::consts::PI / 5.0)
            .with_outer_cutoff(core::f32::consts::PI / 4.0);
        spot.link_node_transform(&node_transform);
        snapshot(&ctx, &stage, "light/pedestal/spot.png", false);
    }
}

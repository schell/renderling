//! Tests of the lighting system.

use core::time::Duration;
use std::time::Instant;

use craballoc::runtime::CpuRuntime;
use crabslab::{Array, CpuSlab, Slab};
use glam::{UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use plotters::{
    chart::{ChartBuilder, SeriesLabelPosition},
    prelude::{
        BitMapBackend, Circle, EmptyElement, IntoDrawingArea, IntoSegmentedCoord, PathElement, Text,
    },
    series::{Histogram, LineSeries, PointSeries},
    style::{Color, IntoFont, ShapeStyle},
};
use spirv_std::num_traits::Zero;

use crate::{
    bvol::BoundingBox,
    camera::Camera,
    color::linear_xfer_vec4,
    draw::DrawIndirectArgs,
    geometry::GeometryDescriptor,
    light::{
        LightTile, LightTiling, LightTilingDescriptor, LightTilingInvocation, SpotLightCalculation,
    },
    math::{hex_to_vec4, scaled_f32_to_u8, ConstTexture, CpuTexture2d, GpuRng, NonAtomicSlab},
    pbr::Material,
    prelude::Transform,
    stage::{Renderlet, Stage, Vertex},
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
    let parent_transform = Transform::from(light_node.transform());
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
    let ctx = crate::Context::headless(w as u32, h as u32);
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
        .modify(|cam| cam.set_projection(crate::camera::perspective(w, h)));
    stage.use_camera(camera);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().unwrap();
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
    let ctx = crate::Context::headless(w as u32, h as u32);
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
        .modify(|cam| cam.set_projection(crate::camera::perspective(w, h)));
    stage.use_camera(camera);

    let down_light = doc.lights.first().unwrap();
    log::info!(
        "down_light: {:#?}",
        down_light.light_details.as_spot().unwrap().get()
    );

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().unwrap();
    img_diff::assert_img_eq("light/spot_lights/frame.png", img);
    frame.present();
}

#[test]
fn light_tiling_light_bounds() {
    let magnification = 8;
    let w = 16.0 * 2.0f32.powi(magnification);
    let h = 9.0 * 2.0f32.powi(magnification);
    let ctx = crate::Context::headless(w as u32, h as u32);
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
    doc.renderlets_iter().for_each(|hy_rend| {
        hy_rend.modify(|r| {
            r.visible = false;
        });
    });

    let colors = [0x6DE1D2FF, 0xFFD63AFF, 0x6DE1D2FF, 0xF75A5AFF].map(|albedo_factor| {
        stage.new_material(Material {
            albedo_factor: {
                let mut color = crate::math::hex_to_vec4(albedo_factor);
                linear_xfer_vec4(&mut color);
                color
            },
            ..Default::default()
        })
    });
    let mut resources = vec![];
    for (i, node) in doc.nodes.iter().enumerate() {
        if node.mesh.is_none() {
            continue;
        }
        let transform = Mat4::from(node.transform.get_global_transform());
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
                        .builder()
                        .with_vertices({
                            bb.get_mesh()
                                .map(|(p, n)| Vertex::default().with_position(p).with_normal(n))
                        })
                        .with_material_id(colors[i % colors.len()].id())
                        .build(),
                );
            }
        }
    }

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().unwrap();
    img_diff::save("light/tiling/bounds.png", img);
    frame.present();
}

fn gen_vec3(prng: &mut GpuRng) -> Vec3 {
    let x = prng.gen_f32(-120.0, 120.0);
    let y = prng.gen_f32(0.0, 80.0);
    let z = prng.gen_f32(-120.0, 120.0);
    Vec3::new(x, y, z)
}

fn gen_light(
    stage: &Stage,
    prng: &mut GpuRng,
    bounding_boxes: &[BoundingBox],
) -> (
    Hybrid<Transform>,
    HybridArray<Vertex>,
    Hybrid<Material>,
    AnalyticalLightBundle,
    Hybrid<Renderlet>,
) {
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

    // Also make a renderlet for the light, so we can see where it is.
    let transform = stage.new_nested_transform();
    transform.modify(|t| {
        t.translation = position;
    });
    let rez = stage
        .builder()
        .with_transform(Transform {
            translation: position,
            ..Default::default()
        })
        .with_vertices(
            light_bb
                .get_mesh()
                .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
        )
        .with_material(Material {
            albedo_factor: color,
            has_lighting: false,
            emissive_factor: color.xyz(),
            emissive_strength_multiplier: 100.0,
            ..Default::default()
        })
        .suffix({
            // suffix the actual analytical light
            let intensity = scale * 100.0;

            let light_descriptor = PointLightDescriptor {
                position,
                color,
                intensity,
            };

            stage.new_analytical_light(light_descriptor, None)
        })
        .build();
    rez
}

fn size() -> UVec2 {
    UVec2::new(
        (10.0 * 2.0f32.powi(8)) as u32,
        (9.0 * 2.0f32.powi(8)) as u32,
    )
}

fn make_camera() -> Camera {
    let size = size();
    Camera::new(
        Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_4,
            size.x as f32 / size.y as f32,
            50.0,
            600.0,
        ),
        Mat4::look_at_rh(Vec3::new(250.0, 200.0, 250.0), Vec3::ZERO, Vec3::Y),
    )
}

#[test]
/// Test the light tiling feature.
fn light_tiling_cpu_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let size = size();
    let ctx = crate::Context::headless(size.x, size.y);
    let stage = ctx
        .new_stage()
        .with_lighting(true)
        .with_bloom(true)
        .with_bloom_mix_strength(0.5);

    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();

    let camera = stage.new_camera(make_camera());
    stage.use_camera(camera);

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
        let transform = Mat4::from(node.transform.get_global_transform());
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
    let mut lights = vec![];

    for _ in 0..MAX_LIGHTS {
        lights.push(gen_light(&stage, &mut prng, &bounding_boxes));
    }

    // Remove the light meshes
    for (_, _, _, _, renderlet) in lights.iter() {
        stage.remove_renderlet(renderlet);
    }
    snapshot(
        &ctx,
        &stage,
        "light/tiling/cpu/4-after-lights-no-meshes.png",
    );

    // Get all the slabs and run the shader on the CPU
    let tiling = LightTiling::new(ctx.runtime(), false, size, 32);
    let desc = tiling.tiling_descriptor.get();
    let depth = stage.depth_texture.read().unwrap();
    let depth_img = crate::texture::read_depth_texture_to_image(
        ctx.runtime(),
        size.x as usize,
        size.y as usize,
        &depth.texture,
    )
    .unwrap();
    let geometry_slab =
        futures_lite::future::block_on(stage.geometry.slab_allocator().read(..)).unwrap();
    let lighting_slab =
        futures_lite::future::block_on(stage.lighting.slab_allocator().read(..)).unwrap();
    let tiling_slab = {
        tiling.prepare(UVec2::new(depth_img.width(), depth_img.height()));
        let _ = tiling.tiling_slab.commit();
        futures_lite::future::block_on(tiling.tiling_slab.read(..)).unwrap()
    };

    let w = depth_img.width() / LightTilingDescriptor::TILE_SIZE.x + 1;
    let h = depth_img.height() / LightTilingDescriptor::TILE_SIZE.y + 1;
    let mut non_atomic_tiling_slab = NonAtomicSlab::new(tiling_slab);
    let depth_img = CpuTexture2d::from_image(depth_img, crate::math::luma_u8_to_vec4);
    for x in 0..w {
        for y in 0..h {
            let global_id = UVec3::new(x, y, 0);
            let invocation = LightTilingInvocation::new(global_id, desc);
            invocation.compute_tiles(
                &depth_img,
                &geometry_slab,
                &lighting_slab,
                &mut non_atomic_tiling_slab,
            );
        }
    }
}

#[test]
/// Test the light tiling feature.
fn light_tiling_sanity() {
    let _ = env_logger::builder().is_test(true).try_init();
    let size = size();
    let ctx = crate::Context::headless(size.x, size.y);
    let stage = ctx
        .new_stage()
        .with_lighting(false)
        .with_bloom(true)
        .with_bloom_mix_strength(0.5);

    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("light_tiling_test.glb"),
        )
        .unwrap();

    let camera = stage.new_camera(make_camera());
    stage.use_camera(camera);
    snapshot(&ctx, &stage, "light/tiling/1-no-lighting.png");

    stage.set_has_lighting(true);

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
    snapshot(&ctx, &stage, "light/tiling/2-before-lights.png");

    crate::test::capture_gpu_frame(&ctx, "light/tiling/2.gputrace", || {
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        frame.present();
    });

    let mut bounding_boxes = vec![];
    for node in doc.nodes.iter() {
        if node.mesh.is_none() {
            continue;
        }
        let transform = Mat4::from(node.transform.get_global_transform());
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
    log::info!("have {} bounding boxes", bounding_boxes.len());

    let mut prng = crate::math::GpuRng::new(666);
    let mut lights: Vec<(
        Hybrid<Transform>,
        HybridArray<Vertex>,
        Hybrid<Material>,
        AnalyticalLightBundle,
        Hybrid<Renderlet>,
    )> = vec![];

    for _ in 0..MAX_LIGHTS {
        lights.push(gen_light(&stage, &mut prng, &bounding_boxes));
    }
    snapshot(&ctx, &stage, "light/tiling/3-after-lights.png");

    // Remove the light meshes
    for (_, _, _, _, renderlet) in lights.iter() {
        stage.remove_renderlet(renderlet);
    }
    snapshot(&ctx, &stage, "light/tiling/4-after-lights-no-meshes.png");

    let tiling = LightTiling::new(ctx.runtime(), false, size, 32);
    let desc = tiling.tiling_descriptor.get();
    let depth = stage.depth_texture.read().unwrap();
    let mut depth_img = crate::texture::read_depth_texture_f32(
        ctx.runtime(),
        size.x as usize,
        size.y as usize,
        depth.texture.as_ref(),
    )
    .unwrap();
    // let mut depth_img = crate::texture::read_depth_texture_to_image(
    //     ctx.runtime(),
    //     size.x as usize,
    //     size.y as usize,
    //     &depth.texture,
    // )
    // .unwrap();
    // img_diff::normalize_gray_img(&mut depth_img);
    img_diff::save("light/tiling/5-depth.png", depth_img);
    tiling.run(&stage.geometry.commit(), &stage.lighting.commit(), &depth);
    let (mut mins_img, mut maxs_img, mut lights_img) =
        futures_lite::future::block_on(tiling.read_images());
    img_diff::normalize_gray_img(&mut mins_img);
    img_diff::normalize_gray_img(&mut maxs_img);
    img_diff::normalize_gray_img(&mut lights_img);
    img_diff::save("light/tiling/5-mins.png", mins_img);
    img_diff::save("light/tiling/5-maxs.png", maxs_img);
    img_diff::save("light/tiling/5-lights.png", lights_img);

    return;
    log::info!("running stats");

    // Stats
    let mut stats = LightTilingStats::default();
    for number_of_lights in [
        1,
        MAX_LIGHTS / 8,
        MAX_LIGHTS / 4,
        MAX_LIGHTS / 2,
        ((MAX_LIGHTS / 2) + MAX_LIGHTS) / 2,
        MAX_LIGHTS,
    ] {
        let mut run = LightTilingStatsRun {
            number_of_lights,
            iterations: vec![],
        };

        for (i, (_, _, _, light, _)) in lights.iter().enumerate() {
            stage.remove_light(light);
            if i < number_of_lights {
                stage.add_light(light);
            }
        }

        const NUM_RUNS: usize = 2;
        for i in 0..NUM_RUNS {
            log::info!("{number_of_lights} {i} running");
            let start = Instant::now();
            let frame = ctx.get_next_frame().unwrap();
            stage.render(&frame.view());
            frame.present();
            ctx.get_device().poll(wgpu::Maintain::wait());
            let duration = start.elapsed();
            run.iterations.push(duration);
        }
        stats.runs.push(run);
    }
    plot(stats);
}

fn snapshot(ctx: &crate::Context, stage: &Stage, path: &str) {
    let frame = ctx.get_next_frame().unwrap();
    let start = std::time::Instant::now();
    stage.render(&frame.view());
    let elapsed = start.elapsed();
    log::info!("shapshot: {}s '{path}'", elapsed.as_secs_f32());
    let img = frame.read_image().unwrap();
    img_diff::save(path, img);
    frame.present();
}

const MAX_LIGHTS: usize = 1024;

struct LightTilingStatsRun {
    number_of_lights: usize,
    iterations: Vec<Duration>,
}

impl LightTilingStatsRun {
    fn avg_frame_time(&self) -> f32 {
        let total: Duration = self.iterations.iter().sum();
        total.as_secs_f32() / self.iterations.len() as f32
    }
}

#[derive(Default)]
struct LightTilingStats {
    runs: Vec<LightTilingStatsRun>,
}

fn plot(stats: LightTilingStats) {
    let path = crate::test::workspace_dir().join("test_output/lights/tiling/frame-time.png");
    let root_drawing_area = BitMapBackend::new(&path, (800, 600)).into_drawing_area();
    root_drawing_area.fill(&plotters::style::WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption(
            "Renderling lighting frame time",
            ("sans-serif", 50).into_font(),
        )
        .margin(30)
        .margin_right(100)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..MAX_LIGHTS + 1,
            0.0..stats
                .runs
                .iter()
                .map(|r| r.avg_frame_time())
                .max_by(|a, b| a.total_cmp(b))
                .unwrap_or_default(),
        )
        .unwrap();
    chart
        .configure_mesh()
        .x_desc("number of lights")
        .y_desc("avg fps")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            stats
                .runs
                .iter()
                .map(|r| (r.number_of_lights, r.avg_frame_time())),
            plotters::style::RED,
        ))
        .unwrap()
        .label("without-tiling")
        .legend(|(x, y)| {
            PathElement::new(vec![(x, y), (x + 20, y)], plotters::style::RED.filled())
        });
    chart
        .draw_series(PointSeries::of_element(
            stats
                .runs
                .iter()
                .map(|r| (r.number_of_lights, r.avg_frame_time())),
            5,
            ShapeStyle::from(&plotters::style::RED).filled(),
            &|(num_lights, seconds_per_frame), size, style| {
                EmptyElement::at((num_lights, seconds_per_frame))
                    + Circle::new((0, 0), size, style)
                    + Text::new(
                        format!("({num_lights}, {:.2} fps)", 1.0 / seconds_per_frame),
                        (0, 15),
                        ("sans-serif", 15),
                    )
            },
        ))
        .unwrap();

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .margin(20)
        .label_font(("sans-serif", 20))
        .draw()
        .unwrap();
    root_drawing_area.present().unwrap();
}

#[test]
/// Ensures that a light with a translated position renders the same
/// as a light at the origin that has a transform applied with
/// that same translation.
///
/// In other words, light in nested transform is the same as light with
/// that same transform pre-applied.
fn pedestal() {
    let ctx = crate::Context::headless(256, 256);
    let stage = ctx
        .new_stage()
        .with_lighting(false)
        .with_msaa_sample_count(4)
        .with_bloom_mix_strength(0.08);
    let doc = stage
        .load_gltf_document_from_path(
            crate::test::workspace_dir()
                .join("gltf")
                .join("pedestal.glb"),
        )
        .unwrap();
    let camera = doc.cameras.first().unwrap();
    camera.camera.modify(|cam| {
        cam.set_projection(Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_6,
            1.0,
            0.1,
            15.0,
        ));
    });

    let color = {
        let mut c = hex_to_vec4(0xEEDF7AFF);
        linear_xfer_vec4(&mut c);
        c
    };
    let position = Vec3::new(0.0, 1.0, 0.0);
    let transform = stage.new_nested_transform();
    transform.modify(|t| t.translation = position);

    stage.set_has_lighting(true);

    {
        log::info!("adding dir light");
        let _dir_light = stage.new_analytical_light(
            DirectionalLightDescriptor {
                direction: Vec3::new(-0.5, -1.0, 0.0),
                color,
                intensity: 5.0,
            },
            None,
        );
        snapshot(&ctx, &stage, "light/pedestal/directional.png");
        log::info!("dropping dir light");
    }
    assert_eq!(0, stage.lighting.lights().count());

    let bb = BoundingBox {
        center: Vec3::ZERO,
        half_extent: Vec3::splat(0.25),
    };
    let _light_mesh_rez = stage
        .builder()
        .with_transform_id(transform.global_transform_id())
        .with_vertices(
            bb.get_mesh()
                .map(|(p, n)| Vertex::default().with_position(p).with_normal(n)),
        )
        .with_material(Material {
            albedo_factor: color,
            emissive_factor: color.xyz(),
            emissive_strength_multiplier: 4.0,
            ..Default::default()
        })
        .build();

    let light_descriptor = PointLightDescriptor {
        position: Vec3::ZERO,
        color,
        intensity: 10.0,
    };
    let _light = stage.new_analytical_light(light_descriptor, Some(transform));
    snapshot(&ctx, &stage, "light/pedestal/point.png");

    // light.transform.modify(|t| t.translation = position);

    // snapshot(&ctx, &stage, "light/point/transform-nest.png");

    // let light_slab =
    //     futures_lite::future::block_on(stage.lighting.light_slab.read(..)).unwrap();
    // let frag = crate::pbr::shade_fragment(
    //     &ConstTexture::new(Vec4::ZERO),
    //     &(),
    //     camera.node_transform.get_global_transform().translation,
    //     Vec3::Y,
    //     Vec3::ZERO,
    //     Vec3::splat(0.5),
    //     0.0,
    //     1.0,
    //     0.0,
    //     Vec3::ZERO,
    //     Vec3::ONE,
    //     Vec3::ONE,
    //     Vec2::ONE,
    //     &light_slab,
    // );
    // println!("frag: {frag}");
}

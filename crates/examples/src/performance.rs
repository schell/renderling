//! Performance examples: culling, light tiling and MSAA.

use crate::cwd_to_manual_assets_dir;
use renderling::{geometry::Vertex, glam::Vec3};

/// A unit cube of vertices, centered on the origin.
fn unit_cube() -> Vec<Vertex> {
    let points: [Vec3; 8] = renderling::math::UNIT_POINTS;
    renderling::math::UNIT_INDICES
        .iter()
        .map(|i| Vertex::default().with_position(points[*i]))
        .collect()
}

/// A horizontal quad at the given height, with the given half extent.
fn floor(y: f32, s: f32) -> Vec<Vertex> {
    [
        Vertex::default()
            .with_position([s, y, s])
            .with_normal(Vec3::Y),
        Vertex::default()
            .with_position([s, y, -s])
            .with_normal(Vec3::Y),
        Vertex::default()
            .with_position([-s, y, -s])
            .with_normal(Vec3::Y),
        Vertex::default()
            .with_position([s, y, s])
            .with_normal(Vec3::Y),
        Vertex::default()
            .with_position([-s, y, -s])
            .with_normal(Vec3::Y),
        Vertex::default()
            .with_position([-s, y, s])
            .with_normal(Vec3::Y),
    ]
    .into()
}

#[tokio::test]
async fn manual_culling() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("performance").unwrap();

    // ANCHOR: culling
    use renderling::{
        camera,
        context::Context,
        glam::{Vec3, Vec4},
        stage::Stage,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_lighting(false)
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let projection = camera::perspective(512.0, 512.0);
    let view = camera::look_at(Vec3::new(0.0, 2.5, 5.5), Vec3::ZERO, Vec3::Y);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);

    // A grid of cubes, some of which are outside the camera's view.
    let geometry = stage.new_vertices(unit_cube());
    for x in 0..7 {
        for z in 0..7 {
            stage
                .new_primitive()
                .with_vertices(&geometry)
                .with_transform(
                    stage
                        .new_transform()
                        .with_translation(Vec3::new(
                            x as f32 * 1.2 - 3.6,
                            0.0,
                            z as f32 * 1.2 - 3.6,
                        ))
                        .with_scale(Vec3::splat(0.25)),
                )
                .with_material(
                    stage
                        .new_material()
                        .with_albedo_factor(Vec4::new(0.4, 0.7, 0.9, 1.0))
                        .with_has_lighting(false),
                );
        }
    }

    // Frustum culling is on by default. Objects outside the camera's
    // view are skipped by the GPU.
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_culled = frame.read_image().await.unwrap();
    img_culled.save("performance/culling.png").unwrap();
    frame.present();

    // Turning frustum culling off must not change the picture - the
    // same objects are visible either way, culling only skips work.
    stage.set_use_frustum_culling(false);
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_unculled = frame.read_image().await.unwrap();
    frame.present();
    // ANCHOR_END: culling

    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let changed = pixel_diff(img_culled.as_raw(), img_unculled.as_raw());
    println!("pixels changed by disabling frustum culling: {changed}");
    assert!(
        changed == 0,
        "frustum culling changed the rendered output ({changed} pixels changed)"
    );

    // ANCHOR: occlusion
    // Occlusion culling also skips objects hidden behind other objects.
    // It is off by default and still a feature in development.
    stage.set_use_frustum_culling(true);
    stage.set_use_occlusion_culling(true);
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    frame
        .read_image()
        .await
        .unwrap()
        .save("performance/occlusion.png")
        .unwrap();
    frame.present();
    // ANCHOR_END: occlusion

    // ANCHOR: msaa
    // Multisample anti-aliasing smooths the edges of geometry. It is set
    // with a sample count - here we go from 1 (no MSAA) to 4.
    stage.set_msaa_sample_count(4);
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_msaa = frame.read_image().await.unwrap();
    img_msaa.save("performance/msaa-4.png").unwrap();
    frame.present();
    // ANCHOR_END: msaa

    let changed = pixel_diff(img_culled.as_raw(), img_msaa.as_raw());
    println!("pixels changed by msaa: {changed}");
    assert!(
        changed > 500,
        "enabling msaa did not change the rendering ({changed} pixels changed)"
    );
}

#[tokio::test]
async fn manual_light_tiling() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("performance").unwrap();

    // ANCHOR: many_lights
    use renderling::{
        camera,
        color::css_srgb_color_to_linear,
        context::Context,
        glam::{Vec3, Vec4},
        light::{AnalyticalLight, Candela, PointLight},
        stage::Stage,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_bloom(false)
        .with_background_color(Vec4::new(0.02, 0.02, 0.02, 1.0));

    let projection = camera::perspective(512.0, 512.0);
    let view = camera::look_at(Vec3::new(4.0, 4.0, 8.0), Vec3::ZERO, Vec3::Y);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);

    // A ground plane to receive the light.
    stage
        .new_primitive()
        .with_vertices(stage.new_vertices(floor(0.0, 3.0)))
        .with_material(
            stage
                .new_material()
                .with_albedo_factor(Vec4::new(0.85, 0.85, 0.85, 1.0)),
        );

    // A grid of colorful point lights. Without light tiling the shader
    // iterates every light for every fragment.
    let mut lights: Vec<AnalyticalLight<PointLight>> = vec![];
    for i in 0..6 {
        for j in 0..6 {
            let light = stage
                .new_point_light()
                .with_position(Vec3::new(i as f32 * 1.2 - 3.0, 2.5, j as f32 * 1.2 - 3.0))
                .with_color(css_srgb_color_to_linear(
                    (60 + i * 35) as u8,
                    (60 + j * 35) as u8,
                    128,
                ))
                .with_intensity(Candela(20.0));
            lights.push(light);
        }
    }
    println!("lights: {}", lights.len());

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_untiled = frame.read_image().await.unwrap();
    img_untiled.save("performance/no-tiling.png").unwrap();
    frame.present();
    // ANCHOR_END: many_lights

    // ANCHOR: tiling
    use renderling::light::LightTilingConfig;

    // Light tiling bins lights into screen tiles so each fragment only
    // considers the lights near it. Run it before rendering - in an
    // application, every frame.
    let tiling = stage.new_light_tiling(LightTilingConfig::default());
    tiling.run(&stage);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_tiled = frame.read_image().await.unwrap();
    img_tiled.save("performance/tiling.png").unwrap();
    frame.present();
    // ANCHOR_END: tiling

    // Tiling should produce nearly the same picture for much less work.
    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let changed = pixel_diff(img_untiled.as_raw(), img_tiled.as_raw());
    let total = img_untiled.as_raw().len() / 4;
    println!(
        "pixels changed by light tiling: {changed} of {total} ({:.1}%)",
        100.0 * changed as f32 / total as f32
    );
    assert!(
        changed * 10 < total,
        "light tiling changed the rendering too much ({changed} of {total} pixels changed)"
    );
}

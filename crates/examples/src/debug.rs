//! Debug mode examples.

use crate::{cwd_to_manual_assets_dir, workspace_dir};

#[tokio::test]
async fn manual_debug() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("debug").unwrap();

    // ANCHOR: setup
    // We'll debug the shadow mapping scene from the previous chapters.
    use renderling::{
        camera::Camera,
        context::Context,
        geometry::Vertex,
        glam::{Mat4, UVec2, Vec3, Vec4},
        gltf::GltfDocument,
        light::{AnalyticalLight, DirectionalLight, Lux},
        primitive::Primitive,
        stage::Stage,
        types::GpuOnlyArray,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let _camera: Camera = {
        let aspect = 1.0;
        let fovy = core::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 10.0;
        let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let eye = Vec3::new(0.5, 0.5, 0.8);
        let target = Vec3::new(0.0, 0.3, 0.0);
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);

        stage
            .new_camera()
            .with_projection_and_view(projection, view)
    };

    let model: GltfDocument<GpuOnlyArray> = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/marble_bust_1k.glb"))
        .unwrap()
        .into_gpu_only();

    let y = -0.01;
    let s = 2.0;
    let floor: Primitive = stage
        .new_primitive()
        .with_vertices(
            stage.new_vertices([
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
            ]),
        )
        .with_material(
            stage
                .new_material()
                .with_albedo_factor(Vec4::new(0.85, 0.85, 0.85, 1.0)),
        );

    let sun: AnalyticalLight<DirectionalLight> = stage
        .new_directional_light()
        .with_direction(Vec3::new(-0.4, -1.0, -0.6).normalize())
        .with_color(Vec4::ONE)
        .with_intensity(Lux::OUTDOOR_OVERCAST_HIGH);

    let shadow_map = stage
        .new_shadow_map(&sun, UVec2::splat(1024), 0.1, 10.0)
        .unwrap();
    shadow_map
        .update(
            &stage,
            model.renderlets_iter().chain(std::iter::once(&floor)),
        )
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_none = frame.read_image().await.unwrap();
    img_none.save("debug/none.png").unwrap();
    frame.present();
    // ANCHOR_END: setup

    // ANCHOR: channel
    use renderling::pbr::debug::DebugChannel;

    // Display the world-space normals, after normal mapping.
    stage.set_debug_mode(DebugChannel::Normals);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_normals = frame.read_image().await.unwrap();
    img_normals.save("debug/normals.png").unwrap();
    frame.present();

    // Display the first set of UV coordinates.
    stage.set_debug_mode(DebugChannel::UvCoords0);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_uv = frame.read_image().await.unwrap();
    img_uv.save("debug/uv-coords.png").unwrap();
    frame.present();

    // Display just the albedo color.
    stage.set_debug_mode(DebugChannel::Albedo);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_albedo = frame.read_image().await.unwrap();
    img_albedo.save("debug/albedo.png").unwrap();
    frame.present();
    // ANCHOR_END: channel

    // Each step should have changed the rendering.
    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let steps = [
        ("normals channel", img_none.as_raw(), img_normals.as_raw()),
        ("uv channel", img_none.as_raw(), img_uv.as_raw()),
        ("albedo channel", img_none.as_raw(), img_albedo.as_raw()),
    ];
    for (name, before, after) in steps {
        let changed = pixel_diff(before, after);
        println!("pixels changed by {name}: {changed}");
        assert!(
            changed > 500,
            "changing {name} did not change the rendering ({changed} pixels changed)"
        );
    }
}

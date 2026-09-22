//! Shadow mapping examples.

use crate::{cwd_to_manual_assets_dir, workspace_dir};

#[tokio::test]
async fn manual_shadow_mapping() {
    let _ = env_logger::builder().try_init();

    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        geometry::Vertex,
        glam::{Mat4, Vec3, Vec4},
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

    // Load the marble bust, as in the lighting section.
    let model: GltfDocument<GpuOnlyArray> = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/marble_bust_1k.glb"))
        .unwrap()
        .into_gpu_only();

    // Add a ground plane for the bust's shadow to land on.
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

    // Create a directional light.
    let sun: AnalyticalLight<DirectionalLight> = stage
        .new_directional_light()
        .with_direction(Vec3::new(-0.4, -1.0, -0.6).normalize())
        .with_color(Vec4::ONE)
        .with_intensity(Lux::OUTDOOR_OVERCAST_HIGH);
    // ANCHOR_END: setup

    cwd_to_manual_assets_dir();

    // Render the scene once without any shadow mapping.
    // ANCHOR: render_without_shadow
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_without_shadow = frame.read_image().await.unwrap();
    img_without_shadow.save("lighting/shadow-none.png").unwrap();
    frame.present();
    // ANCHOR_END: render_without_shadow

    // ANCHOR: shadowmap
    use renderling::glam::UVec2;

    // Create a shadow map for the sun.
    //
    // The first argument is the light to cast shadows from. The `size`
    // determines the resolution of the shadow map - bigger maps give
    // crisper shadows but use more memory. `z_near` and `z_far` bound
    // the light's frustum - only objects within the frustum cast
    // shadows.
    //
    // Shadow maps are stored in a texture atlas shared by the whole
    // stage. The size of the atlas can be configured with
    // `Context::with_shadow_mapping_atlas_texture_size`.
    let shadow_map = stage
        .new_shadow_map(&sun, UVec2::splat(1024), 0.1, 10.0)
        .unwrap();
    // ANCHOR_END: shadowmap

    // ANCHOR: update
    // Update the shadow map by rendering the given primitives as shadow
    // casters, from the light's point of view.
    //
    // In a typical application this is done every frame, before
    // `Stage::render`.
    shadow_map
        .update(
            &stage,
            model.renderlets_iter().chain(std::iter::once(&floor)),
        )
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_with_shadow = frame.read_image().await.unwrap();
    img_with_shadow.save("lighting/shadow.png").unwrap();
    frame.present();
    // ANCHOR_END: update

    // ANCHOR: tuning
    // Tune the shadow map by modifying its descriptor.
    //
    // The bias values compensate for the limited precision of the
    // shadow map. Too little bias causes "shadow acne", where surfaces
    // shadow themselves. Too much bias causes "peter panning", where
    // shadows detach from their casters.
    //
    // `pcf_samples` controls the softness of shadow edges through
    // percentage-closer filtering. Higher values are more expensive.
    {
        let mut desc = shadow_map.descriptor_lock();
        desc.bias_min = 0.0005;
        desc.bias_max = 0.005;
        desc.pcf_samples = 4;
    }

    // Update and render again to see the difference.
    shadow_map
        .update(
            &stage,
            model.renderlets_iter().chain(std::iter::once(&floor)),
        )
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    frame.present();
    // ANCHOR_END: tuning

    // The shadow map should have changed the rendering.
    let without = img_without_shadow.as_raw();
    let with = img_with_shadow.as_raw();
    assert_eq!(without.len(), with.len());
    let changed = without
        .chunks_exact(4)
        .zip(with.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    println!("pixels changed by shadow mapping: {changed}");
    assert!(
        changed > 500,
        "shadow mapping did not change the rendering ({changed} pixels changed)"
    );
}

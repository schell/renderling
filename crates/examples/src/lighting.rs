//! Lighting examples.

use crate::{cwd_to_manual_assets_dir, workspace_dir};

#[tokio::test]
async fn manual_lighting() {
    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        glam::Vec4,
        glam::{Mat4, Vec3},
        gltf::GltfDocument,
        stage::Stage,
        types::GpuOnlyArray,
    };

    let ctx = Context::headless(256, 256).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0))
        .with_lighting(false);

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
    println!("bounds: {:?}", model.bounding_volume());

    let skybox = stage
        .new_skybox_from_path(workspace_dir().join("img/hdr/helipad.hdr"))
        .unwrap();
    stage.use_skybox(&skybox);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    frame.present();
    // ANCHOR_END: setup

    cwd_to_manual_assets_dir();

    // ANCHOR: lighting_on
    stage.set_has_lighting(true);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/no-lights.png").unwrap();
    frame.present();
    // ANCHOR_END: lighting_on

    // ANCHOR: directional
    use renderling::{
        color::css_srgb_color_to_linear,
        light::{AnalyticalLight, DirectionalLight},
    };

    let sunset_amber_sunlight_color = css_srgb_color_to_linear(250, 198, 104);

    let _directional: AnalyticalLight<DirectionalLight> = stage
        .new_directional_light()
        .with_direction(Vec3::new(-0.5, -0.5, 0.0))
        .with_color(sunset_amber_sunlight_color);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/directional.png").unwrap();
    frame.present();
    // ANCHOR_END: directional
}

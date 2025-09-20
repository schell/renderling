//! Skybox manual page.

use crate::{cwd_to_manual_assets_dir, test_output_dir, workspace_dir};

#[tokio::test]
async fn manual_skybox() {
    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        glam::Vec4,
        glam::{Mat4, Vec3},
        stage::Stage,
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

    use renderling::{gltf::GltfDocument, types::GpuOnlyArray};
    let model: GltfDocument<GpuOnlyArray> = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/marble_bust_1k.glb"))
        .unwrap()
        .into_gpu_only();
    println!("bounds: {:?}", model.bounding_volume());

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    frame.present();
    // ANCHOR_END: setup

    // ANCHOR: skybox
    let skybox = stage
        .new_skybox_from_path(workspace_dir().join("img/hdr/helipad.hdr"))
        .unwrap();
    stage.use_skybox(&skybox);
    // ANCHOR_END: skybox

    cwd_to_manual_assets_dir();

    // ANCHOR: render_skybox
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let image = frame.read_image().await.unwrap();
    image.save("skybox.png").unwrap();
    frame.present();
    // ANCHOR_END: render_skybox
}

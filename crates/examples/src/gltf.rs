//! GLTF manual page.

use crate::workspace_dir;

#[tokio::test]
async fn manual_gltf() {
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
        .with_background_color(Vec4::new(0.5, 0.5, 0.5, 1.0));

    let _camera: Camera = {
        let aspect = 1.0;
        let fovy = core::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 1000.0;
        let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let y = 50.0;
        let eye = Vec3::new(120.0, y, 120.0);
        let target = Vec3::new(0.0, y, 0.0);
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);

        stage
            .new_camera()
            .with_projection_and_view(projection, view)
    };
    // ANCHOR_END: setup

    // ANCHOR: load
    use renderling::{gltf::GltfDocument, types::GpuOnlyArray};
    let _model: GltfDocument<GpuOnlyArray> = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/Fox.glb"))
        .unwrap()
        .into_gpu_only();
    // ANCHOR_END: load

    super::cwd_to_manual_assets_dir();

    // ANCHOR: render_1
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("gltf-example-shadow.png").unwrap();
    frame.present();
    // ANCHOR_END: render_1

    // ANCHOR: no_lights
    stage.set_has_lighting(false);
    // ANCHOR_END: no_lights

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("gltf-example-unlit.png").unwrap();
    frame.present();
}

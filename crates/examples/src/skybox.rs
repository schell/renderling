//! Skybox manual page.

#[tokio::test]
async fn manual_stage() {
    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        geometry::Vertex,
        glam::Vec4,
        glam::{Mat4, Vec3},
        stage::Stage,
    };

    let ctx = Context::headless(256, 256).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.5, 0.5, 0.5, 1.0));

    let _camera: Camera = stage
        .new_camera()
        .with_default_perspective(256.0, 256.0)
        .with_view(Mat4::look_at_rh(Vec3::splat(1.5), Vec3::ZERO, Vec3::Y));

    let vertices = stage.new_vertices(renderling::math::unit_cube().into_iter().map(
        |(position, normal)| {
            Vertex::default()
                .with_position(position)
                .with_normal(normal)
                .with_color({
                    // The color can vary from vertex to vertex
                    //
                    // X axis is green
                    let g: f32 = position.x + 0.5;
                    // Y axis is blue
                    let b: f32 = position.y + 0.5;
                    // Z is red
                    let r: f32 = position.z + 0.5;
                    Vec4::new(r, g, b, 1.0)
                })
        },
    ));

    let material = stage.new_material().with_albedo_factor(Vec4::ONE);

    let _prim = stage
        .new_primitive()
        .with_vertices(&vertices)
        .with_material(&material);
    // ANCHOR_END: setup

    // ANCHOR: render_cube
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    frame.present();
    // ANCHOR_END: render_cube

    // Excluded from the manual because it's off-topic
    super::cwd_to_manual_assets_dir();

    // ANCHOR: skybox
    let skybox = stage
        .new_skybox_from_path("qwantani_dusk_2_puresky_1k.hdr")
        .unwrap();
    stage.set_skybox(skybox);
    // ANCHOR_END: skybox

    // ANCHOR: render_skybox
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let image = frame.read_image().await.unwrap();
    image.save("skybox.png").unwrap();
    frame.present();
    // ANCHOR_END: render_skybox
}

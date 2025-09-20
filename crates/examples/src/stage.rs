//! Stage manual page.

#[tokio::test]
async fn manual_stage() {
    env_logger::init();

    // ANCHOR: creation
    use renderling::{context::Context, glam::Vec4, stage::Stage};

    let ctx = Context::headless(256, 256).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.5, 0.5, 0.5, 1.0));
    // ANCHOR_END: creation

    // ANCHOR: camera
    use renderling::{
        camera::Camera,
        glam::{Mat4, Vec3},
    };

    let camera: Camera = stage
        .new_camera()
        .with_default_perspective(256.0, 256.0)
        .with_view(Mat4::look_at_rh(Vec3::splat(1.5), Vec3::ZERO, Vec3::Y));
    // This is technically not necessary because Stage always "uses" the first
    // camera created, but we do it here for demonstration.
    stage.use_camera(&camera);
    // ANCHOR_END: camera

    // ANCHOR: unit_cube_vertices
    use renderling::geometry::{Vertex, Vertices};

    let vertices: Vertices = stage.new_vertices(renderling::math::unit_cube().into_iter().map(
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
    // ANCHOR_END: unit_cube_vertices

    // ANCHOR: unload_vertices
    use renderling::types::GpuOnlyArray;

    let vertices: Vertices<GpuOnlyArray> = vertices.into_gpu_only();
    // ANCHOR_END: unload_vertices

    // ANCHOR: material
    let material = stage
        .new_material()
        .with_albedo_factor(Vec4::ONE)
        .with_has_lighting(false);
    // ANCHOR_END: material

    // ANCHOR: prim
    let prim = stage
        .new_primitive()
        .with_vertices(&vertices)
        .with_material(&material);
    // ANCHOR_END: prim

    // Excluded from the manual because it's off-topic
    let current_dir =
        std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR")).join("manual/src/assets");
    let current_dir = current_dir.canonicalize().unwrap();
    std::env::set_current_dir(current_dir).unwrap();

    // ANCHOR: render
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());

    let img = frame.read_image().await.unwrap();
    img.save("stage-example.png").unwrap();
    frame.present();
    // ANCHOR_END: render

    // ANCHOR: committed_size_bytes
    let bytes_committed = stage.used_gpu_buffer_byte_size();
    println!("bytes_committed: {bytes_committed}");
    // ANCHOR_END: committed_size_bytes

    // ANCHOR: removal
    let staged_prim_count = stage.remove_primitive(&prim);
    assert_eq!(0, staged_prim_count);
    drop(vertices);
    drop(material);
    drop(prim);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("stage-example-gone.png").unwrap();
    frame.present();
    // ANCHOR_END: removal
}

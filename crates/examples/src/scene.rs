//! Scene hierarchy examples.

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

#[tokio::test]
async fn manual_scene() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("scene").unwrap();

    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        glam::{Mat4, Quat, Vec3, Vec4},
        stage::Stage,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let _camera: Camera = {
        let aspect = 1.0;
        let fovy = core::f32::consts::PI / 4.0;
        let projection = Mat4::perspective_rh(fovy, aspect, 0.1, 100.0);
        let eye = Vec3::new(0.0, 2.0, 6.0);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);

        stage
            .new_camera()
            .with_projection_and_view(projection, view)
    };

    // Build a three-level hierarchy: root -> child -> grandchild.
    //
    // Each node has a _local_ transform, relative to its parent.
    let root = stage.new_nested_transform();
    let child = stage
        .new_nested_transform()
        .with_local_translation(Vec3::X * 1.2);
    let grandchild = stage
        .new_nested_transform()
        .with_local_translation(Vec3::X * 1.2);
    root.add_child(&child);
    child.add_child(&grandchild);

    // Attach a colored cube to each node. Primitives take the node's
    // _global_ transform, which is composed from the whole hierarchy.
    let cube = |color: Vec4| {
        stage
            .new_primitive()
            .with_vertices(stage.new_vertices(unit_cube()))
            .with_material(
                stage
                    .new_material()
                    .with_albedo_factor(color)
                    .with_has_lighting(false),
            )
    };
    cube(Vec4::new(0.6, 0.6, 0.6, 1.0)).with_transform(&root);
    cube(Vec4::new(0.0, 0.85, 0.85, 1.0)).with_transform(&child);
    cube(Vec4::new(0.95, 0.85, 0.1, 1.0)).with_transform(&grandchild);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_before = frame.read_image().await.unwrap();
    img_before.save("scene/hierarchy.png").unwrap();
    frame.present();
    // ANCHOR_END: setup

    // ANCHOR: parent
    // Rotate the root - the entire hierarchy rotates with it, because
    // the child and grandchild are positioned relative to the root.
    root.set_local_rotation(Quat::from_axis_angle(Vec3::Y, core::f32::consts::FRAC_PI_4));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_parent = frame.read_image().await.unwrap();
    img_parent.save("scene/moved-parent.png").unwrap();
    frame.present();
    // ANCHOR_END: parent

    // ANCHOR: child
    // Move the child - the root stays put, and the grandchild follows the
    // child.
    child.set_local_translation(Vec3::new(1.2, 1.2, 0.0));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_child = frame.read_image().await.unwrap();
    img_child.save("scene/moved-child.png").unwrap();
    frame.present();
    // ANCHOR_END: child

    // Each step should have changed the rendering.
    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let steps = [
        (
            "rotating the parent",
            img_before.as_raw(),
            img_parent.as_raw(),
        ),
        ("moving the child", img_parent.as_raw(), img_child.as_raw()),
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

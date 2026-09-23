//! Animation examples: playback, skinning and morph targets.

use crate::{cwd_to_manual_assets_dir, workspace_dir};

#[tokio::test]
async fn manual_animation() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("animation").unwrap();

    // ANCHOR: setup
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
        .with_bloom(false)
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let projection = camera::perspective(512.0, 512.0);
    let view = camera::look_at(Vec3::Z * 3.0, Vec3::ZERO, Vec3::Y);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);

    // Load a GLTF file containing an animation.
    let doc = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/animated_triangle.gltf"))
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_frame_0 = frame.read_image().await.unwrap();
    img_frame_0.save("animation/frame-0.png").unwrap();
    frame.present();
    // ANCHOR_END: setup

    // ANCHOR: animator
    use renderling::gltf::Animator;

    // Collect every node in the scene, and build an `Animator` for the
    // document's first animation clip.
    let nodes = doc
        .recursive_nodes_in_scene(doc.default_scene.unwrap_or_default())
        .collect::<Vec<_>>();
    let mut animator = Animator::new(nodes, doc.animations.first().unwrap().clone());
    // ANCHOR_END: animator

    // ANCHOR: progress
    // Advance the animation and render frames. In an application this
    // would happen every frame, with `dt` the time since the last one.
    let dt = 1.0 / 8.0;
    let mut img_frame_2 = None;
    let mut img_frame_5 = None;
    for i in 1..=8 {
        animator.progress(dt).unwrap();

        if i == 2 || i == 5 {
            let frame = ctx.get_next_frame().unwrap();
            stage.render(&frame.view());
            let img = frame.read_image().await.unwrap();
            img.save(format!("animation/frame-{i}.png")).unwrap();
            if i == 2 {
                img_frame_2 = Some(img);
            } else {
                img_frame_5 = Some(img);
            }
            frame.present();
        }
    }
    let img_frame_2 = img_frame_2.unwrap();
    let img_frame_5 = img_frame_5.unwrap();
    // ANCHOR_END: progress

    // The animation should have moved the triangle.
    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    for (name, before, after) in [
        ("frame 0 to 2", img_frame_0.as_raw(), img_frame_2.as_raw()),
        ("frame 2 to 5", img_frame_2.as_raw(), img_frame_5.as_raw()),
    ] {
        let changed = pixel_diff(before, after);
        println!("pixels changed from {name}: {changed}");
        assert!(
            changed > 500,
            "the animation did not change the rendering between {name} ({changed} pixels changed)"
        );
    }
}

#[tokio::test]
async fn manual_morph_targets() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("animation").unwrap();

    // ANCHOR: morph
    use renderling::{
        camera,
        context::Context,
        geometry::{MorphTarget, Vertex},
        glam::{Vec3, Vec4},
        stage::Stage,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_lighting(false)
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let projection = camera::perspective(512.0, 512.0);
    let view = camera::look_at(Vec3::Z * 3.0, Vec3::ZERO, Vec3::Y);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);

    // A quad.
    let quad = stage
        .new_primitive()
        .with_vertices(stage.new_vertices([
            Vertex::default().with_position([-1.0, -1.0, 0.0]),
            Vertex::default().with_position([1.0, -1.0, 0.0]),
            Vertex::default().with_position([1.0, 1.0, 0.0]),
            Vertex::default().with_position([-1.0, 1.0, 0.0]),
        ]))
        .with_material(
            stage
                .new_material()
                .with_albedo_factor(Vec4::new(0.95, 0.85, 0.1, 1.0))
                .with_has_lighting(false),
        );

    // One morph target per vertex, describing where each vertex moves at
    // full weight. Here the quad twists: the left edge comes toward the
    // camera, the right edge moves away.
    let twist = stage.new_morph_targets([vec![
        MorphTarget {
            position: Vec3::new(-1.0, -1.0, 1.0),
            ..Default::default()
        },
        MorphTarget {
            position: Vec3::new(1.0, -1.0, -1.0),
            ..Default::default()
        },
        MorphTarget {
            position: Vec3::new(1.0, 1.0, -1.0),
            ..Default::default()
        },
        MorphTarget {
            position: Vec3::new(-1.0, 1.0, 1.0),
            ..Default::default()
        },
    ]]);
    let weights = stage.new_morph_target_weights([0.0f32]);
    quad.set_morph_targets(twist, weights.clone());

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_flat = frame.read_image().await.unwrap();
    img_flat.save("animation/morph-0.png").unwrap();
    frame.present();

    // Morph weights can be changed at runtime.
    weights.set_item(0, 1.0);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_twisted = frame.read_image().await.unwrap();
    img_twisted.save("animation/morph-1.png").unwrap();
    frame.present();
    // ANCHOR_END: morph

    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let changed = pixel_diff(img_flat.as_raw(), img_twisted.as_raw());
    println!("pixels changed by morphing: {changed}");
    assert!(
        changed > 500,
        "changing the morph weight did not change the rendering ({changed} pixels changed)"
    );
}

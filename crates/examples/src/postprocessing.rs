//! Post-processing examples: bloom and tonemapping.

use crate::{cwd_to_manual_assets_dir, workspace_dir};

#[tokio::test]
async fn manual_postprocessing() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("postprocessing").unwrap();

    // ANCHOR: setup
    use renderling::{
        camera,
        context::Context,
        glam::{Vec3, Vec4},
        stage::Stage,
    };

    let width = 512;
    let height = 256;
    let ctx = Context::headless(width, height).await;
    // Bloom is on by default - we turn it off so we can show the
    // difference.
    let stage: Stage = ctx
        .new_stage()
        .with_bloom(false)
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let projection = camera::perspective(width as f32, height as f32);
    let view = camera::look_at(Vec3::new(0.0, 2.0, 18.0), Vec3::ZERO, Vec3::Y);
    let _camera = stage
        .new_camera()
        .with_projection_and_view(projection, view);

    // A night sky and an emissive-strength test model full of glowing
    // objects, so there are bright areas for the bloom effect to pick up.
    let skybox = stage
        .new_skybox_from_path(workspace_dir().join("img/hdr/night.hdr"))
        .unwrap();
    stage.use_skybox(&skybox);
    let ibl = stage.new_ibl(&skybox);
    stage.use_ibl(&ibl);

    let _model = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/EmissiveStrengthTest.glb"))
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_no_bloom = frame.read_image().await.unwrap();
    img_no_bloom.save("postprocessing/bloom-none.png").unwrap();
    frame.present();
    // ANCHOR_END: setup

    // ANCHOR: bloom
    // Turn bloom on, either at stage creation with `.with_bloom(true)` or
    // at runtime.
    stage.set_has_bloom(true);
    // How much of the blurred bright areas to mix back into the image.
    stage.set_bloom_mix_strength(0.1);
    // The radius of the blur, in texels.
    stage.set_bloom_filter_radius(2.0);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_bloom = frame.read_image().await.unwrap();
    img_bloom.save("postprocessing/bloom.png").unwrap();
    frame.present();
    // ANCHOR_END: bloom

    // ANCHOR: exposure
    // The stage renders in high dynamic range (HDR). Before the image is
    // shown it is multiplied by the exposure and run through the tone
    // mapping algorithm, which maps the HDR values into the display's
    // range.
    let mut config = stage.tonemapping().get_tonemapping_config();
    config.exposure = 3.0;
    stage.tonemapping().set_tonemapping_config(config);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_exposure = frame.read_image().await.unwrap();
    img_exposure.save("postprocessing/exposure.png").unwrap();
    frame.present();
    // ANCHOR_END: exposure

    // ANCHOR: tonemap
    use renderling::tonemapping::Tonemap;

    // By default no tone mapping algorithm is used (`Tonemap::NONE`), and
    // HDR values outside the display range simply clip. A filmic operator
    // like ACES compresses the highlights instead.
    config.tonemap = Tonemap::ACES_HILL_EXPOSURE_BOOST;
    config.exposure = 1.0;
    stage.tonemapping().set_tonemapping_config(config);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_aces = frame.read_image().await.unwrap();
    img_aces.save("postprocessing/tonemap-aces.png").unwrap();
    frame.present();
    // ANCHOR_END: tonemap

    // Each step should have changed the rendering.
    let pixel_diff = |a: &[u8], b: &[u8]| {
        a.chunks_exact(4)
            .zip(b.chunks_exact(4))
            .filter(|(a, b)| a != b)
            .count()
    };
    let steps = [
        ("bloom", img_no_bloom.as_raw(), img_bloom.as_raw()),
        ("exposure", img_bloom.as_raw(), img_exposure.as_raw()),
        ("tone mapping", img_exposure.as_raw(), img_aces.as_raw()),
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

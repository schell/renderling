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
        light::{AnalyticalLight, DirectionalLight, Lux},
    };

    let sunset_amber_sunlight_color = css_srgb_color_to_linear(250, 198, 104);

    let directional: AnalyticalLight<DirectionalLight> = stage
        .new_directional_light()
        .with_direction(Vec3::new(-0.5, -0.5, 0.0))
        .with_color(sunset_amber_sunlight_color)
        .with_intensity(Lux::OUTDOOR_OVERCAST_HIGH);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/directional.png").unwrap();
    frame.present();
    // ANCHOR_END: directional

    // ANCHOR: remove_directional
    stage.remove_light(&directional);
    drop(directional);
    // ANCHOR_END: remove_directional

    // ANCHOR: point
    use renderling::light::{Candela, PointLight};

    let point: AnalyticalLight<PointLight> = stage
        .new_point_light()
        .with_position({
            let bust_aabb = model.bounding_volume().unwrap();
            bust_aabb.max
        })
        .with_color(sunset_amber_sunlight_color)
        .with_intensity(Candela(100.0));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/point.png").unwrap();
    frame.present();
    // ANCHOR_END: point

    // ANCHOR: remove_point
    stage.remove_light(&point);
    drop(point);
    // ANCHOR_END: remove_point

    // ANCHOR: spot
    use renderling::light::SpotLight;

    let camera_eye = Vec3::new(0.5, 0.5, 0.8);
    let camera_target = Vec3::new(0.0, 0.3, 0.0);
    let position = camera_eye;
    let direction = camera_target - camera_eye;
    let spot: AnalyticalLight<SpotLight> = stage
        .new_spot_light()
        .with_position(position)
        .with_direction(direction)
        // the cutoff values determine the angle of the cone
        .with_inner_cutoff(0.15)
        .with_outer_cutoff(0.2)
        .with_color(sunset_amber_sunlight_color)
        .with_intensity(Candela(12_000.0));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/spot.png").unwrap();
    frame.present();
    // ANCHOR_END: spot

    // ANCHOR: remove_spot
    stage.remove_light(&spot);
    drop(spot);
    // ANCHOR_END: remove_spot
}

#[tokio::test]
async fn manual_lighting_ibl() {
    let _ = env_logger::builder().try_init();

    cwd_to_manual_assets_dir();

    // ANCHOR: ibl_setup
    use renderling::{
        camera::Camera,
        context::Context,
        glam::Vec4,
        glam::{Mat4, Vec3},
        gltf::GltfDocument,
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

    let skybox = stage
        .new_skybox_from_path(workspace_dir().join("img/hdr/helipad.hdr"))
        .unwrap();
    stage.use_skybox(&skybox);
    // ANCHOR_END: ibl_setup

    // ANCHOR: ibl
    use renderling::pbr::ibl::Ibl;

    let ibl: Ibl = stage.new_ibl(&skybox);
    stage.use_ibl(&ibl);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/ibl.png").unwrap();
    frame.present();
    // ANCHOR_END: ibl

    // ANCHOR: mix
    use renderling::{color::css_srgb_color_to_linear, light::Candela};

    let sunset_amber_sunlight_color = css_srgb_color_to_linear(250, 198, 104);
    let _point = stage
        .new_point_light()
        .with_position({
            let bust_aabb = model.bounding_volume().unwrap();
            bust_aabb.max
        })
        .with_color(sunset_amber_sunlight_color)
        .with_intensity(Candela(100.0));

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("lighting/ibl-analytical-mixed.png").unwrap();
    frame.present();
    // ANCHOR_END: mix
}

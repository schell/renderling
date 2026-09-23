//! Materials and textures examples.

use crate::{cwd_to_manual_assets_dir, workspace_dir};
use renderling::{
    geometry::Vertex,
    glam::{Vec2, Vec3},
};

/// A unit cube with UV coordinates, centered on the origin.
fn uv_unit_cube() -> Vec<Vertex> {
    let p: [Vec3; 8] = renderling::math::UNIT_POINTS;
    let tl = Vec2::new(0.0, 0.0);
    let tr = Vec2::new(1.0, 0.0);
    let bl = Vec2::new(0.0, 1.0);
    let br = Vec2::new(1.0, 1.0);

    vec![
        // top
        Vertex::default().with_position(p[0]).with_uv0(bl),
        Vertex::default().with_position(p[2]).with_uv0(tr),
        Vertex::default().with_position(p[1]).with_uv0(tl),
        Vertex::default().with_position(p[0]).with_uv0(bl),
        Vertex::default().with_position(p[3]).with_uv0(br),
        Vertex::default().with_position(p[2]).with_uv0(tr),
        // bottom
        Vertex::default().with_position(p[4]).with_uv0(bl),
        Vertex::default().with_position(p[6]).with_uv0(tr),
        Vertex::default().with_position(p[5]).with_uv0(tl),
        Vertex::default().with_position(p[4]).with_uv0(bl),
        Vertex::default().with_position(p[7]).with_uv0(br),
        Vertex::default().with_position(p[6]).with_uv0(tr),
        // left
        Vertex::default().with_position(p[7]).with_uv0(bl),
        Vertex::default().with_position(p[0]).with_uv0(tr),
        Vertex::default().with_position(p[1]).with_uv0(tl),
        Vertex::default().with_position(p[7]).with_uv0(bl),
        Vertex::default().with_position(p[4]).with_uv0(br),
        Vertex::default().with_position(p[0]).with_uv0(tr),
        // right
        Vertex::default().with_position(p[5]).with_uv0(bl),
        Vertex::default().with_position(p[2]).with_uv0(tr),
        Vertex::default().with_position(p[3]).with_uv0(tl),
        Vertex::default().with_position(p[5]).with_uv0(bl),
        Vertex::default().with_position(p[6]).with_uv0(br),
        Vertex::default().with_position(p[2]).with_uv0(tr),
        // front
        Vertex::default().with_position(p[4]).with_uv0(bl),
        Vertex::default().with_position(p[3]).with_uv0(tr),
        Vertex::default().with_position(p[0]).with_uv0(tl),
        Vertex::default().with_position(p[4]).with_uv0(bl),
        Vertex::default().with_position(p[5]).with_uv0(br),
        Vertex::default().with_position(p[3]).with_uv0(tr),
    ]
}

/// Returns the number of pixels that differ between two raw RGBA images.
fn pixel_diff(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4)
        .zip(b.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count()
}

#[tokio::test]
async fn manual_materials() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("material").unwrap();

    // ANCHOR: setup
    use renderling::{
        camera::Camera,
        context::Context,
        glam::{Mat4, Vec3, Vec4},
        primitive::Primitive,
        stage::Stage,
    };

    let ctx = Context::headless(512, 512).await;
    let stage: Stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(0.25, 0.25, 0.25, 1.0));

    let _camera: Camera = {
        let aspect = 1.0;
        let fovy = core::f32::consts::PI / 4.0;
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Mat4::perspective_rh(fovy, aspect, znear, zfar);
        let eye = Vec3::new(0.0, 0.5, 4.2);
        let target = Vec3::ZERO;
        let up = Vec3::Y;
        let view = Mat4::look_at_rh(eye, target, up);

        stage
            .new_camera()
            .with_projection_and_view(projection, view)
    };

    // Two cubes sharing the same vertices, each with its own transform.
    let geometry = stage.new_vertices(uv_unit_cube());
    let offset = 0.9;
    let left: Primitive = stage
        .new_primitive()
        .with_vertices(&geometry)
        .with_transform(
            stage
                .new_transform()
                .with_translation(Vec3::new(-offset, 0.0, 0.0)),
        );
    let right: Primitive = stage
        .new_primitive()
        .with_vertices(&geometry)
        .with_transform(
            stage
                .new_transform()
                .with_translation(Vec3::new(offset, 0.0, 0.0)),
        );
    // ANCHOR_END: setup

    // ANCHOR: albedo_texture
    use renderling::atlas::AtlasImage;

    // Load two images and stage them in the stage's texture atlas.
    //
    // `set_images` returns one `AtlasTexture` handle per image. Note that
    // calling it again repacks the atlas and invalidates any previous
    // handles, so stage all the images you need up front.
    let sandstone = AtlasImage::from_path(workspace_dir().join("img/sandstone.png")).unwrap();
    let dirt = AtlasImage::from_path(workspace_dir().join("img/dirt.jpg")).unwrap();
    let entries = stage.set_images([sandstone, dirt]).unwrap();
    // ANCHOR_END: albedo_texture

    // ANCHOR: albedo_factor
    use renderling::color::css_srgb_color_to_linear;

    // Create an unlit material with a teal albedo factor.
    let material = stage
        .new_material()
        .with_albedo_factor(css_srgb_color_to_linear(0, 128, 128))
        .with_has_lighting(false);

    // Both primitives share the same material.
    left.set_material(&material);
    right.set_material(&material);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_teal = frame.read_image().await.unwrap();
    img_teal.save("material/albedo-factor.png").unwrap();
    frame.present();
    // ANCHOR_END: albedo_factor

    // ANCHOR: use_texture
    // The albedo factor multiplies the albedo texture, so reset it to
    // white and use the first image as the albedo texture.
    material
        .set_albedo_factor(Vec4::ONE)
        .set_albedo_texture(&entries[0]);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_sandstone = frame.read_image().await.unwrap();
    img_sandstone.save("material/albedo-texture.png").unwrap();
    frame.present();
    // ANCHOR_END: use_texture

    // ANCHOR: update
    // Updating a material at runtime is just as easy. Here we switch the
    // albedo texture of both cubes to the second image with a single call.
    material.set_albedo_texture(&entries[1]);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_dirt = frame.read_image().await.unwrap();
    img_dirt.save("material/albedo-updated.png").unwrap();
    frame.present();
    // ANCHOR_END: update

    // ANCHOR: pbr
    use renderling::light::{AnalyticalLight, DirectionalLight, Lux};

    // Turn on lighting and give the scene an environment so the material
    // has something to reflect. (See the skybox and image-based lighting
    // chapters.)
    let skybox = stage
        .new_skybox_from_path(workspace_dir().join("img/hdr/helipad.hdr"))
        .unwrap();
    stage.use_skybox(&skybox);
    let ibl = stage.new_ibl(&skybox);
    stage.use_ibl(&ibl);

    let _sun: AnalyticalLight<DirectionalLight> = stage
        .new_directional_light()
        .with_direction(Vec3::new(-0.4, -1.0, -0.5).normalize())
        .with_color(Vec4::ONE)
        .with_intensity(Lux::OUTDOOR_OVERCAST_HIGH);

    // Give the material PBR parameters: make our sandstone cubes metallic
    // with low roughness, like polished gold.
    material
        .set_has_lighting(true)
        .set_albedo_texture(&entries[0])
        .set_metallic_factor(1.0)
        .set_roughness_factor(0.2);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_pbr = frame.read_image().await.unwrap();
    img_pbr.save("material/pbr.png").unwrap();
    frame.present();
    // ANCHOR_END: pbr

    // ANCHOR: roughness
    // Increasing the roughness blurs the reflections, making the surface
    // appear matte.
    material.set_roughness_factor(1.0);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_rough = frame.read_image().await.unwrap();
    img_rough.save("material/pbr-rough.png").unwrap();
    frame.present();
    // ANCHOR_END: roughness

    // ANCHOR: emissive
    // Emissive color is added directly to the final color, regardless of
    // lighting. Combined with bloom it makes objects glow.
    material
        .set_emissive_factor(Vec3::new(1.0, 0.25, 0.1))
        .set_emissive_strength_multiplier(2.0);

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img_emissive = frame.read_image().await.unwrap();
    img_emissive.save("material/emissive.png").unwrap();
    frame.present();
    // ANCHOR_END: emissive

    // Each step should have changed the rendering.
    let steps = [
        ("albedo factor", img_teal.as_raw(), img_sandstone.as_raw()),
        ("albedo texture", img_sandstone.as_raw(), img_dirt.as_raw()),
        ("texture update", img_dirt.as_raw(), img_pbr.as_raw()),
        ("metallic", img_pbr.as_raw(), img_rough.as_raw()),
        ("roughness", img_rough.as_raw(), img_emissive.as_raw()),
    ];
    for (name, before, after) in steps {
        let changed = pixel_diff(before, after);
        println!("pixels changed by {name}: {changed}");
        assert!(
            changed > 500,
            "changing {name} did not change the rendering ({changed} pixels changed)"
        );
    }

    // The textured render should contain far more distinct colors than a
    // flat, untextured render.
    let unique_colors = |raw: &[u8]| {
        raw.chunks_exact(4)
            .map(|p| [p[0], p[1], p[2], p[3]])
            .collect::<std::collections::HashSet<[u8; 4]>>()
            .len()
    };
    let teal_colors = unique_colors(img_teal.as_raw());
    let textured_colors = unique_colors(img_sandstone.as_raw());
    println!("unique colors: teal={teal_colors}, textured={textured_colors}");
    assert!(
        textured_colors > teal_colors + 50,
        "expected the albedo texture to render ({textured_colors} unique colors)"
    );
}

#[tokio::test]
async fn manual_materials_normal_map() {
    let _ = env_logger::builder().try_init();
    cwd_to_manual_assets_dir();
    std::fs::create_dir_all("material").unwrap();

    // ANCHOR: normal_map
    use renderling::{context::Context, glam::Vec4, stage::Stage};

    // GLTF files can carry textures of their own, including normal maps.
    // This brick sphere has lighting and a normal map baked in, and the
    // loader wires them into the stage and material for us.
    let ctx = Context::headless(800, 450).await;
    let stage: Stage = ctx
        .new_stage()
        .with_lighting(true)
        .with_background_color(Vec4::new(0.01, 0.01, 0.01, 1.0));

    let _doc = stage
        .load_gltf_document_from_path(workspace_dir().join("gltf/normal_mapping_brick_sphere.glb"))
        .unwrap();

    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let img = frame.read_image().await.unwrap();
    img.save("material/normal-map.png").unwrap();
    frame.present();
    // ANCHOR_END: normal_map
}

//! WASM tests.
#![allow(dead_code)]

use glam::Vec4;
use image::DynamicImage;
use renderling::prelude::*;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::wasm_bindgen::UnwrapThrowExt;
use wire_types::{Error, PixelType};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn can_create_headless_ctx() {
    let _ctx = renderling::Context::try_new_headless(256, 256, None)
        .await
        .unwrap_throw();
}

#[wasm_bindgen_test]
async fn stage_creation() {
    let ctx = renderling::Context::try_new_headless(256, 256, None)
        .await
        .unwrap_throw();
    let _stage = ctx.new_stage();
}

fn image_from_bytes(bytes: &[u8]) -> image::DynamicImage {
    image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .expect_throw("could not guess format")
        .decode()
        .expect_throw("could not decode")
}

async fn load_test_img(path: &str) -> image::DynamicImage {
    let result = loading_bytes::load(&format!("http://127.0.0.1:4000/test_img/{path}")).await;
    let bytes = match result {
        Ok(bytes) => bytes,
        Err(e) => panic!("{e}"),
    };
    image_from_bytes(&bytes)
}

async fn assert_img_eq(filename: &str, seen: impl Into<image::DynamicImage>) {
    let img: DynamicImage = seen.into();
    let width = img.width();
    let height = img.height();
    let (pixel, bytes) = match img {
        DynamicImage::ImageRgb8(image_buffer) => (PixelType::Rgb8, image_buffer.to_vec()),
        DynamicImage::ImageRgba8(image_buffer) => (PixelType::Rgba8, image_buffer.to_vec()),
        _ => panic!("Image type is not yet supported in the WASM tests"),
    };
    let wire_data = wire_types::Image {
        width,
        height,
        bytes,
        pixel,
    };
    let data = serde_json::to_string(&wire_data).unwrap();
    let result = loading_bytes::post_json_wasm::<Result<(), wire_types::Error>>(
        &format!("http://127.0.0.1:4000/assert_img_eq/{filename}"),
        &data,
    )
    .await
    .unwrap();

    if let Err(Error { description }) = result {
        panic!("{description}");
    }
}

#[wasm_bindgen_test]
async fn can_load_image() {
    let _img = load_test_img("jolt.png").await;
}

#[wasm_bindgen_test]
async fn can_img_diff() {
    let a = load_test_img("jolt.png").await;
    assert_img_eq("jolt.png", a).await;

    let b = load_test_img("cmy_triangle/hdr.png").await;
    assert_img_eq("cmy_triangle/hdr.png", b).await;
}

#[wasm_bindgen_test]
async fn can_clear_background() {
    let ctx = Context::try_new_headless(100, 100, None).await.unwrap();
    let stage = ctx
        .new_stage()
        .with_background_color(Vec4::new(1.0, 0.0, 0.0, 1.0));
    let frame = ctx.get_next_frame().unwrap();
    stage.render(&frame.view());
    let seen = frame.read_image().unwrap();
    assert_img_eq("cmy_triangle/hdr.png", seen).await;
}

// #[wasm_bindgen_test]
// #[should_panic]
// async fn can_save_wrong_diffs() {
//     let img = load_test_img("jolt.png").await;
//     assert_img_eq("cmy_triangle/hdr.png", img).await;
// }

// #[wasm_bindgen_test]
// async fn can_render_hello_triangle() {
//     // This is a wasm version of cmy_triangle_sanity
//     let ctx = Context::try_new_headless(100, 100, None).await.unwrap();
//     let stage = ctx.new_stage().with_background_color(Vec4::splat(1.0));
//     let _camera = stage.new_camera(Camera::default_ortho2d(100.0, 100.0));
//     let _rez = stage
//         .builder()
//         .with_vertices(renderling::::right_tri_vertices())
//         .build();

//     let frame = ctx.get_next_frame().unwrap();
//     stage.render(&frame.view());
//     frame.present();

//     let hdr_img = stage.hdr_texture().read_hdr_image(&ctx).unwrap();
// }

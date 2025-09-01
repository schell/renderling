//! Provides image diffing for testing.
use glam::{Vec3, Vec4, Vec4Swizzles};
use image::{DynamicImage, Luma, Rgb, Rgb32FImage, Rgba32FImage};
use snafu::prelude::*;
use std::path::Path;

pub const TEST_IMG_DIR: &str = concat!(std::env!("CARGO_WORKSPACE_DIR"), "test_img");
pub const TEST_OUTPUT_DIR: &str = concat!(std::env!("CARGO_WORKSPACE_DIR"), "test_output");
pub const WASM_TEST_OUTPUT_DIR: &str =
    concat!(std::env!("CARGO_WORKSPACE_DIR"), "test_output/wasm");
const PIXEL_MAGNITUDE_THRESHOLD: f32 = 0.1;
pub const LOW_PIXEL_THRESHOLD: f32 = 0.02;
const IMAGE_DIFF_THRESHOLD: f32 = 0.05;

fn checkerboard_background_color(x: u32, y: u32) -> Vec3 {
    let size = 16;
    let x_square_index = x / size;
    let x_grey = x_square_index % 2 == 0;
    let y_square_index = y / size;
    let y_grey = y_square_index % 2 == 0;
    if (x_grey && y_grey) || (!x_grey && !y_grey) {
        Vec3::from([0.5, 0.5, 0.5])
    } else {
        Vec3::from([1.0, 1.0, 1.0])
    }
}

#[derive(Debug, Snafu)]
enum ImgDiffError {
    #[snafu(display("Images are different sizes. Expected {lhs:?}, saw {rhs:?}"))]
    ImageSize { lhs: (u32, u32), rhs: (u32, u32) },
}

pub struct DiffCfg {
    /// The threshold for a pixel to be considered different.
    ///
    /// Difference is measured as the magnitude of vector subtraction
    /// between the two pixels.
    pub pixel_threshold: f32,
    /// The percentage of "different" pixels (as determined using
    /// `pixel_threshold`) to "correct" pixels that the image must contain
    /// before it is considered an error.
    pub image_threshold: f32,
    /// The name of the test.
    pub test_name: Option<&'static str>,
    /// The output directory to store comparisons in.
    pub output_dir: &'static str,
}

impl Default for DiffCfg {
    fn default() -> Self {
        Self {
            pixel_threshold: PIXEL_MAGNITUDE_THRESHOLD,
            image_threshold: IMAGE_DIFF_THRESHOLD,
            test_name: None,
            output_dir: TEST_OUTPUT_DIR,
        }
    }
}

pub struct DiffResults {
    num_pixels: usize,
    diff_image: Rgb32FImage,
    mask_image: DynamicImage,
    max_delta_length: f32,
    avg_delta_length: f32,
}

fn get_results(
    left_image: &Rgba32FImage,
    right_image: &Rgba32FImage,
    threshold: f32,
) -> Result<Option<DiffResults>, ImgDiffError> {
    let lid @ (width, height) = left_image.dimensions();
    let rid = right_image.dimensions();
    snafu::ensure!(lid == rid, ImageSizeSnafu { lhs: lid, rhs: rid });

    let results = left_image
        .enumerate_pixels()
        .flat_map(|(x, y, left_pixel)| {
            let right_pixel = right_image.get_pixel(x, y);
            if left_pixel == right_pixel {
                None
            } else {
                // pre-multiply alpha
                let left_pixel = Vec4::from(left_pixel.0);
                let left_pixel = (left_pixel * left_pixel.w).xyz();
                let right_pixel = Vec4::from(right_pixel.0);
                let right_pixel = (right_pixel * right_pixel.w).xyz();
                let delta = (left_pixel - right_pixel).abs();
                if delta.length() > threshold {
                    Some((x, y, delta))
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    let mut max_delta_length: f32 = 0.0;
    let mut sum_delta_length: f32 = 0.0;
    let diffs: usize = results.len();
    if diffs == 0 {
        Ok(None)
    } else {
        let mut mask_image = image::ImageBuffer::from_pixel(width, height, Luma([255u8]));
        let mut output_image = image::ImageBuffer::from_pixel(width, height, Rgb([0.0, 0.0, 0.0]));

        for x in 0..width {
            for y in 0..height {
                output_image.put_pixel(x, y, Rgb(checkerboard_background_color(x, y).into()));
            }
        }

        for (x, y, delta) in results {
            let length = delta.length();
            sum_delta_length += length;
            max_delta_length = length.max(max_delta_length);
            mask_image.put_pixel(x, y, Luma([0]));
            output_image.put_pixel(x, y, Rgb(delta.into()));
        }
        Ok(Some(DiffResults {
            num_pixels: diffs,
            diff_image: output_image,
            mask_image: mask_image.into(),
            max_delta_length,
            avg_delta_length: sum_delta_length / diffs as f32,
        }))
    }
}

pub fn save_to(
    dir: impl AsRef<std::path::Path>,
    filename: impl AsRef<std::path::Path>,
    seen: impl Into<DynamicImage>,
) -> Result<(), String> {
    let path = dir.as_ref().join(filename);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let img: DynamicImage = seen.into();
    let img_buffer = img.into_rgba8();
    let img = DynamicImage::from(img_buffer);
    img.save(path).map_err(|e| e.to_string())
}

pub fn save(filename: impl AsRef<std::path::Path>, seen: impl Into<DynamicImage>) {
    save_to(TEST_OUTPUT_DIR, filename, seen).unwrap()
}

pub fn assert_eq_cfg(
    filename: &str,
    lhs: impl Into<DynamicImage>,
    rhs: impl Into<DynamicImage>,
    cfg: DiffCfg,
) -> Result<(), String> {
    let lhs = lhs.into();
    let lhs = lhs.into_rgba32f();
    let rhs = rhs.into().into_rgba32f();
    let DiffCfg {
        pixel_threshold,
        image_threshold,
        test_name,
        output_dir,
    } = cfg;
    let results = match get_results(&lhs, &rhs, pixel_threshold) {
        Ok(maybe_diff) => maybe_diff,
        Err(e) => return Err(format!("Asserting {filename} failed: {e}")),
    };
    if let Some(DiffResults {
        num_pixels: diffs,
        diff_image,
        mask_image,
        max_delta_length,
        avg_delta_length,
    }) = results
    {
        println!("{filename} has {diffs} pixel differences (threshold={pixel_threshold})");
        println!("  max_delta_length: {max_delta_length}");
        println!(
            "  avg_delta_length: {avg_delta_length} (average of deltas of pixels past the \
             threshold)"
        );
        let percent_diff = diffs as f32 / (lhs.width() * lhs.height()) as f32;
        println!("{filename}'s image is {percent_diff} different (threshold={image_threshold})");
        if percent_diff < image_threshold {
            return Ok(());
        }

        let mut dir = Path::new(output_dir).join(test_name.unwrap_or(filename));
        dir.set_extension("");
        std::fs::create_dir_all(&dir).expect("cannot create test output dir");
        let expected = dir.join("expected.png");
        let seen = dir.join("seen.png");
        let diff = dir.join("diff.png");
        let mask = dir.join("mask.png");
        let lhs = DynamicImage::from(lhs).into_rgba8();
        let rhs = DynamicImage::from(rhs).into_rgba8();
        lhs.save_with_format(&expected, image::ImageFormat::Png)
            .expect("can't save expected");
        rhs.save_with_format(&seen, image::ImageFormat::Png)
            .expect("can't save seen");
        let diff_image = DynamicImage::from(diff_image).into_rgba8();
        diff_image
            .save_with_format(&diff, image::ImageFormat::Png)
            .expect("can't save diff");
        let mask_image = mask_image.into_rgba8();
        mask_image
            .save_with_format(&mask, image::ImageFormat::Png)
            .expect("can't save diff mask");
        Err(format!(
            "{} has >= {} differences above the threshold\nexpected: {}\nseen: {}\ndiff: {}",
            filename,
            diffs,
            expected.display(),
            seen.display(),
            diff.display()
        ))
    } else {
        Ok(())
    }
}

pub fn assert_eq(filename: &str, lhs: impl Into<DynamicImage>, rhs: impl Into<DynamicImage>) {
    assert_eq_cfg(filename, lhs, rhs, DiffCfg::default()).unwrap()
}

pub fn assert_img_eq_cfg_result(
    filename: &str,
    seen: impl Into<DynamicImage>,
    cfg: DiffCfg,
) -> Result<(), String> {
    let path = Path::new(TEST_IMG_DIR).join(filename);
    let lhs = image::open(&path)
        .unwrap_or_else(|e| panic!("can't open expected image '{}': {e}", path.display(),));
    assert_eq_cfg(filename, lhs, seen, cfg)
}

pub fn assert_img_eq_cfg(filename: &str, seen: impl Into<DynamicImage>, cfg: DiffCfg) {
    assert_img_eq_cfg_result(filename, seen, cfg).unwrap()
}

pub fn assert_img_eq(filename: &str, seen: impl Into<DynamicImage>) {
    assert_img_eq_cfg(filename, seen, DiffCfg::default())
}

/// Normalize the depth image to make it easier to see.
///
/// ## Warning
/// This is only normalization, not linearization.
pub fn normalize_gray_img(seen: &mut image::GrayImage) {
    let mut max = 0u8;
    let mut min = u8::MAX;
    seen.pixels().for_each(|Luma([c])| {
        max = max.max(*c);
        min = min.min(*c);
    });
    let total = (max - min) as f32;
    seen.pixels_mut().for_each(|c| {
        let comps = c.0.map(|u| {
            let percent = (u as f32 - min as f32) / total;
            let float = percent * 255.0;
            float as u8
        });
        c.0 = comps;
    });
    log::info!("normalize_gray_img-min: {min}");
    log::info!("normalize_gray_img-max: {max}");
}

#[cfg(test)]
mod test {
    use crate::assert_img_eq;

    #[test]
    fn can_compare_images_sanity() {
        let img = image::open("../../test_img/jolt.png").unwrap();
        assert_img_eq("jolt.png", img);
    }
}

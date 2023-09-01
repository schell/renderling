//! Provides image diffing for testing.
use glam::{Vec4, Vec4Swizzles};
use image::{DynamicImage, Rgba, Rgba32FImage};
use snafu::prelude::*;
use std::path::Path;

const TEST_IMG_DIR: &str = "../../test_img";
const TEST_OUTPUT_DIR: &str = "../../test_output";

#[derive(Debug, Snafu)]
enum ImgDiffError {
    #[snafu(display("Images are different sizes"))]
    ImageSize,
}

fn get_results(
    left_image: &Rgba32FImage,
    right_image: &Rgba32FImage,
    threshold: f32,
) -> Result<Option<(usize, Rgba32FImage)>, ImgDiffError> {
    let lid @ (width, height) = left_image.dimensions();
    let rid = left_image.dimensions();
    snafu::ensure!(lid == rid, ImageSizeSnafu);

    let results = left_image
        .enumerate_pixels()
        .flat_map(|(x, y, left_pixel)| {
            let right_pixel = right_image.get_pixel(x, y);
            if left_pixel == right_pixel {
                None
            } else {
                let left_pixel = Vec4::from(left_pixel.0);
                let right_pixel = Vec4::from(right_pixel.0);
                let delta = (left_pixel - right_pixel).abs();
                if delta.length() > threshold {
                    Some((x, y, delta))
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    let diffs: usize = results.len();
    if diffs == 0 {
        Ok(None)
    } else {
        let mut output_image =
            image::ImageBuffer::from_pixel(width, height, Rgba([0.0, 0.0, 0.0, 0.0]));
        for (x, y, delta) in results {
            let color = delta.xyz().extend(1.0);
            output_image.put_pixel(x, y, Rgba(color.into()));
        }
        Ok(Some((diffs, output_image)))
    }
}

pub fn save(filename: &str, seen: impl Into<DynamicImage>) {
    let path = Path::new(TEST_OUTPUT_DIR).join(filename);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    seen.into().save(path).unwrap();
}

pub fn assert_eq(filename: &str, lhs: impl Into<DynamicImage>, rhs: impl Into<DynamicImage>) {
    let lhs = lhs.into();
    let lhs = lhs.into_rgba32f();
    let rhs = rhs.into().into_rgba32f();
    if let Some((diffs, diff_image)) = get_results(&lhs, &rhs, 0.5).unwrap() {
        let mut dir = Path::new(TEST_OUTPUT_DIR).join(filename);
        dir.set_extension("");
        std::fs::create_dir_all(&dir).expect("cannot create test output dir");
        let expected = dir.join("expected.png");
        let seen = dir.join("seen.png");
        let diff = dir.join("diff.png");
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
        panic!(
            "{} has >= {} differences above the threshold\nexpected: {}\nseen: {}\ndiff: {}",
            filename,
            diffs,
            expected.display(),
            seen.display(),
            diff.display()
        );
    }
}

pub fn assert_img_eq(filename: &str, seen: impl Into<DynamicImage>) {
    let cwd = std::env::current_dir().expect("no cwd");
    let lhs = image::open(Path::new(TEST_IMG_DIR).join(filename)).unwrap_or_else(|_| {
        panic!(
            "can't open expected image '{}'",
            cwd.join(filename).display()
        )
    });
    assert_eq(filename, lhs, seen)
}

//! Provides image diffing for testing.
use std::path::Path;

use snafu::prelude::*;

const TEST_IMG_DIR: &'static str = "../../test_img";
const TEST_OUTPUT_DIR: &'static str = "../../test_output";

#[derive(Clone, Copy)]
pub enum Save {
    Yes,
    No,
}

pub fn save(seen: image::RgbaImage, filename: &'static str) {
    std::fs::create_dir_all(TEST_OUTPUT_DIR).unwrap();
    seen.save(Path::new(TEST_OUTPUT_DIR).join(filename)).unwrap();
}

pub fn assert_img_eq_save(
    // whether this should be saved to the test_img dir
    should_save: Save,
    test_name: &'static str,
    filename: &'static str,
    seen: image::RgbaImage,
) -> Result<(), snafu::Whatever> {
    if matches!(Save::Yes, should_save) {
        seen.save_with_format(
            Path::new(TEST_IMG_DIR).join(filename),
            image::ImageFormat::Png,
        )
        .unwrap();
    }
    assert_img_eq(test_name, filename, seen)
}

pub fn assert_img_eq(
    test_name: &'static str,
    filename: &'static str,
    seen: image::RgbaImage,
) -> Result<(), snafu::Whatever> {
    let cwd = std::env::current_dir().whatever_context("no cwd")?;
    let left_image = image::open(Path::new(TEST_IMG_DIR).join(filename))
        .whatever_context(format!(
            "can't open expected image '{}'",
            cwd.join(filename).display()
        ))?
        .to_rgba8();
    let right_image = seen;
    if let Some((diffs, diff_image)) = dify::diff::get_results(
        left_image.clone(),
        right_image.clone(),
        0.1,
        false,
        None,
        &None,
        &None,
    ) {
        let dir = Path::new(TEST_OUTPUT_DIR).join(test_name);
        std::fs::create_dir_all(&dir).whatever_context("cannot create test output dir")?;
        let expected = dir.join("expected.jpg");
        let seen = dir.join("seen.jpg");
        let diff = dir.join("diff.jpg");
        left_image
            .save_with_format(&expected, image::ImageFormat::Jpeg)
            .whatever_context("can't save expected")?;
        right_image
            .save_with_format(&seen, image::ImageFormat::Jpeg)
            .whatever_context("can't save seen")?;
        diff_image
            .save_with_format(&diff, image::ImageFormat::Jpeg)
            .whatever_context("can't save diff")?;
        panic!(
            "{} has >= {} differences above the threshold\nexpected: {}\nseen: {}\ndiff: {}",
            test_name,
            diffs,
            expected.display(),
            seen.display(),
            diff.display()
        );
    }
    Ok(())
}

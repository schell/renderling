//! Provides image diffing for testing.
use std::path::Path;

use snafu::prelude::*;

const TEST_OUTPUT_DIR: &'static str = "../../test_output";

pub fn assert_img_eq(
    test_name: &'static str,
    expected_path: &'static str,
    seen: image::RgbaImage,
) -> Result<(), snafu::Whatever> {
    let cwd = std::env::current_dir().whatever_context("no cwd")?;
    let left_image = image::open(expected_path)
        .whatever_context(format!(
            "can't open expected image '{}'",
            cwd.join(expected_path).display()
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

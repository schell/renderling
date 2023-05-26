//! Provides image diffing for testing.
use std::path::Path;

use image::DynamicImage;

const TEST_IMG_DIR: &'static str = "../../test_img";
const TEST_OUTPUT_DIR: &'static str = "../../test_output";

pub fn save(filename: &str, seen: impl Into<DynamicImage>) {
    std::fs::create_dir_all(TEST_OUTPUT_DIR).unwrap();
    seen.into()
        .save(Path::new(TEST_OUTPUT_DIR).join(filename))
        .unwrap();
}

pub fn assert_img_eq(filename: &str, seen: impl Into<DynamicImage>) {
    let testname = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    assert_img_eq_with_testname(testname, filename, seen)
}

pub fn assert_img_eq_with_testname(testname: &str, filename: &str, seen: impl Into<DynamicImage>) {
    let cwd = std::env::current_dir().expect("no cwd");
    let left_image = image::open(Path::new(TEST_IMG_DIR).join(filename))
        .expect(&format!(
            "can't open expected image '{}'",
            cwd.join(filename).display()
        ))
        .to_rgba8();
    let dyn_image: DynamicImage = seen.into();
    let right_image = dyn_image.into_rgba8();
    if let Some((diffs, diff_image)) = dify::diff::get_results(
        left_image.clone(),
        right_image.clone(),
        // this threshold is how different to the pixels are based on first
        // converting each to Yiq (luminance, hue and saturation) and then adding
        // a * Y^2 + b * i^2 + c * q^2
        11.0,
        false,
        None,
        &None,
        &None,
    ) {
        let dir = Path::new(TEST_OUTPUT_DIR).join(testname);
        std::fs::create_dir_all(&dir).expect("cannot create test output dir");
        let expected = dir.join("expected.png");
        let seen = dir.join("seen.png");
        let diff = dir.join("diff.png");
        left_image
            .save_with_format(&expected, image::ImageFormat::Png)
            .expect("can't save expected");
        right_image
            .save_with_format(&seen, image::ImageFormat::Png)
            .expect("can't save seen");
        diff_image
            .save_with_format(&diff, image::ImageFormat::Png)
            .expect("can't save diff");
        panic!(
            "{} has >= {} differences above the threshold\nexpected: {}\nseen: {}\ndiff: {}",
            testname,
            diffs,
            expected.display(),
            seen.display(),
            diff.display()
        );
    }
}

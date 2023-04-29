//! Provides image diffing for testing.
use std::path::Path;

const TEST_IMG_DIR: &'static str = "../../test_img";
const TEST_OUTPUT_DIR: &'static str = "../../test_output";

#[allow(dead_code)]
pub fn save(filename: &'static str, seen: image::RgbaImage) {
    std::fs::create_dir_all(TEST_OUTPUT_DIR).unwrap();
    seen.save(Path::new(TEST_OUTPUT_DIR).join(filename))
        .unwrap();
}

pub fn assert_img_eq(filename: &'static str, seen: image::RgbaImage) {
    let testname = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    assert_img_eq_with_testname(testname, filename, seen)
}

pub fn assert_img_eq_with_testname(testname: &str, filename: &'static str, seen: image::RgbaImage) {
    let cwd = std::env::current_dir().expect("no cwd");
    let left_image = image::open(Path::new(TEST_IMG_DIR).join(filename))
        .expect(&format!(
            "can't open expected image '{}'",
            cwd.join(filename).display()
        ))
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
        let dir = Path::new(TEST_OUTPUT_DIR).join(testname);
        std::fs::create_dir_all(&dir).expect("cannot create test output dir");
        let expected = dir.join("expected.jpg");
        let seen = dir.join("seen.jpg");
        let diff = dir.join("diff.jpg");
        left_image
            .save_with_format(&expected, image::ImageFormat::Jpeg)
            .expect("can't save expected");
        right_image
            .save_with_format(&seen, image::ImageFormat::Jpeg)
            .expect("can't save seen");
        diff_image
            .save_with_format(&diff, image::ImageFormat::Jpeg)
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

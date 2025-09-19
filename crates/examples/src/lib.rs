//! # Examples for the manual
//!
//! This crate contains examples and snippets that get pulled into the manual
//! via mdbook links. It also contains tests.

#[cfg(test)]
mod context;

#[cfg(test)]
mod stage;

#[cfg(test)]
mod gltf;

#[cfg(test)]
mod skybox;

pub fn cwd_to_manual_assets_dir() -> std::path::PathBuf {
    let current_dir =
        std::path::PathBuf::from(std::env!("CARGO_WORKSPACE_DIR")).join("manual/src/assets");
    let current_dir = current_dir.canonicalize().unwrap();
    std::env::set_current_dir(&current_dir).unwrap();
    println!("current dir: {:?}", std::env::current_dir());
    current_dir
}

pub fn workspace_dir() -> std::path::PathBuf {
    renderling_build::workspace_dir().canonicalize().unwrap()
}

pub fn test_output_dir() -> std::path::PathBuf {
    renderling_build::test_output_dir().canonicalize().unwrap()
}

pub fn cwd_to_cargo_workspace() -> std::path::PathBuf {
    let current_dir = workspace_dir();
    std::env::set_current_dir(&current_dir).unwrap();
    println!("current dir: {:?}", std::env::current_dir());
    current_dir
}

doc_comment::doctest!("../../../manual/src/stage.md", stage_md);

#[test]
fn can_test() {
    assert_eq!(1, 1);
}

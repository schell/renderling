//! Xtask dependency helpers.
//!
//! This module helps installing xtask's required dependencies.

pub async fn has_binary(name: impl AsRef<str>) -> bool {
    let output = tokio::process::Command::new("hash")
        .arg(name.as_ref())
        .output()
        .await
        .expect("Failed to execute process");

    output.status.success()
}

pub async fn cargo_install(name: impl AsRef<str>) {
    log::warn!("installing mdbook");

    let mut process = tokio::process::Command::new("cargo")
        .args(["install", name.as_ref()])
        .spawn()
        .unwrap();
    let status = process.wait().await.unwrap();
    if !status.success() {
        panic!("Failed installing {}", name.as_ref());
    }
}

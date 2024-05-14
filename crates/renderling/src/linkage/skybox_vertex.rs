#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [skybox::skybox_vertex](crate::skybox::skybox_vertex).
//!
//! **source path**: `crates/renderling/src/linkage/skybox-skybox_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "skybox::skybox_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "skyboxskybox_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("skybox-skybox_vertex.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}


//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [skybox::skybox_cubemap_fragment](crate::skybox::skybox_cubemap_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/skybox-skybox_cubemap_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "skybox::skybox_cubemap_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "skyboxskybox_cubemap_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("skybox-skybox_cubemap_fragment.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

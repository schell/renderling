
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [skybox::skybox_cubemap_vertex](crate::skybox::skybox_cubemap_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/skybox-skybox_cubemap_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "skybox::skybox_cubemap_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "skyboxskybox_cubemap_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("skybox-skybox_cubemap_vertex.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}
pub fn get_from_cache(
    device: &wgpu::Device,
    cache: &mut std::collections::HashMap<&'static str, Arc<ShaderLinkage>>,
) -> Arc<ShaderLinkage> {
    cache
        .entry(ENTRY_POINT)
        .or_insert_with(|| linkage(device).into())
        .clone()
}

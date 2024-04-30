
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::generate_mipmap_vertex](crate::convolution::generate_mipmap_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/convolution-generate_mipmap_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
pub const ENTRY_POINT: &str = "convolution::generate_mipmap_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-generate_mipmap_vertex.spv"
        ))),
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

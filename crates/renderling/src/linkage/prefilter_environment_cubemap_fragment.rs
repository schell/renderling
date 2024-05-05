
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::prefilter_environment_cubemap_fragment](crate::convolution::prefilter_environment_cubemap_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! convolution-prefilter_environment_cubemap_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "convolution::prefilter_environment_cubemap_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "convolutionprefilter_environment_cubemap_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-prefilter_environment_cubemap_fragment.spv"
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

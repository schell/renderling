//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::brdf_lut_convolution_vertex](crate::convolution::brdf_lut_convolution_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/convolution-brdf_lut_convolution_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
pub const ENTRY_POINT: &str = "convolution::brdf_lut_convolution_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-brdf_lut_convolution_vertex.spv"
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

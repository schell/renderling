#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::prefilter_environment_cubemap_vertex](crate::convolution::prefilter_environment_cubemap_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! convolution-prefilter_environment_cubemap_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "convolution::prefilter_environment_cubemap_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "convolutionprefilter_environment_cubemap_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-prefilter_environment_cubemap_vertex.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

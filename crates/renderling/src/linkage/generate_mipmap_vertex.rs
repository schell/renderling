
#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::generate_mipmap_vertex](crate::convolution::generate_mipmap_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/convolution-generate_mipmap_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "convolution::generate_mipmap_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "convolutiongenerate_mipmap_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-generate_mipmap_vertex.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

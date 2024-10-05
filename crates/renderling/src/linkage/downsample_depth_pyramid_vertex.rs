#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::downsample_depth_pyramid_vertex](crate::cull::downsample_depth_pyramid_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/cull-downsample_depth_pyramid_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::downsample_depth_pyramid_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "culldownsample_depth_pyramid_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "cull-downsample_depth_pyramid_vertex.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

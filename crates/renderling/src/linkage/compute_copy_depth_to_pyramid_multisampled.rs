#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::compute_copy_depth_to_pyramid_multisampled](crate::cull::compute_copy_depth_to_pyramid_multisampled).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! cull-compute_copy_depth_to_pyramid_multisampled.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::compute_copy_depth_to_pyramid_multisampled";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "cullcompute_copy_depth_to_pyramid_multisampled";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "cull-compute_copy_depth_to_pyramid_multisampled.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

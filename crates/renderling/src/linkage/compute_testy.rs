#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::compute_testy](crate::cull::compute_testy).
//!
//! **source path**: `crates/renderling/src/linkage/cull-compute_testy.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::compute_testy";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "cullcompute_testy";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("cull-compute_testy.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

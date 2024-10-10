#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::compute_occlusion_culling](crate::cull::compute_occlusion_culling).
//!
//! **source path**:
//! `crates/renderling/src/linkage/cull-compute_occlusion_culling.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::compute_occlusion_culling";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "cullcompute_occlusion_culling";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("cull-compute_occlusion_culling.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

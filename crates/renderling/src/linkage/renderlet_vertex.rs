
#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [stage::renderlet_vertex](crate::stage::renderlet_vertex).
//!
//! **source path**: `crates/renderling/src/linkage/stage-renderlet_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "stage::renderlet_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "stagerenderlet_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("stage-renderlet_vertex.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

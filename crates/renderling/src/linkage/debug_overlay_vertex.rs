#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [debug::debug_overlay_vertex](crate::debug::debug_overlay_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/debug-debug_overlay_vertex.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "debug::debug_overlay_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "debugdebug_overlay_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("debug-debug_overlay_vertex.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

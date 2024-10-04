#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [debug::debug_overlay](crate::debug::debug_overlay).
//!
//! **source path**: `crates/renderling/src/linkage/debug-debug_overlay.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "debug::debug_overlay";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "debugdebug_overlay";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("debug-debug_overlay.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

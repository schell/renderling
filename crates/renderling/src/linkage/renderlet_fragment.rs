#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [stage::renderlet_fragment](crate::stage::renderlet_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/stage-renderlet_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "stage::renderlet_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "stagerenderlet_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("stage-renderlet_fragment.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

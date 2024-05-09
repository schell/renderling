
#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tonemapping::tonemapping_fragment](crate::tonemapping::tonemapping_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tonemapping-tonemapping_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tonemapping::tonemapping_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tonemappingtonemapping_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device
                .create_shader_module(wgpu::include_spirv!("tonemapping-tonemapping_fragment.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

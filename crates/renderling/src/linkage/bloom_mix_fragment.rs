#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [bloom::bloom_mix_fragment](crate::bloom::bloom_mix_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/bloom-bloom_mix_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "bloom::bloom_mix_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "bloombloom_mix_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("bloom-bloom_mix_fragment.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

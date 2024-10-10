#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::test_copy_depth](crate::cull::test_copy_depth).
//!
//! **source path**: `crates/renderling/src/linkage/cull-test_copy_depth.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::test_copy_depth";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "culltest_copy_depth";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device.create_shader_module(wgpu::include_spirv!("cull-test_copy_depth.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

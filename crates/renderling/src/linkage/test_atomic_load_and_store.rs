#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [stage::test_atomic_load_and_store](crate::stage::test_atomic_load_and_store).
//!
//! **source path**:
//! `crates/renderling/src/linkage/stage-test_atomic_load_and_store.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "stage::test_atomic_load_and_store";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "stagetest_atomic_load_and_store";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(
            device
                .create_shader_module(wgpu::include_spirv!("stage-test_atomic_load_and_store.spv")),
        ),
        entry_point: ENTRY_POINT,
    }
}

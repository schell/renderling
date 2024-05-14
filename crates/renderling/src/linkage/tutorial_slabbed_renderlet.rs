#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_slabbed_renderlet](crate::tutorial::tutorial_slabbed_renderlet).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_slabbed_renderlet.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tutorial::tutorial_slabbed_renderlet";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tutorialtutorial_slabbed_renderlet";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_slabbed_renderlet.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

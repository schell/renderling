#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_slabbed_vertices_no_instance](crate::tutorial::tutorial_slabbed_vertices_no_instance).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! tutorial-tutorial_slabbed_vertices_no_instance.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tutorial::tutorial_slabbed_vertices_no_instance";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tutorialtutorial_slabbed_vertices_no_instance";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_slabbed_vertices_no_instance.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

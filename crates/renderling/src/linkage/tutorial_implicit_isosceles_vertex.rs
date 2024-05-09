
#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_implicit_isosceles_vertex](crate::tutorial::tutorial_implicit_isosceles_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_implicit_isosceles_vertex.
//! spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tutorial::tutorial_implicit_isosceles_vertex";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tutorialtutorial_implicit_isosceles_vertex";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_implicit_isosceles_vertex.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

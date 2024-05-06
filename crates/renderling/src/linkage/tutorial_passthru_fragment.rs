
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_passthru_fragment](crate::tutorial::tutorial_passthru_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_passthru_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tutorial::tutorial_passthru_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tutorialtutorial_passthru_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_passthru_fragment.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

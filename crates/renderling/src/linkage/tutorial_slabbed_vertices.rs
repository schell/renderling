
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_slabbed_vertices](crate::tutorial::tutorial_slabbed_vertices).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_slabbed_vertices.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "tutorial::tutorial_slabbed_vertices";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "tutorialtutorial_slabbed_vertices";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_slabbed_vertices.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}
pub fn get_from_cache(
    device: &wgpu::Device,
    cache: &mut std::collections::HashMap<&'static str, Arc<ShaderLinkage>>,
) -> Arc<ShaderLinkage> {
    cache
        .entry(ENTRY_POINT)
        .or_insert_with(|| linkage(device).into())
        .clone()
}

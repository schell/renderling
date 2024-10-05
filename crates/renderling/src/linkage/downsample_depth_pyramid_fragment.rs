#![allow(dead_code)]
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [cull::downsample_depth_pyramid_fragment](crate::cull::downsample_depth_pyramid_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/cull-downsample_depth_pyramid_fragment.spv`
use super::ShaderLinkage;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub const ENTRY_POINT: &str = "cull::downsample_depth_pyramid_fragment";
#[cfg(target_arch = "wasm32")]
pub const ENTRY_POINT: &str = "culldownsample_depth_pyramid_fragment";
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "cull-downsample_depth_pyramid_fragment.spv"
        ))),
        entry_point: ENTRY_POINT,
    }
}

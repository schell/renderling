#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
#[cfg(not(target_arch = "wasm32"))]
mod target {
    pub const ENTRY_POINT: &str = "cull::compute_copy_depth_to_pyramid";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/cull-compute_copy_depth_to_pyramid.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!(
            "creating native linkage for {}",
            "compute_copy_depth_to_pyramid"
        );
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod target {
    pub const ENTRY_POINT: &str = "cullcompute_copy_depth_to_pyramid";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_wgsl!("../../shaders/cull-compute_copy_depth_to_pyramid.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!(
            "creating web linkage for {}",
            "compute_copy_depth_to_pyramid"
        );
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    target::linkage(device)
}

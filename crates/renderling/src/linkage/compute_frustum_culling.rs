#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
#[cfg(not(target_arch = "wasm32"))]
mod target {
    pub const ENTRY_POINT: &str = "cull::compute_frustum_culling";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/cull-compute_frustum_culling.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!("creating native linkage for {}", "compute_frustum_culling");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod target {
    pub const ENTRY_POINT: &str = "cullcompute_frustum_culling";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_wgsl!("../../shaders/cull-compute_frustum_culling.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!("creating web linkage for {}", "compute_frustum_culling");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    target::linkage(device)
}

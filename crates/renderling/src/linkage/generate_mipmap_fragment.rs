#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
#[cfg(not(target_arch = "wasm32"))]
mod target {
    pub const ENTRY_POINT: &str = "convolution::generate_mipmap_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/convolution-generate_mipmap_fragment.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!("creating native linkage for {}", "generate_mipmap_fragment");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod target {
    pub const ENTRY_POINT: &str = "convolutiongenerate_mipmap_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_wgsl!("../../shaders/convolution-generate_mipmap_fragment.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!("creating web linkage for {}", "generate_mipmap_fragment");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    target::linkage(device)
}

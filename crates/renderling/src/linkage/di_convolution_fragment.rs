#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
#[cfg(not(target_arch = "wasm32"))]
mod target {
    pub const ENTRY_POINT: &str = "ibl::diffuse_irradiance::di_convolution_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/ibl-diffuse_irradiance-di_convolution_fragment.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::debug!("creating native linkage for {}", "di_convolution_fragment");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod target {
    pub const ENTRY_POINT: &str = "ibldiffuse_irradiancedi_convolution_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_wgsl!("../../shaders/ibl-diffuse_irradiance-di_convolution_fragment.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::debug!("creating web linkage for {}", "di_convolution_fragment");
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    target::linkage(device)
}

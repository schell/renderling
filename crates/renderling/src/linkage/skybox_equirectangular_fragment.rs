#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
#[cfg(not(target_arch = "wasm32"))]
mod target {
    pub const ENTRY_POINT: &str = "skybox::skybox_equirectangular_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/skybox-skybox_equirectangular_fragment.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!(
            "creating native linkage for {}",
            "skybox_equirectangular_fragment"
        );
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
#[cfg(target_arch = "wasm32")]
mod target {
    pub const ENTRY_POINT: &str = "skyboxskybox_equirectangular_fragment";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_wgsl!("../../shaders/skybox-skybox_equirectangular_fragment.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        log::info!(
            "creating web linkage for {}",
            "skybox_equirectangular_fragment"
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

#![allow(dead_code)]
//! Automatically generated by Renderling's `build.rs`.
use crate::linkage::ShaderLinkage;
mod native {
    pub const ENTRY_POINT: &str = "tutorial::tutorial_slabbed_vertices";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/tutorial-tutorial_slabbed_vertices.spv")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
mod web {
    pub const ENTRY_POINT: &str = "tutorialtutorial_slabbed_vertices";
    pub fn descriptor() -> wgpu::ShaderModuleDescriptor<'static> {
        wgpu::include_spirv!("../../shaders/tutorial-tutorial_slabbed_vertices.wgsl")
    }
    pub fn linkage(device: &wgpu::Device) -> super::ShaderLinkage {
        super::ShaderLinkage {
            entry_point: ENTRY_POINT,
            module: device.create_shader_module(descriptor()).into(),
        }
    }
}
pub fn linkage_native(device: &wgpu::Device) -> ShaderLinkage {
    native::linkage(device)
}
pub fn linkage_web(device: &wgpu::Device) -> ShaderLinkage {
    web::linkage(device)
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    if cfg!(feature = "wasm") {
        web::linkage(device)
    } else {
        native::linkage(device)
    }
}

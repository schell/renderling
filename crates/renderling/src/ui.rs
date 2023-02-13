//! Ui pipeline and material definitions.
use crate::{AnyMaterialUniform, Material, MaterialUniform};

#[cfg(feature = "text")]
mod text;
pub use renderling_shader::ui::UiColorBlend;
#[cfg(feature = "text")]
pub use text::*;

pub type UiVertex = crate::linkage::ui::Vertex;

pub struct UiMaterialUniform {
    bindgroup: wgpu::BindGroup,
}

impl MaterialUniform for UiMaterialUniform {
    fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bindgroup
    }
}

/// A material for ui meshes.
#[derive(Debug)]
pub struct UiMaterial {
    pub diffuse_texture: crate::texture::Texture,
    pub color_blend: UiColorBlend,
}

impl Material for UiMaterial {
    fn create_material_uniform(&self, device: &wgpu::Device) -> AnyMaterialUniform {
        AnyMaterialUniform::new(UiMaterialUniform {
            bindgroup: crate::linkage::ui::create_ui_material_bindgroup(device, &self),
        })
    }
}

/// A pipeline for UI.
#[derive(Debug)]
pub struct UiPipeline {
    inner: wgpu::RenderPipeline,
}

impl crate::Pipeline for UiPipeline {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.inner
    }
}

impl UiPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        UiPipeline {
            inner: crate::linkage::ui::create_pipeline(device, format),
        }
    }
}

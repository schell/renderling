//! Ui pipeline and material definitions.
use crate::{AnyMaterialUniform, Material, MaterialUniform};

#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text")]
pub use text::*;


pub type UiVertex = renderling_ui::Vertex;

/// Variants of uv/color blending.
///
/// This determines how UV and Color coords are blended
/// together.
#[derive(Debug, Copy, Clone)]
pub enum UiColorBlend {
    /// The mesh should be colored only with its color attribute
    ColorOnly = 0,
    /// The mesh should be colored only with its uv vertex attribute
    UvOnly = 1,
    /// The mesh should replace uv red with its color vertex attribute.
    ///
    /// This is used for colored text.
    ReplaceRedUvWithColor = 2,
}

#[cfg(test)]
mod ui {
    #[test]
    fn ui_color() {
        assert!(super::UiColorBlend::ReplaceRedUvWithColor as u32 == 2);
    }
}

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
            bindgroup: renderling_ui::create_ui_material_bindgroup(
                device,
                self.color_blend as u32,
                &self.diffuse_texture.view,
                &self.diffuse_texture.sampler,
            ),
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
            inner: renderling_ui::create_pipeline(device, format),
        }
    }
}

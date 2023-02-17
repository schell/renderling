//! Forward pipeline and blinn-phong material definitions
use crate::{AnyMaterialUniform, MaterialUniform};

pub type ForwardVertex = crate::linkage::pbr::Vertex;

impl MaterialUniform for crate::linkage::pbr::MaterialUniform {
    fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bindgroup
    }
}

/// A blinn-phong material for forward meshes.
#[derive(Debug)]
pub struct BlinnPhongMaterial {
    pub diffuse_texture: crate::texture::Texture,
    pub specular_texture: crate::texture::Texture,
    pub shininess: f32,
}

impl crate::Material for BlinnPhongMaterial {
    fn create_material_uniform(&self, device: &wgpu::Device) -> crate::AnyMaterialUniform {
        let material_uniform = crate::linkage::pbr::MaterialUniform::new(device, &self);
        AnyMaterialUniform::new(material_uniform)
    }
}

pub struct ForwardPipeline {
    inner: wgpu::RenderPipeline,
}

impl crate::Pipeline for ForwardPipeline {
    fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.inner
    }
}

impl ForwardPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        ForwardPipeline {
            inner: crate::linkage::pbr::create_pipeline(device, format),
        }
    }
}

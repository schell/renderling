//! Forward pipeline and blinn-phong material definitions
use glam::Vec4;

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

fn to_color_bytes(color: Vec4) -> [u8; 4] {
    let [r, g, b, a] = color.to_array();
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}

impl BlinnPhongMaterial {
    pub fn from_colors(
        gpu: &crate::WgpuState,
        diffuse: glam::Vec4,
        specular: glam::Vec4,
        shininess: f32,
    ) -> Self {
        let diffuse_texture = crate::Texture::new(
            &gpu.device,
            &gpu.queue,
            None,
            None,
            4,
            1,
            1,
            &to_color_bytes(diffuse),
        );
        let specular_texture = crate::Texture::new(
            &gpu.device,
            &gpu.queue,
            None,
            None,
            4,
            1,
            1,
            &to_color_bytes(specular),
        );
        BlinnPhongMaterial {
            diffuse_texture,
            specular_texture,
            shininess,
        }
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
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        primitive: Option<wgpu::PrimitiveState>,
    ) -> Self {
        ForwardPipeline {
            inner: crate::linkage::pbr::create_pipeline(device, format, primitive),
        }
    }
}

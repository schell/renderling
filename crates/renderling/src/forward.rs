//! Forward pipeline and blinn-phong material definitions
use crate::{AnyMaterialUniform, MaterialUniform};

impl MaterialUniform for renderling_forward::MaterialUniform {
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
        let material_uniform = renderling_forward::MaterialUniform::new(
            device,
            &self.diffuse_texture.view,
            &self.diffuse_texture.sampler,
            &self.specular_texture.view,
            &self.specular_texture.sampler,
            self.shininess,
        );
        AnyMaterialUniform::new(material_uniform)
    }
}

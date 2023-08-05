//! Material builders.

use glam::Vec4;
use renderling_shader::{pbr::PbrMaterial, scene::{GpuTexture, LightingModel}};

use crate::{Id, SceneBuilder};

pub struct UnlitMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: PbrMaterial,
}

impl<'a> UnlitMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let material = PbrMaterial::default();
        Self { builder, material }
    }

    pub fn with_base_color(mut self, color: impl Into<Vec4>) -> Self {
        self.material.factor0 = color.into();
        self
    }

    pub fn with_texture0(mut self, texture_id: Id<GpuTexture>) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_texture2(mut self, texture_id: Id<GpuTexture>) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> Id<PbrMaterial> {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        Id::new(id as u32)
    }
}

pub struct PbrMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: PbrMaterial,
}

impl<'a> PbrMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let mut material = PbrMaterial::default();
        material.lighting_model = LightingModel::PBR_LIGHTING;
        Self { builder, material }
    }

    pub fn with_base_color_factor(mut self, factor: impl Into<Vec4>) -> Self {
        self.material.factor0 = factor.into();
        self
    }

    pub fn with_base_color_texture(mut self, texture_id: Id<GpuTexture>) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_base_color_texture_coord(mut self, tex_coord: u32) -> Self {
        self.material.texture0_tex_coord = tex_coord;
        self
    }

    pub fn with_metallic_factor(mut self, metallic: f32) -> Self {
        self.material.factor1.y = metallic;
        self
    }

    pub fn with_roughness_factor(mut self, roughness: f32) -> Self {
        self.material.factor1.z = roughness;
        self
    }

    pub fn with_metallic_roughness_texture(mut self, texture_id: Id<GpuTexture>) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn with_metallic_roughness_texture_coord(mut self, tex_coord: u32) -> Self {
        self.material.texture1_tex_coord = tex_coord;
        self
    }

    pub fn build(self) -> Id<PbrMaterial> {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        Id::new(id as u32)
    }
}

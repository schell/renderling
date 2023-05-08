//! Material builders.

use glam::Vec4;
use renderling_shader::scene::{GpuMaterial, LightingModel};

use crate::SceneBuilder;

pub struct PhongMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> PhongMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let mut material = GpuMaterial::default();
        material.lighting_model = LightingModel::PHONG_LIGHTING;
        Self { builder, material }
    }

    pub fn with_diffuse_texture(mut self, texture_id: u32) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_specular_texture(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

pub struct UnlitMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> UnlitMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let material = GpuMaterial::default();
        Self { builder, material }
    }

    pub fn with_base_color(mut self, color: impl Into<Vec4>) -> Self {
        self.material.factor0 = color.into();
        self
    }

    pub fn with_texture0(mut self, texture_id: u32) -> Self {
        self.material.texture0 = texture_id;
        self
    }

    pub fn with_texture2(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

pub struct PbrMaterialBuilder<'a> {
    builder: &'a mut SceneBuilder,
    material: GpuMaterial,
}

impl<'a> PbrMaterialBuilder<'a> {
    pub fn new(builder: &'a mut SceneBuilder) -> Self {
        let mut material = GpuMaterial::default();
        material.lighting_model = LightingModel::PBR_LIGHTING;
        Self { builder, material }
    }

    pub fn with_base_color_factor(mut self, factor: impl Into<Vec4>) -> Self {
        self.material.factor0 = factor.into();
        self
    }

    pub fn with_base_color_texture(mut self, texture_id: u32) -> Self {
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

    pub fn with_metallic_roughness_texture(mut self, texture_id: u32) -> Self {
        self.material.texture1 = texture_id;
        self
    }

    pub fn with_metallic_roughness_texture_coord(mut self, tex_coord: u32) -> Self {
        self.material.texture1_tex_coord = tex_coord;
        self
    }

    pub fn build(self) -> u32 {
        let id = self.builder.materials.len();
        self.builder.materials.push(self.material);
        id as u32
    }
}

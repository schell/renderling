//! Atlas images, used for materials. CPU and GPU.
use crabslab::{Id, SlabItem};
use glam::{Vec3, Vec4};

use crate::atlas::AtlasTextureDescriptor;

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

/// Represents a material on the GPU.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, SlabItem, core::fmt::Debug)]
pub struct MaterialDescriptor {
    pub emissive_factor: Vec3,
    pub emissive_strength_multiplier: f32,
    pub albedo_factor: Vec4,
    pub metallic_factor: f32,
    pub roughness_factor: f32,

    pub albedo_texture_id: Id<AtlasTextureDescriptor>,
    pub metallic_roughness_texture_id: Id<AtlasTextureDescriptor>,
    pub normal_texture_id: Id<AtlasTextureDescriptor>,
    pub ao_texture_id: Id<AtlasTextureDescriptor>,
    pub emissive_texture_id: Id<AtlasTextureDescriptor>,

    pub albedo_tex_coord: u32,
    pub metallic_roughness_tex_coord: u32,
    pub normal_tex_coord: u32,
    pub ao_tex_coord: u32,
    pub emissive_tex_coord: u32,

    pub has_lighting: bool,
    pub ao_strength: f32,
}

impl Default for MaterialDescriptor {
    fn default() -> Self {
        Self {
            emissive_factor: Vec3::ZERO,
            emissive_strength_multiplier: 1.0,
            albedo_factor: Vec4::ONE,
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            albedo_texture_id: Id::NONE,
            metallic_roughness_texture_id: Id::NONE,
            normal_texture_id: Id::NONE,
            ao_texture_id: Id::NONE,
            albedo_tex_coord: 0,
            metallic_roughness_tex_coord: 0,
            normal_tex_coord: 0,
            ao_tex_coord: 0,
            has_lighting: true,
            ao_strength: 0.0,
            emissive_texture_id: Id::NONE,
            emissive_tex_coord: 0,
        }
    }
}

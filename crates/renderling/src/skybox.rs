//! Skybox shaders and CPU code.
use crabslab::{Id, Slab};
use glam::{Mat3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[allow(unused_imports)]
use spirv_std::num_traits::Float;

use crate::{
    camera::shader::CameraDescriptor,
    math::{self, IsVector},
};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

const INV_ATAN: Vec2 = Vec2::new(0.1591, core::f32::consts::FRAC_1_PI);

/// Takes a unit direction and converts it to a uv lookup in an equirectangular
/// map.
pub fn direction_to_equirectangular_uv(dir: Vec3) -> Vec2 {
    let mut uv = Vec2::new(f32::atan2(dir.z, dir.x), f32::asin(dir.y));
    uv *= INV_ATAN;
    uv += 0.5;
    uv
}

/// Vertex shader for a skybox.
#[spirv(vertex)]
pub fn skybox_vertex(
    #[spirv(instance_index)] camera_index: u32,
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    local_pos: &mut Vec3,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let camera_id = Id::<CameraDescriptor>::from(camera_index);
    let camera = slab.read(camera_id);
    let point = math::CUBE[vertex_index as usize];
    *local_pos = point;
    let camera_view_without_translation = Mat3::from_mat4(camera.view());
    let rot_view = Mat4::from_mat3(camera_view_without_translation);
    let position = camera.projection() * rot_view * point.extend(1.0);
    *clip_pos = position.xyww();
}

/// Colors a skybox using a cubemap texture.
#[spirv(fragment)]
pub fn skybox_cubemap_fragment(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    let env_color: Vec3 = texture.sample(*sampler, local_pos.alt_norm_or_zero()).xyz();
    *out_color = env_color.extend(1.0);
}

/// Vertex shader that draws a cubemap.
///
/// Uses the `instance_index` as the [`Id`] for a [`CameraDescriptor`].
///
/// Used to create a cubemap from an equirectangular image as well as cubemap
/// convolutions.
#[spirv(vertex)]
pub fn skybox_cubemap_vertex(
    #[spirv(instance_index)] camera_id: Id<CameraDescriptor>,
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    local_pos: &mut Vec3,
    #[spirv(position)] gl_pos: &mut Vec4,
) {
    let camera = slab.read(camera_id);
    let pos = crate::math::CUBE[vertex_index as usize];
    *local_pos = pos;
    *gl_pos = camera.view_projection() * pos.extend(1.0);
}

/// Fragment shader that colors a skybox using an equirectangular texture.
#[spirv(fragment)]
pub fn skybox_equirectangular_fragment(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    let uv = direction_to_equirectangular_uv(local_pos.alt_norm_or_zero());
    let env_color: Vec3 = texture.sample(*sampler, uv).xyz();
    *out_color = env_color.extend(1.0);
}

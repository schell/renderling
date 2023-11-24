//! Skybox shader.

use glam::{Mat3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{
    image::{Cubemap, Image2d},
    spirv, Sampler,
};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{math, stage::GpuConstants, IsVector};

const INV_ATAN: Vec2 = Vec2::new(0.1591, core::f32::consts::FRAC_1_PI);

/// Takes a unit direction and converts it to a uv lookup in an equirectangular
/// map.
pub fn direction_to_equirectangular_uv(dir: Vec3) -> Vec2 {
    let mut uv = Vec2::new(f32::atan2(dir.z, dir.x), f32::asin(dir.y));
    uv *= INV_ATAN;
    uv += 0.5;
    uv
}

#[spirv(vertex)]
pub fn vertex(
    #[spirv(vertex_index)] vertex_id: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &GpuConstants,
    local_pos: &mut Vec3,
    #[spirv(position)] gl_pos: &mut Vec4,
) {
    let point = math::CUBE[vertex_id as usize];
    *local_pos = point;
    let camera_view_without_translation = Mat3::from_mat4(constants.camera_view);
    let rot_view = Mat4::from_mat3(camera_view_without_translation);
    let clip_pos = constants.camera_projection * rot_view * point.extend(1.0);
    *gl_pos = clip_pos.xyww();
}

/// Colors a skybox using a cubemap texture.
#[spirv(fragment)]
pub fn fragment_cubemap(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Cubemap,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    let env_color: Vec3 = texture.sample(*sampler, local_pos.alt_norm_or_zero()).xyz();
    *out_color = env_color.extend(1.0);
}

/// Passes the singular `Vec3` position attribute to the fragment shader unchanged,
/// while transforming `gl_pos` by the camera projection*view;
///
/// Used to create a cubemap from an equirectangular image as well as cubemap convolutions.
#[spirv(vertex)]
pub fn vertex_position_passthru(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] constants: &GpuConstants,
    in_pos: Vec3,
    local_pos: &mut Vec3,
    #[spirv(position)] gl_pos: &mut Vec4,
) {
    *local_pos = in_pos;
    *gl_pos = constants.camera_projection * constants.camera_view * in_pos.extend(1.0);
}

/// Colors a skybox using an equirectangular texture.
#[spirv(fragment)]
pub fn fragment_equirectangular(
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    let uv = direction_to_equirectangular_uv(local_pos.alt_norm_or_zero());
    let env_color: Vec3 = texture.sample(*sampler, uv).xyz();
    *out_color = env_color.extend(1.0);
}

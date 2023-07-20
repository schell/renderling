//! Skybox shader.

use glam::{Mat3, Mat4, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{
    math::{self, UNIT_INDICES},
    scene::GpuConstants,
};

pub fn vertex_old(
    vertex_id: u32,
    constants: &GpuConstants,
    local_pos: &mut Vec3,
    gl_pos: &mut Vec4,
) {
    let index = math::UNIT_INDICES[vertex_id as usize];
    *local_pos = math::UNIT_POINTS[index];
    // let camera_view_without_translation = Mat3::from_mat4(constants.camera_view);
    // let rot_view = Mat4::from_mat3(camera_view_without_translation);
    let clip_pos = (constants.camera_projection * constants.camera_view)
        .transform_point3(*local_pos)
        .extend(1.0);
    //*gl_pos = clip_pos.xyww();
    *gl_pos = clip_pos;
}

const POINTS: [Vec3; 6] = {
    [
        math::UNIT_POINTS[math::UNIT_INDICES[6+0]],
        math::UNIT_POINTS[math::UNIT_INDICES[6+1]],
        math::UNIT_POINTS[math::UNIT_INDICES[6+2]],
        math::UNIT_POINTS[math::UNIT_INDICES[6+3]],
        math::UNIT_POINTS[math::UNIT_INDICES[6+4]],
        math::UNIT_POINTS[math::UNIT_INDICES[6+5]],
    ]
};

pub fn vertex_works(vertex_id: u32, constants: &GpuConstants, local_pos: &mut Vec3, gl_pos: &mut Vec4) {
    let index = vertex_id as usize % 6;
    let point = POINTS[index];
    *local_pos = point;
    *gl_pos = constants.camera_projection * constants.camera_view * point.extend(1.0);
}

pub fn vertex_no_works(vertex_id: u32, constants: &GpuConstants, local_pos: &mut Vec3, gl_pos: &mut Vec4) {
    let index = vertex_id as usize % 6;
    // doesn't work
    let point = math::UNIT_POINTS[math::UNIT_INDICES[6+index]];
    *local_pos = point;
    *gl_pos = constants.camera_projection * constants.camera_view * point.extend(1.0);
}

pub fn vertex(vertex_id: u32, constants: &GpuConstants, local_pos: &mut Vec3, gl_pos: &mut Vec4) {
    vertex_no_works(vertex_id, constants, local_pos, gl_pos)
}

const INV_ATAN: Vec2 = Vec2::new(0.1591, 0.3183);
pub fn sample_spherical_map(pos: Vec3) -> Vec2 {
    let mut uv = Vec2::new(f32::atan2(pos.z, pos.x), f32::asin(pos.y));
    uv *= INV_ATAN;
    uv += 0.5;
    uv
}

pub fn fragment_equirectangular(
    texture: &Image2d,
    sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    // let uv = sample_spherical_map(local_pos.normalize_or_zero());
    let uv = local_pos.xy();
    let mut env_color: Vec3 = texture.sample(*sampler, uv).xyz();
    if env_color.x != 1.0 {
        env_color.x = 1.0;
    }
    if env_color.y != 0.0 {
        env_color.y = 0.0;
    }
    if env_color.z != 0.0 {
        env_color.z = 0.0;
    }
    *out_color = env_color.extend(1.0);
}

//! Skybox shader.

use glam::{Vec3, Vec4, Mat4, Mat3, Vec4Swizzles};
use spirv_std::{image::Image3d, Sampler};

use crate::{scene::GpuConstants, math};

pub fn vertex(
    vertex_id: u32,
    constants: &GpuConstants,
    local_pos: &mut Vec3,
    gl_pos: &mut Vec4,
) {
    let vertex_index = math::UNIT_INDICES[vertex_id as usize];
    *local_pos = math::UNIT_POINTS[vertex_index];
    let camera_view_without_translation = Mat3::from_mat4(constants.camera_view);
    let rot_view = Mat4::from_mat3(camera_view_without_translation);
    let clip_pos = constants.camera_projection * rot_view * local_pos.extend(1.0);
    *gl_pos = clip_pos.xyww();
}

pub fn fragment(
    environment_map: &Image3d,
    sampler: &Sampler,
    local_pos: Vec3,
    out_color: &mut Vec4,
) {
    let env_color = environment_map.sample_by_lod(*sampler, local_pos, 1.2).xyz();
    *out_color = env_color.extend(1.0);
}

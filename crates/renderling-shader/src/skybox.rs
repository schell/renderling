//! Skybox shader.

use glam::{Mat3, Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, Sampler};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::scene::GpuConstants;

const CUBE: [Vec3; 36] = {
    let p0 = Vec3::new(-0.5, 0.5, 0.5);
    let p1 = Vec3::new(-0.5, 0.5, -0.5);
    let p2 = Vec3::new(0.5, 0.5, -0.5);
    let p3 = Vec3::new(0.5, 0.5, 0.5);
    let p4 = Vec3::new(-0.5, -0.5, 0.5);
    let p7 = Vec3::new(-0.5, -0.5, -0.5);
    let p6 = Vec3::new(0.5, -0.5, -0.5);
    let p5 = Vec3::new(0.5, -0.5, 0.5);

    let points = [
        p0, p2, p1, p0, p3, p2, // top
        p0, p4, p3, p4, p5, p3, // front
        p3, p6, p2, p3, p5, p6, // right
        p1, p7, p0, p7, p4, p0, // left
        p4, p6, p5, p4, p7, p6, // bottom
        p2, p7, p1, p2, p6, p7, // back
    ];

    points
};

pub fn vertex(vertex_id: u32, constants: &GpuConstants, local_pos: &mut Vec3, gl_pos: &mut Vec4) {
    let point = CUBE[vertex_id as usize];
    *local_pos = point;
    let camera_view_without_translation = Mat3::from_mat4(constants.camera_view);
    let rot_view = Mat4::from_mat3(camera_view_without_translation);
    let clip_pos = constants.camera_projection * rot_view * point.extend(1.0);
    *gl_pos = clip_pos.xyww();
}

const INV_ATAN: Vec2 = Vec2::new(0.1591, 0.3183);
pub fn sample_spherical_map(pos: Vec3) -> Vec2 {
    let mut uv = Vec2::new(f32::atan2(pos.z, pos.x), f32::asin(-pos.y));
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
    let uv = sample_spherical_map(local_pos.normalize_or_zero());
    let env_color: Vec3 = texture.sample(*sampler, uv).xyz();
    *out_color = env_color.extend(1.0);
}

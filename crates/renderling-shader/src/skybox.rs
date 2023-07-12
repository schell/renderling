//! Skybox shader.

use glam::{Vec3, Vec4, Vec2};

use crate::scene::GpuConstants;

pub fn vertex(
    position: Vec3,
    normal: Vec3,
    tangent: Vec4,
    uv: Vec2,

    constants: &GpuConstants,

    local_pos: &mut Vec3,
    gl_pos: &mut Vec4,
) {

}

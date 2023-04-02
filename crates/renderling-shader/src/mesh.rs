//! An attempt to store an entire scene of mesh data on the GPU.

use glam::{Vec4, Vec2};

#[derive(Default)]
pub struct MeshVertex {
    pub position: Vec4,
    pub color: Vec4,
    pub uv: Vec4,
    pub norm: Vec4,
}

/// A structure to store an entire scene of mesh data on the GPU.
pub struct MeshStore<const MAX_LENGTH: usize> {
    pub positions: [Vec4; MAX_LENGTH],
    pub colors: [Vec4; MAX_LENGTH],
    pub uv: [Vec2; MAX_LENGTH],
    pub norm: [Vec4; MAX_LENGTH],
}

pub struct Mesh {
    pub first_index: u32,
    pub count: u32,
}

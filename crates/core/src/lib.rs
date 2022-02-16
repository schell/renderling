//! Canonical types for render pipelines.
use std::ops::Range;

/// Shader resources for view and projection matrices.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjection {
    pub projection: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
}

/// A wrapped bindgroup that represents a uniform camera.
pub struct Camera<'a> {
    pub bindgroup: &'a wgpu::BindGroup,
}

/// Draw parameters for a renderable object. This roughly corresponds to
/// [`wgpu::RenderPass::draw_indexed`] and [`wgpu::RenderPass::draw`].
#[derive(Clone)]
pub enum ObjectDraw<'a> {
    Indexed {
        index_buffer: wgpu::BufferSlice<'a>,
        index_format: wgpu::IndexFormat,
        index_range: Range<u32>,
        base_vertex: i32,
    },
    Default {
        vertex_range: Range<u32>,
    }
}

//! Canonical types and helpers for authoring render pipelines.
use std::ops::Range;

use wgpu::util::DeviceExt;

pub mod light;

/// Projection and view matrices.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProjection {
    pub projection: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
}

/// A bindgroup that represents a camera uniform.
// TODO: remove Camera<'a>
pub struct Camera<'a> {
    pub bindgroup: &'a wgpu::BindGroup,
}

/// Draw parameters for a renderable object.
///
/// This roughly corresponds to [`wgpu::RenderPass::draw_indexed`] and
/// [`wgpu::RenderPass::draw`].
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
    },
}

/// Perform a clearing render pass.
pub fn conduct_clear_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_view: &wgpu::TextureView,
    depth_view: Option<&wgpu::TextureView>,
    clear_color: wgpu::Color,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("renderling clear pass"),
    });

    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: frame_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(clear_color),
                store: true,
            },
        })],
        depth_stencil_attachment: depth_view.map(|view| wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),
    });

    queue.submit(std::iter::once(encoder.finish()));
}

pub fn camera_uniform_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("proj+view matrix uniform bind group layout"),
    })
}


pub fn create_camera_uniform(
    device: &wgpu::Device,
    viewproj: ViewProjection,
    label: &str,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&label),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        contents: bytemuck::cast_slice(&[viewproj]),
    });

    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_uniform_layout(device),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some(label),
    });
    (buffer, bindgroup)
}

/// A uniform buffer living in the GPU.
pub struct UniformBuffer {
    pub buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,
}

/// A vertex buffer living in the GPU.
pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
    pub len: usize,
}

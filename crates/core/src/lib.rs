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

/// A renderable object.
///
/// Bundles together buffers, ranges, material and draw instructions.
pub struct ShaderObject<'a> {
    pub mesh_buffer: wgpu::BufferSlice<'a>,
    pub instances: wgpu::BufferSlice<'a>,
    pub instances_range: Range<u32>,
    pub material: Option<&'a wgpu::BindGroup>,
    pub name: Option<&'a str>,
    pub draw: ObjectDraw<'a>,
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

/// Begin a new rendering pass.
pub fn begin_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    label: Option<&'a str>,
    pipeline: &'a wgpu::RenderPipeline,
    frame_texture_view: &'a wgpu::TextureView,
    depth_texture_view: &'a wgpu::TextureView,
) -> wgpu::RenderPass<'a> {
    // start the render pass
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            }),
            stencil_ops: None,
        }),
    });
    render_pass.set_pipeline(pipeline);

    render_pass
}

/// Perform a clearing render pass on a frame and/or a depth texture.
pub fn conduct_clear_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: Option<&str>,
    frame_view: Option<&wgpu::TextureView>,
    depth_view: Option<&wgpu::TextureView>,
    clear_color: wgpu::Color,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("renderling clear pass"),
    });

    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[frame_view.map(|view| wgpu::RenderPassColorAttachment {
            view,
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

pub fn render_object<'a, 'b: 'a>(
    render_pass: &'a mut wgpu::RenderPass<'b>,
    object: ShaderObject<'b>,
    default_material_bindgroup: &'b wgpu::BindGroup,
) {
    // bind the material using the default for any non-textured meshes
    render_pass.set_bind_group(
        1,
        object.material.unwrap_or(default_material_bindgroup),
        &[],
    );
    render_pass.set_vertex_buffer(0, object.mesh_buffer);
    render_pass.set_vertex_buffer(1, object.instances);
    // draw
    match &object.draw {
        ObjectDraw::Indexed {
            index_buffer,
            index_range,
            base_vertex,
            index_format,
        } => {
            render_pass.set_index_buffer(*index_buffer, *index_format);
            render_pass.draw_indexed(
                index_range.clone(),
                *base_vertex,
                object.instances_range.clone(),
            );
        }
        ObjectDraw::Default { vertex_range } => {
            render_pass.draw(vertex_range.clone(), object.instances_range.clone());
        }
    }
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

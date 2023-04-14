//! Links `wgpu` machinery to shaders.
//!
//! This module holds the various SPIR-V shaders used for rendering, and the
//! `wgpu` code that instantiates their pipelines. The .spv files are generated
//! by a program in the main renderling repo.
//!
//! To run, from the repo's project directory use:
//! `cd shaders && cargo run --release`
//!
//! To add a new shader:
//! * add a new module to renedrling-shader with your vertex and fragment
//!   functions
//! * add a new shader crate to shaders/ with a wrapper for your shader code
//!   from step one
//! * add the shader crate name to the SHADERS const in shaders/build.rs
//! * add a tuple of the stem name of the spv file and the shader source (via
//!   env var - don't worry, you'll see) to the SHADERS const in
//!   shaders/src/main.rs
//! * run `cd shaders && cargo run --release`
//! * add whatever linkage you may need on the wgpu side to
//!   renderling/src/linkage

use std::ops::Range;

use encase::UniformBuffer;
use renderling_shader::CameraRaw;
use wgpu::util::DeviceExt;
#[cfg(feature = "ui")]
pub mod ui;

#[cfg(feature = "pbr")]
pub mod pbr;

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

pub fn create_camera_uniform(
    device: &wgpu::Device,
    camera: &CameraRaw,
    label: &str,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    let mut data = UniformBuffer::new(vec![]);
    data.write(camera).unwrap();

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&label),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        contents: data.into_inner().as_slice(),
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

/// A vertex buffer living in the GPU.
pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
    pub len: usize,
}

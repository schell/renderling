//! A renderling for user interfaces.
use std::ops::Range;
use wgpu::{util::DeviceExt, TextureFormat};

pub use renderling_core::{ObjectDraw, ViewProjection, Camera};

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = [x, y, z];
        self
    }

    pub fn with_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r, g, b, a];
        self
    }

    pub fn with_uv(mut self, u: f32, v: f32) -> Self {
        self.uv = [u, v];
        self
    }
}

pub fn create_camera_buffer_bindgroup(
    device: &wgpu::Device,
    viewproj: ViewProjection,
    name: &str,
) -> (wgpu::Buffer, wgpu::BindGroup) {
    let label = format!("ui camera {} uniform buffer", name);
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(&label),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        contents: bytemuck::cast_slice(&[viewproj]),
    });

    let label = format!("ui camera {} bindgroup", name);
    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_uniform_bindgroup_layout(device),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some(&label),
    });
    (buffer, bindgroup)
}

pub fn object_texture_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    filtering: true,
                    comparison: false,
                },
                count: None,
            },
        ],
        label: Some("Ui's object texture bindgroup layout"),
    })
}

pub fn camera_uniform_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("ui pipeline proj+view matrix uniform bind group layout"),
    })
}

/// Create a bindgroup for a texture using a blending operation.
///
/// The blending ops are as follows:
///   0 - color only
///   1 - texture only
///   3 - multiply texture's red channel with color
pub fn create_ui_material_bindgroup(
    device: &wgpu::Device,
    color_blend: u32,
    diffuse_texture_view: &wgpu::TextureView,
    diffuse_texture_sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    let blend_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("ui object blend buffer"),
        contents: bytemuck::cast_slice(&[color_blend]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &object_texture_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: blend_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&diffuse_texture_sampler),
            },
        ],
        label: Some("ui pass diffuse texture bind group"),
    })
}

pub fn create_pipeline(device: &wgpu::Device, format: TextureFormat) -> wgpu::RenderPipeline {
    let ui_vert_shader = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
    let ui_frag_shader = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ui render pipeline layout"),
        bind_group_layouts: &[
            &camera_uniform_bindgroup_layout(device),
            &object_texture_bindgroup_layout(device),
        ],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &ui_vert_shader,
                entry_point: "main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: {
                            let position_size = std::mem::size_of::<[f32; 3]>();
                            let color_size = std::mem::size_of::<[f32; 4]>();
                            let uv_size = std::mem::size_of::<[f32; 2]>();
                            (position_size + color_size + uv_size) as wgpu::BufferAddress
                        },
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Float32x2],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
                        // We need to switch from using a step mode of Vertex to Instance
                        // This means that our shaders will only change to use the next
                        // instance when the shader starts processing a new instance
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x4] ,
                    }
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &ui_frag_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                mask: !0,
                alpha_to_coverage_enabled: false,
                count: 1,
            },
        })
}

pub struct Object<'a> {
    pub mesh_buffer: wgpu::BufferSlice<'a>,
    pub instances: wgpu::BufferSlice<'a>,
    pub instances_range: Range<u32>,
    pub material: Option<&'a wgpu::BindGroup>,
    pub name: Option<&'a str>,
    pub draw: ObjectDraw<'a>,
}

pub fn render<'a, O>(
    label: &'a str,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    pipeline: &'a wgpu::RenderPipeline,
    frame_texture_view: &'a wgpu::TextureView,
    depth_texture_view: &'a wgpu::TextureView,
    default_material_bindgroup: &'a wgpu::BindGroup,
    camera: &'a Camera<'a>,
    objects: O,
)
where
    O: Iterator<Item = &'a Object<'a>>,
{
    tracing::trace!(
        "{} rendering",
        label,
    );

    let encoder_label = format!("{} ui encoder", label);
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some(&encoder_label),
    });

    let render_pass_label = format!("{} ui render pass", label);
    // start the render pass
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(&render_pass_label),
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &frame_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }],
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

    // bind the camera to our shader uniform
    render_pass.set_bind_group(0, camera.bindgroup, &[]);

    // draw objects
    for object in objects {
        tracing::trace!("    object {:?}", object.name);

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
            ObjectDraw::Indexed { index_buffer, index_range, base_vertex, index_format } => {
                render_pass.set_index_buffer(*index_buffer, *index_format);
                render_pass.draw_indexed(index_range.clone(), *base_vertex, object.instances_range.clone());
            }
            ObjectDraw::Default { vertex_range } => {
                render_pass.draw(
                    vertex_range.clone(),
                    object.instances_range.clone(),
                );
            }
        }
    }

    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
}

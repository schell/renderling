//! UI shader pipeline
use encase::UniformBuffer;
use renderling_shader::ui::ShaderColorBlend;
use wgpu::util::DeviceExt;

use crate::{linkage::camera_uniform_layout, UiMaterial};

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
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("Ui's object texture bindgroup layout"),
    })
}

/// Create a bindgroup for a texture using a blending operation.
///
/// The blending ops are as follows:
///   0 - color only
///   1 - texture only
///   2 - multiply texture's red channel with color
pub fn create_ui_material_bindgroup(
    device: &wgpu::Device,
    material: &UiMaterial,
) -> wgpu::BindGroup {
    let shader_blend_style = ShaderColorBlend::from(material.color_blend);
    let blend_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("ui object blend buffer"),
        contents: bytemuck::cast_slice(&[shader_blend_style]),
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
                resource: wgpu::BindingResource::TextureView(&material.diffuse_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&material.diffuse_texture.sampler),
            },
        ],
        label: Some("ui pass diffuse texture bind group"),
    })
}

pub fn create_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let vertex_shader = device.create_shader_module(wgpu::include_spirv!("ui-vertex_ui.spv"));
    let fragment_shader = device.create_shader_module(wgpu::include_spirv!("ui-fragment_ui.spv"));
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ui render pipeline layout"),
        bind_group_layouts: &[
            &camera_uniform_layout(device),
            &object_texture_bindgroup_layout(device),
        ],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ui render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "ui::vertex_ui",
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
                        // This means that our shaders will only change to use the next
                        // instance when the shader starts processing a new instance
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Float32x4, 6 => Float32x4] ,
                    }
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "ui::fragment_ui",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
                unclipped_depth: false,
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
            multiview: None,
        })
}

pub fn render<'a, I, Extra>(
    label: &'a str,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    pipeline: &'a wgpu::RenderPipeline,
    frame_texture_view: &'a wgpu::TextureView,
    depth_texture_view: &'a wgpu::TextureView,
    default_material_bindgroup: &'a wgpu::BindGroup,
    camera: &'a wgpu::BindGroup,
    objects: I,
) where
    I: Iterator<Item = crate::linkage::ShaderObject<'a>>,
    Extra: 'a,
{
    log::trace!("{} rendering", label,);

    let encoder_label = format!("{} ui encoder", label);
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some(&encoder_label),
    });

    let mut render_pass = crate::linkage::begin_render_pass(
        &mut encoder,
        Some(label),
        pipeline,
        frame_texture_view,
        depth_texture_view,
    );

    // bind the camera to our shader uniform
    render_pass.set_bind_group(0, camera, &[]);

    // draw objects
    for object in objects {
        log::trace!("    object {:?}", object.name);
        crate::linkage::render_object(&mut render_pass, object, default_material_bindgroup);
    }

    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
}

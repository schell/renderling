//! A physically based rendering shader.
use encase::UniformBuffer;
use wgpu::{util::DeviceExt, TextureFormat};

pub use renderling_shader::light::{
    DirectionalLightRaw, DirectionalLights, PointLightRaw, PointLights, SpotLightRaw, SpotLights,
    DIRECTIONAL_LIGHTS_MAX, POINT_LIGHTS_MAX, SPOT_LIGHTS_MAX,
};

pub use crate::linkage::{camera_uniform_layout, create_camera_uniform, ObjectDraw};
use crate::BlinnPhongMaterial;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = [x, y, z];
        self
    }

    pub fn with_uv(mut self, u: f32, v: f32) -> Self {
        self.uv = [u, v];
        self
    }

    pub fn with_normal(mut self, x: f32, y: f32, z: f32) -> Self {
        self.normal = [x, y, z];
        self
    }
}

pub fn material_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("forward material bindgroup layout"),
    })
}

pub fn lights_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: Some("pbr-lights-uniform"),
    })
}

/// Helper struct for managing the light uniforms.
pub struct LightsUniform {
    pub point_buffer: wgpu::Buffer,
    pub spot_buffer: wgpu::Buffer,
    pub directional_buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,
}

impl LightsUniform {
    pub fn new(
        device: &wgpu::Device,
        point_lights: &PointLights,
        spot_lights: &SpotLights,
        dir_lights: &DirectionalLights,
    ) -> LightsUniform {
        let point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("point-lights-buffer"),
            contents: todo!("remove this"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let spot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("spot-lights-buffer"),
            contents: todo!("remove this"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let directional_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("directional-lights-buffer"),
            contents: todo!("remove this"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &lights_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: point_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: spot_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: directional_buffer.as_entire_binding(),
                },
            ],
            label: Some("forward light bind group"),
        });

        log::trace!(
            "new lights uniform has {} point lights, {} spot lights and {} dir lights ",
            point_lights.length,
            spot_lights.length,
            dir_lights.length,
        );

        LightsUniform {
            directional_buffer,
            point_buffer,
            spot_buffer,
            bindgroup,
        }
    }

    pub fn update_point_lights(&self, queue: &wgpu::Queue, lights: &PointLights) {
        log::trace!("updating to {} point lights", lights.length);
        queue.write_buffer(&self.point_buffer, 0, todo!());
    }

    pub fn update_spot_lights(&self, queue: &wgpu::Queue, lights: &SpotLights) {
        log::trace!("updating to {} spot lights", lights.length);
        queue.write_buffer(&self.spot_buffer, 0, todo!());
    }

    pub fn update_directional_lights(&self, queue: &wgpu::Queue, lights: &DirectionalLights) {
        log::trace!("updating to {} dir lights", lights.length);
        queue.write_buffer(&self.directional_buffer, 0, todo!());
    }
}

pub fn create_pipeline(
    device: &wgpu::Device,
    format: TextureFormat,
    primitive: Option<wgpu::PrimitiveState>,
) -> wgpu::RenderPipeline {
    let vertex_shader = device.create_shader_module(wgpu::include_spirv!("pbr-vertex_pbr.spv"));
    let fragment_shader = device.create_shader_module(wgpu::include_spirv!("pbr-fragment_pbr.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("renderling forward pipeline layout"),
        bind_group_layouts: &[
            &camera_uniform_layout(device),
            &material_bindgroup_layout(device),
            &lights_bindgroup_layout(device),
        ],
        push_constant_ranges: &[],
    });

    let primitive = primitive.unwrap_or_else(|| wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Cw,
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
        unclipped_depth: false,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("renderling forward pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "pbr::vertex_pbr",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: {
                            let position_size = std::mem::size_of::<[f32; 3]>();
                            let uv_size = std::mem::size_of::<[f32; 2]>();
                            let normal_size = std::mem::size_of::<[f32; 3]>();
                            (position_size + uv_size + normal_size) as wgpu::BufferAddress
                        },
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: (std::mem::size_of::<[[f32; 4]; 4]>() + std::mem::size_of::<[[f32; 3]; 3]>()) as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            3 => Float32x4,
                            4 => Float32x4,
                            5 => Float32x4,
                            6 => Float32x4,
                            7 => Float32x3,
                            8 => Float32x3,
                            9 => Float32x3
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "pbr::fragment_pbr",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
    pipeline
}

/// Helper struct for blinn-phong materials.
pub struct MaterialUniform {
    pub shininess_buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,
}

impl MaterialUniform {
    /// Creates a buffer to store shininess and a bindgroup for the material.
    pub fn new(device: &wgpu::Device, material: &BlinnPhongMaterial) -> MaterialUniform {
        let mut data = UniformBuffer::new(vec![]);
        data.write(&material.shininess).unwrap();
        let shininess_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("renderling forward material shininess"),
            contents: data.into_inner().as_slice(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&material.diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&material.diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&material.specular_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&material.specular_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: shininess_buffer.as_entire_binding(),
                },
            ],
            label: Some("renderling forward material bind group"),
        });

        MaterialUniform {
            shininess_buffer,
            bindgroup,
        }
    }
}

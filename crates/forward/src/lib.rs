//! A forward-shader renderling for simple 3d scenes.
use std::ops::Range;
use wgpu::{util::DeviceExt, TextureFormat};

pub use renderling_core::{Camera, ObjectDraw, light::*, camera_uniform_layout, create_camera_uniform};

#[repr(C)]
#[derive(Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
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
        label: Some("forward lights bindgroup layout"),
    })
}

pub fn create_pipeline(
    device: &wgpu::Device,
    format: TextureFormat,
) -> anyhow::Result<wgpu::RenderPipeline> {
    let forward_vert_shader = device.create_shader_module(wgpu::include_spirv!("forward.vert.spv"));
    let forward_frag_shader = device.create_shader_module(wgpu::include_spirv!("forward.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("renderling forward pipeline layout"),
        bind_group_layouts: &[
            &camera_uniform_layout(device),
            &material_bindgroup_layout(device),
            &lights_bindgroup_layout(device),
        ],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("renderling forward pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &forward_vert_shader,
                entry_point: "main",
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
                module: &forward_frag_shader,
                entry_point: "main",
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
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
    Ok(pipeline)
}


/// Helper struct for blinn-phong materials.
pub struct MaterialUniform {
    pub shininess_buffer: wgpu::Buffer,
    pub bindgroup: wgpu::BindGroup,
}

impl MaterialUniform {
    /// Creates a buffer to store shininess and a bindgroup for the material.
    pub fn new(
        device: &wgpu::Device,
        diffuse_texture_view: &wgpu::TextureView,
        diffuse_texture_sampler: &wgpu::Sampler,
        specular_texture_view: &wgpu::TextureView,
        specular_texture_sampler: &wgpu::Sampler,
        shininess: f32,
    ) -> MaterialUniform {
        let shininess: [f32; 4] = [shininess; 4];
        let shininess_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("renderling forward material shininess"),
            contents: bytemuck::cast_slice(&shininess),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(diffuse_texture_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(specular_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(specular_texture_sampler),
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

pub const MAX_POINT_LIGHTS: usize = 64;
pub const MAX_SPOT_LIGHTS: usize = 32;
pub const MAX_DIRECTIONAL_LIGHTS: usize = 8;

/// The shader only supports a limited number of lights and we need to set some
/// parameters to control the lighting loop in the shader.
trait Light: Default {
    fn set_num_lights(&mut self, num_lights: u32);

    fn process_lights<L: Light>(lights: &mut Vec<L>, max_lights: usize, fill: bool) {
        let len = lights.len();
        let resize_len = if fill { max_lights } else { len.max(1) };
        lights.resize_with(resize_len, L::default);
        lights
            .iter_mut()
            .for_each(|light| light.set_num_lights(len as u32));
    }
}

impl Light for PointLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
}

impl Light for SpotLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
}

impl Light for DirectionalLight {
    fn set_num_lights(&mut self, num_lights: u32) {
        self.num_lights = num_lights;
    }
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
        mut point_lights: Vec<PointLight>,
        mut spot_lights: Vec<SpotLight>,
        mut dir_lights: Vec<DirectionalLight>,
    ) -> LightsUniform {
        PointLight::process_lights(&mut point_lights, MAX_POINT_LIGHTS, true);
        let point_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("forward point light buffer"),
            contents: bytemuck::cast_slice(point_lights.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        SpotLight::process_lights(&mut spot_lights, MAX_SPOT_LIGHTS, true);
        let spot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("forward spot light buffer"),
            contents: bytemuck::cast_slice(&spot_lights),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        DirectionalLight::process_lights(&mut dir_lights, MAX_DIRECTIONAL_LIGHTS, true);
        let directional_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("forward directional light buffer"),
            contents: bytemuck::cast_slice(&dir_lights),
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

        LightsUniform {
            directional_buffer,
            point_buffer,
            spot_buffer,
            bindgroup,
        }
    }

    pub fn update_point_lights(&self, queue: &wgpu::Queue, mut lights: Vec<PointLight>) {
        PointLight::process_lights(&mut lights, MAX_POINT_LIGHTS, false);
        queue.write_buffer(&self.point_buffer, 0, bytemuck::cast_slice(&lights));
    }

    pub fn update_spot_lights(&self, queue: &wgpu::Queue, mut lights: Vec<SpotLight>) {
        SpotLight::process_lights(&mut lights, MAX_SPOT_LIGHTS, false);
        queue.write_buffer(&self.spot_buffer, 0, bytemuck::cast_slice(&lights));
    }

    pub fn update_directional_lights(
        &self,
        queue: &wgpu::Queue,
        mut lights: Vec<DirectionalLight>,
    ) {
        DirectionalLight::process_lights(&mut lights, MAX_DIRECTIONAL_LIGHTS, false);
        queue.write_buffer(&self.directional_buffer, 0, bytemuck::cast_slice(&lights));
    }
}

/// A renderable object.
///
/// Bundles together buffers, ranges and draw instructions.
///
/// **Note:** There is a slot for `Extra` data to help with collation and
/// sorting, if need be.
#[derive(Clone)]
pub struct Object<'a, Extra> {
    pub mesh_buffer: wgpu::BufferSlice<'a>,
    pub instances: wgpu::BufferSlice<'a>,
    pub instances_range: Range<u32>,
    pub name: Option<&'a str>,
    pub draw: ObjectDraw<'a>,
    pub extra: Extra,
}

/// A collection of objects that share the same material.
pub struct ObjectGroup<'a, Extra> {
    pub material: &'a wgpu::BindGroup,
    pub objects: Vec<Object<'a, Extra>>,
}

/// Conducts a render pass.
pub fn render<'a, O, Extra>(
    label: &'a str,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    pipeline: &'a wgpu::RenderPipeline,
    frame_view: &'a wgpu::TextureView,
    depth_view: &'a wgpu::TextureView,
    lights: &'a wgpu::BindGroup,
    camera: &'a Camera<'a>,
    object_groups: O,
) where
    O: Iterator<Item = &'a ObjectGroup<'a, Extra>>,
    Extra: 'a,
{
    tracing::trace!("rendering {}", label);

    let encoder_label = format!("{} forward encoder", label);
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some(&encoder_label),
    });

    let render_pass_label = format!("{} forward render pass", label);
    // start the render pass
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(&render_pass_label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_view,
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
    // bind the lights
    render_pass.set_bind_group(2, lights, &[]);
    // draw objects
    for (group, i) in object_groups.zip(0..) {
        // bind the material for this group
        tracing::trace!("group {}", i);
        render_pass.set_bind_group(1, group.material, &[]);

        // draw objects
        for object in group.objects.iter() {
            tracing::trace!("    object {:?}", object.name);

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
    }

    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
}

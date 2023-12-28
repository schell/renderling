//! Render pipelines and layouts for creating cubemaps.

pub fn cubemap_making_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("cubemap-making bindgroup"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    })
}

pub fn cubemap_making_bindgroup(
    device: &wgpu::Device,
    label: Option<&str>,
    buffer: &wgpu::Buffer,
    // The texture to sample the environment from
    texture: &crate::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &cubemap_making_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

pub struct CubemapMakingRenderPipeline(pub wgpu::RenderPipeline);

impl CubemapMakingRenderPipeline {
    /// Create the rendering pipeline that creates cubemaps from equirectangular
    /// images.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        log::trace!("creating cubemap-making render pipeline with format '{format:?}'");
        let vertex_shader = device.create_shader_module(wgpu::include_spirv!(
            "linkage/skybox-vertex_position_passthru.spv"
        ));
        let fragment_shader = device.create_shader_module(wgpu::include_spirv!(
            "linkage/skybox-fragment_equirectangular.spv"
        ));
        let bg_layout = cubemap_making_bindgroup_layout(device);
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cubemap-making pipeline layout"),
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        CubemapMakingRenderPipeline(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("cubemap-making pipeline"),
                layout: Some(&pp_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "skybox::vertex_position_passthru",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: {
                            let position_size = std::mem::size_of::<glam::Vec3>();
                            position_size as wgpu::BufferAddress
                        },
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float32x3
                        ],
                    }],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader,
                    entry_point: "skybox::fragment_equirectangular",
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            },
        ))
    }
}

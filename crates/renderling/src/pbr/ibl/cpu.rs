//! CPU side of IBL

/// Pipeline for creating a prefiltered environment map from an existing
/// environment cubemap.
pub(crate) fn create_prefiltered_environment_pipeline_and_bindgroup(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
    environment_texture: &crate::texture::Texture,
) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let label = Some("prefiltered environment");
    let bindgroup_layout_desc = wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
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
    };
    let bg_layout = device.create_bind_group_layout(&bindgroup_layout_desc);
    let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bg_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&environment_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&environment_texture.sampler),
            },
        ],
    });
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    let vertex_linkage = crate::linkage::prefilter_environment_cubemap_vertex::linkage(device);
    let fragment_linkage = crate::linkage::prefilter_environment_cubemap_fragment::linkage(device);
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("prefiltered environment"),
        layout: Some(&pp_layout),
        vertex: wgpu::VertexState {
            module: &vertex_linkage.module,
            entry_point: Some(vertex_linkage.entry_point),
            buffers: &[],
            compilation_options: Default::default(),
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
            module: &fragment_linkage.module,
            entry_point: Some(fragment_linkage.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
        cache: None,
    });
    (pipeline, bindgroup)
}

pub fn diffuse_irradiance_convolution_bindgroup_layout(
    device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("convolution bindgroup"),
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
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
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
    })
}

pub fn diffuse_irradiance_convolution_bindgroup(
    device: &wgpu::Device,
    label: Option<&str>,
    buffer: &wgpu::Buffer,
    // The texture to sample the environment from
    texture: &crate::texture::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &diffuse_irradiance_convolution_bindgroup_layout(device),
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

pub struct DiffuseIrradianceConvolutionRenderPipeline(pub wgpu::RenderPipeline);

impl DiffuseIrradianceConvolutionRenderPipeline {
    /// Create the rendering pipeline that performs a convolution.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        log::trace!("creating convolution render pipeline with format '{format:?}'");
        let vertex_linkage = crate::linkage::skybox_cubemap_vertex::linkage(device);
        let fragment_linkage = crate::linkage::di_convolution_fragment::linkage(device);
        // let fragment_shader = device.create_shader_module(wgpu::include_wgsl!(
        //     // TODO: rewrite this shader in Rust after atomics are added to naga spv
        //     "../../wgsl/diffuse_irradiance_convolution.wgsl"
        // ));
        log::trace!("  done.");
        let bg_layout = diffuse_irradiance_convolution_bindgroup_layout(device);
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("convolution pipeline layout"),
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        // TODO: merge irradiance pipeline with cubemap
        let pipeline = DiffuseIrradianceConvolutionRenderPipeline(device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("convolution pipeline"),
                layout: Some(&pp_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_linkage.module,
                    entry_point: Some(vertex_linkage.entry_point),
                    buffers: &[],
                    compilation_options: Default::default(),
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
                    module: &fragment_linkage.module,
                    entry_point: Some(fragment_linkage.entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                multiview: None,
                cache: None,
            },
        ));
        log::trace!("  completed pipeline creation");
        pipeline
    }
}

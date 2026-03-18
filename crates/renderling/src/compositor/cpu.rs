//! CPU-side compositor for alpha-blending a source texture onto a target.

/// Alpha-blends a source texture onto a target framebuffer using a
/// fullscreen quad.
///
/// This is useful for overlaying MSAA-resolved UI content on top of an
/// existing 3D scene without overwriting the scene's pixels.
///
/// ```ignore
/// let compositor = Compositor::new(device, format);
/// // ... render 3D scene to `view` ...
/// // ... render UI to `ui_texture` ...
/// compositor.composite(device, queue, &ui_texture_view, &view);
/// ```
pub struct Compositor {
    pipeline: wgpu::RenderPipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl Compositor {
    /// Create a new compositor targeting the given texture format.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("compositor"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compositor"),
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
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compositor"),
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

        let vertex = crate::linkage::compositor_vertex::linkage(device);
        let fragment = crate::linkage::compositor_fragment::linkage(device);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("compositor"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: Some(vertex.entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
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
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragment.module,
                entry_point: Some(fragment.entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bindgroup_layout,
            sampler,
        }
    }

    /// Alpha-blend the `source` texture onto the `target` framebuffer.
    ///
    /// The existing content of `target` is preserved (`LoadOp::Load`)
    /// and the source is drawn on top using the pipeline's alpha blend
    /// state.
    pub fn composite(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        source: &wgpu::TextureView,
        target: &wgpu::TextureView,
    ) {
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compositor"),
            layout: &self.bindgroup_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compositor"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("compositor"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, Some(&bindgroup), &[]);
            pass.draw(0..6, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}

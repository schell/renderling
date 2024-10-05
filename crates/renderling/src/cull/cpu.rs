//! CPU side of compute culling.

use glam::UVec2;
use snafu::Snafu;

use crate::texture::{
    mips::{MipMapError, MipMapGenerator},
    DepthTexture, Texture,
};

#[derive(Debug, Snafu)]
pub enum CullingError {
    #[snafu(display(
        "Texture is not a depth texture, expected '{:?}' but saw '{seen:?}'",
        Texture::DEPTH_FORMAT
    ))]
    NotADepthTexture { seen: wgpu::TextureFormat },
}

const FRUSTUM_LABEL: Option<&str> = Some("compute-frustum-culling");

/// Computes furstum culling on the GPU.
pub struct FrustumCulling {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    bindgroup: Option<wgpu::BindGroup>,
}

impl FrustumCulling {
    fn new_bindgroup(
        slab_buffer: &wgpu::Buffer,
        indirect_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: FRUSTUM_LABEL,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        indirect_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: FRUSTUM_LABEL,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let linkage = crate::linkage::compute_frustum_culling::linkage(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: FRUSTUM_LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: FRUSTUM_LABEL,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: linkage.entry_point,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Self {
            pipeline,
            bindgroup_layout,
            bindgroup: None,
        }
    }

    pub fn invalidate_bindgroup(&mut self) {
        self.bindgroup = None;
    }

    fn get_bindgroup(
        &mut self,
        device: &wgpu::Device,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) -> &wgpu::BindGroup {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::new_bindgroup(
                slab_buffer,
                indirect_draw_buffer,
                &self.bindgroup_layout,
                device,
            ));
        }
        // UNWRAP: safe because we just set it
        self.bindgroup.as_ref().unwrap()
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
        indirect_draw_count: u32,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: FRUSTUM_LABEL,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: FRUSTUM_LABEL,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            let bindgroup = self.get_bindgroup(device, slab_buffer, indirect_draw_buffer);
            compute_pass.set_bind_group(0, bindgroup, &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
}

const OCCLUSION_LABEL: Option<&str> = Some("compute-occlusion-culling");
const DEPTH_PYRAMID_LABEL: Option<&str> = Some("compute-occlusion-culling-depth-pyramid");

pub struct DepthPyramid {
    downsample_pipeline: wgpu::RenderPipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    texture: wgpu::Texture,
    mip_views: Vec<wgpu::TextureView>,
    mip_samplers: Vec<wgpu::Sampler>,
    mip_bindgroups: Vec<wgpu::BindGroup>,
}

impl DepthPyramid {
    pub fn new(device: &wgpu::Device, size: UVec2, sample_count: u32) -> Self {
        let size = wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        };
        let width_levels = size.width.ilog2();
        let height_levels = size.height.ilog2();
        let mip_level_count = width_levels.min(height_levels);
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: DEPTH_PYRAMID_LABEL,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: sample_count > 1,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        let mut mip_views = vec![];
        let mut mip_samplers = vec![];
        let mut mip_bindgroups = vec![];
        let mip_sampler_desc = wgpu::SamplerDescriptor {
            label: DEPTH_PYRAMID_LABEL,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        for mip_level in 0..mip_level_count {
            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: DEPTH_PYRAMID_LABEL,
                format: None,
                dimension: None,
                aspect: wgpu::TextureAspect::DepthOnly,
                base_mip_level: mip_level,
                mip_level_count: Some(1),
                base_array_layer: 0,
                array_layer_count: None,
            });
            let sampler = device.create_sampler(&mip_sampler_desc);
            let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: DEPTH_PYRAMID_LABEL,
                layout: &bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

            mip_views.push(view);
            mip_samplers.push(sampler);
            mip_bindgroups.push(bindgroup);
        }

        let vertex = crate::linkage::downsample_depth_pyramid_vertex::linkage(device);
        let fragment = crate::linkage::downsample_depth_pyramid_fragment::linkage(device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: DEPTH_PYRAMID_LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: DEPTH_PYRAMID_LABEL,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: &vertex.entry_point,
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
            multisample: wgpu::MultisampleState {
                count: sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment.module,
                entry_point: &fragment.entry_point,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: Texture::DEPTH_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            downsample_pipeline: pipeline,
            bindgroup_layout,
            texture,
            mip_views,
            mip_samplers,
            mip_bindgroups,
        }
    }

    /// Generate mip maps.
    ///
    /// # Errs
    /// Errors if the texture's format doesn't match the generator format.
    pub fn write_depth_pyramid(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        depth_texture: &Texture,
    ) -> Result<(), CullingError> {
        snafu::ensure!(
            depth_texture.texture.format() == self.texture.format(),
            NotADepthTextureSnafu {
                seen: depth_texture.texture.format()
            }
        );

        // First copy the depth texture to the _top_ of our _upside-down_ pyramid.
        //
        // ----------------- <- top (original depth_texture size, mip_level = 0)
        //    -----------
        //       -----
        //         -         <- bottom (1 pixel, mip_level = mip_level_count - 1)
        queue.submit(std::iter::once({
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            encoder.copy_texture_to_texture(
                wgpu::ImageCopyTexture {
                    texture: &depth_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::DepthOnly,
                },
                wgpu::ImageCopyTexture {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::DepthOnly,
                },
                self.texture.size(),
            );
            encoder.finish()
        }));

        // Now run through all mips downsampling one to the next
        let tail_mip_views = self.mip_views.iter().skip(1);
        let mips = self.mip_bindgroups.iter().zip(tail_mip_views);
        for (i, (prev_bindgroup, mip_view)) in mips.enumerate() {
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("depth-downsample-mip{i}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: mip_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(&self.downsample_pipeline);
                render_pass.set_bind_group(0, prev_bindgroup, &[]);
                render_pass.draw(0..6, 0..1);
            }

            queue.submit(std::iter::once(encoder.finish()));
        }

        Ok(())
    }
}

/// Computes occlusion culling on the GPU.
pub struct OcclusionCulling {
    pipeline: wgpu::ComputePipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    bindgroup: Option<wgpu::BindGroup>,
    mip_generator: MipMapGenerator,
    depth_pyramid: Vec<Texture>,
}

impl OcclusionCulling {
    fn new_bindgroup(
        slab_buffer: &wgpu::Buffer,
        indirect_buffer: &wgpu::Buffer,
        layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: OCCLUSION_LABEL,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        indirect_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
        })
    }

    pub fn new(device: &wgpu::Device) -> Self {
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: OCCLUSION_LABEL,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let linkage = crate::linkage::compute_occlusion_culling::linkage(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: OCCLUSION_LABEL,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: OCCLUSION_LABEL,
                    bind_group_layouts: &[&bindgroup_layout],
                    push_constant_ranges: &[],
                }),
            ),
            module: &linkage.module,
            entry_point: linkage.entry_point,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        Self {
            pipeline,
            bindgroup_layout,
            bindgroup: None,
            mip_generator: MipMapGenerator::new(device, Texture::DEPTH_FORMAT),
            depth_pyramid: vec![],
        }
    }

    pub fn invalidate_bindgroup(&mut self) {
        self.bindgroup = None;
    }

    fn get_bindgroup(
        &mut self,
        device: &wgpu::Device,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
    ) -> &wgpu::BindGroup {
        if self.bindgroup.is_none() {
            self.bindgroup = Some(Self::new_bindgroup(
                slab_buffer,
                indirect_draw_buffer,
                &self.bindgroup_layout,
                device,
            ));
        }
        // UNWRAP: safe because we just set it
        self.bindgroup.as_ref().unwrap()
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab_buffer: &wgpu::Buffer,
        indirect_draw_buffer: &wgpu::Buffer,
        indirect_draw_count: u32,
        depth_texture: &DepthTexture,
    ) -> Result<(), MipMapError> {
        let depth_pyramid = if self.depth_pyramid.is_empty() {
            log::info!("occlusion culling - generating depth pyramid mips");
            let width_levels = depth_texture.texture.width().ilog2();
            let height_levels = depth_texture.texture.height().ilog2();
            let mip_levels = width_levels.min(height_levels);
            self.depth_pyramid =
                self.mip_generator
                    .generate(device, queue, &depth_texture.texture, mip_levels)?;
            &self.depth_pyramid
        } else {
            &self.depth_pyramid
        };

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: OCCLUSION_LABEL,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: OCCLUSION_LABEL,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            let bindgroup = self.get_bindgroup(device, slab_buffer, indirect_draw_buffer);
            compute_pass.set_bind_group(0, bindgroup, &[]);
            compute_pass.dispatch_workgroups(indirect_draw_count / 32 + 1, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        Ok(())
    }
}

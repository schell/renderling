//! Resources for the bloom filter pass.

use std::sync::Arc;

use glam::UVec2;
use moongraph::{View, ViewMut};

use crate::{HdrSurface, Uniform};

fn create_bloom_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    width: u32,
    height: u32,
) -> crate::Texture {
    crate::Texture::new_with(
        device,
        queue,
        Some("bloom pingpong tex"),
        Some(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING),
        Some(device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })),
        wgpu::TextureFormat::Rgba16Float,
        4,
        1,
        width,
        height,
        1,
        &[],
    )
}

fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bloom"),
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
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
    })
}

fn create_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
    let bg_layout = create_bindgroup_layout(device);
    let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("bloom"),
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("bloom filter"),
        layout: Some(&pp_layout),
        vertex: wgpu::VertexState {
            module: &device
                .create_shader_module(wgpu::include_spirv!("linkage/vertex_generate_mipmap.spv")),
            entry_point: "vertex_generate_mipmap",
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
        fragment: Some(wgpu::FragmentState {
            module: &device
                .create_shader_module(wgpu::include_spirv!("linkage/fragment_bloom.spv")),
            entry_point: "fragment_bloom",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: 1,
        },
        multiview: None,
    })
}

fn create_bindgroup(
    device: &wgpu::Device,
    horizontal_uniform: &Uniform<u32>,
    size_uniform: &Uniform<UVec2>,
    texture: &crate::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bloom filter"),
        layout: &create_bindgroup_layout(device),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    horizontal_uniform.buffer().as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(
                    size_uniform.buffer().as_entire_buffer_binding(),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
    })
}

pub struct BloomFilter {
    // A plain transparent texture to use as a default in case of "no bloom"
    _default_texture: crate::Texture,
    default_texture_bindgroup: Arc<wgpu::BindGroup>,
    textures: [crate::Texture; 2],
    tonemap_bindgroup: Arc<wgpu::BindGroup>,
    pipeline: wgpu::RenderPipeline,
    horizontal_uniform: Uniform<u32>,
    size_uniform: Uniform<UVec2>,
    initial_bindgroup: Option<wgpu::BindGroup>,
    bindgroups: [wgpu::BindGroup; 2],
}

impl BloomFilter {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> Self {
        let default_texture = crate::Texture::new_with(
            device,
            queue,
            Some("BloomFilter.default_texture"),
            None,
            Some(device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("BloomFilter.default_texture"),
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            })),
            wgpu::TextureFormat::Rgba8Unorm,
            4,
            1,
            1,
            1,
            1,
            &[0, 0, 0, 0],
        );
        let tonemap_bg_layout = crate::hdr::texture_and_sampler_layout(device, Some("bloom"));
        let default_texture_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("no-bloom"),
            layout: &tonemap_bg_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&default_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&default_texture.sampler),
                },
            ],
        });

        let textures = [
            create_bloom_texture(device, queue, width, height),
            create_bloom_texture(device, queue, width, height),
        ];
        let tonemap_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("tonemap-bloom"),
            layout: &tonemap_bg_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&textures[1].view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&textures[1].sampler),
                },
            ],
        });

        let size_uniform = Uniform::new(
            device,
            UVec2::new(width, height),
            wgpu::BufferUsages::empty(),
            wgpu::ShaderStages::FRAGMENT,
        );
        let horizontal_uniform = Uniform::new(
            device,
            1,
            wgpu::BufferUsages::empty(),
            wgpu::ShaderStages::FRAGMENT,
        );
        let bindgroups = [
            // bindgroup 'A' reads from pingpong 1 and writes to pingpong 0 (see `run`)
            create_bindgroup(device, &horizontal_uniform, &size_uniform, &textures[1]),
            // bindgroup 'B' reads from pingpong 0 and writes to pingpong 1 (see `run`)
            create_bindgroup(device, &horizontal_uniform, &size_uniform, &textures[0]),
        ];
        BloomFilter {
            pipeline: create_pipeline(device),
            size_uniform,
            horizontal_uniform,
            textures,
            initial_bindgroup: None,
            bindgroups,
            _default_texture: default_texture,
            default_texture_bindgroup: default_texture_bindgroup.into(),
            tonemap_bindgroup: tonemap_bindgroup.into(),
        }
    }

    pub fn run(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        hdr_surface: &crate::HdrSurface,
    ) -> Arc<wgpu::BindGroup> {
        let brightness_texture = if let Some(tex) = hdr_surface.bloom_texture.as_ref() {
            tex
        } else {
            return self.default_texture_bindgroup.clone();
        };

        // update the size if the size has changed
        let size = brightness_texture.texture.size();
        let size = UVec2::new(size.width, size.height);
        if size != *self.size_uniform {
            *self.size_uniform = size;
            self.size_uniform.update(queue);
        }

        if brightness_texture.texture.size() != self.textures[0].texture.size() {
            let width = size.x;
            let height = size.y;
            self.textures = [
                create_bloom_texture(device, queue, width, height),
                create_bloom_texture(device, queue, width, height),
            ];
            self.bindgroups = [
                create_bindgroup(
                    device,
                    &self.horizontal_uniform,
                    &self.size_uniform,
                    &self.textures[1],
                ),
                create_bindgroup(
                    device,
                    &self.horizontal_uniform,
                    &self.size_uniform,
                    &self.textures[0],
                ),
            ];
        }

        // if the brightness texture is not
        if self.initial_bindgroup.is_none() {
            self.initial_bindgroup = Some(
                // initial bindgroup reads from brightness texture
                create_bindgroup(
                    device,
                    &self.horizontal_uniform,
                    &self.size_uniform,
                    brightness_texture,
                ),
            );
        };
        // UNWRAP: safe because we just set it above
        let initial_bindgroup = self.initial_bindgroup.as_ref().unwrap();

        // first do a clear pass on the pingpong textures
        crate::frame::conduct_clear_pass(
            device,
            queue,
            Some("bloom filter clear"),
            vec![&self.textures[0].view, &self.textures[1].view],
            None,
            wgpu::Color::TRANSPARENT,
        );

        for i in 0..10 {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("bloom-filter{i}")),
            });

            // index == 0 is group 'A', 1 is group 'B'
            let index = i % 2;

            *self.horizontal_uniform = index as u32;
            self.horizontal_uniform.update(queue);

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("bloomfilter{i}_index{index}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.textures[index].view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.pipeline);

                // * if i == 0 we read from brightness_texture and write to textures[0]
                // * if index == 1 we read from textures[0] and write to textures[1]
                // * if index == 0 we read from textures[1] and write to textures[0]
                let bindgroup = if i == 0 {
                    initial_bindgroup
                } else {
                    &self.bindgroups[index]
                };
                render_pass.set_bind_group(0, bindgroup, &[]);
                render_pass.draw(0..6, 0..1);
            }

            queue.submit([encoder.finish()]);
        }

        self.tonemap_bindgroup.clone()
    }
}

pub struct BloomResult(pub Arc<wgpu::BindGroup>);

pub fn bloom_filter(
    (device, queue, mut bloom, hdr): (
        View<crate::Device>,
        View<crate::Queue>,
        ViewMut<BloomFilter>,
        View<HdrSurface>,
    ),
) -> Result<(BloomResult,), crate::WgpuStateError> {
    let bg = bloom.run(&device, &queue, &hdr);
    Ok((BloomResult(bg),))
}

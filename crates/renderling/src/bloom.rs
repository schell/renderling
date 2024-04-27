//! Physically based bloom.
//!
//! As described in [learnopengl.com's Physically Based Bloom article](https://learnopengl.com/Guest-Articles/2022/Phys.-Based-Bloom).
use crabslab::{Id, Slab, SlabItem};
use glam::{UVec2, Vec2, Vec4, Vec4Swizzles};
use spirv_std::{image::Image2d, spirv, Sampler};

#[derive(Clone, Copy, SlabItem)]
pub struct BloomConfig {
    pub resolution: UVec2,
    pub upsample_filter_radius: Vec2,
}

impl Default for BloomConfig {
    fn default() -> Self {
        Self {
            resolution: UVec2::ONE,
            upsample_filter_radius: Vec2::ONE,
        }
    }
}

#[cfg(feature = "bloom_vertex")]
/// A passthru vertex shader to facilitate a bloom effect.
#[spirv(vertex)]
pub fn bloom_vertex(
    #[spirv(vertex_index)] vertex_index: u32,
    #[spirv(instance_index)] in_id: u32,
    out_uv: &mut Vec2,
    #[spirv(flat)] out_id: &mut u32,
    #[spirv(position)] out_clip_pos: &mut Vec4,
) {
    let i = (vertex_index % 6) as usize;
    *out_uv = crate::math::UV_COORD_QUAD_CCW[i];
    *out_clip_pos = crate::math::CLIP_SPACE_COORD_QUAD_CCW[i];
    *out_id = in_id;
}

#[cfg(feature = "bloom_downsample_fragment")]
/// Performs downsampling on a texture.
///
/// As taken from Call Of Duty method - presented at ACM Siggraph 2014.
///
/// This particular method was customly designed to eliminate
/// "pulsating artifacts and temporal stability issues".
#[spirv(fragment)]
pub fn bloom_downsample_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_pixel_size_id: Id<Vec2>,
    // frag_color
    downsample: &mut Vec4,
) {
    use glam::Vec3;

    let Vec2 { x, y } = slab.read(in_pixel_size_id);

    // Take 13 samples around current texel:
    // a - b - c
    // - j - k -
    // d - e - f
    // - l - m -
    // g - h - i
    // === ('e' is the current texel) ===
    let a = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y + 2.0 * y));
    let b = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y + 2.0 * y));
    let c = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y + 2.0 * y));

    let d = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y));
    let e = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y));
    let f = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y));

    let g = texture.sample(*sampler, Vec2::new(in_uv.x - 2.0 * x, in_uv.y - 2.0 * y));
    let h = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y - 2.0 * y));
    let i = texture.sample(*sampler, Vec2::new(in_uv.x + 2.0 * x, in_uv.y - 2.0 * y));

    let j = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y + y));
    let k = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y + y));
    let l = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y - y));
    let m = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y - y));

    // Apply weighted distribution:
    // 0.5 + 0.125 + 0.125 + 0.125 + 0.125 = 1
    // a,b,d,e * 0.125
    // b,c,e,f * 0.125
    // d,e,g,h * 0.125
    // e,f,h,i * 0.125
    // j,k,l,m * 0.5
    // This shows 5 square areas that are being sampled. But some of them overlap,
    // so to have an energy preserving downsample we need to make some adjustments.
    // The weights are the distributed so that the sum of j,k,l,m (e.g.)
    // contribute 0.5 to the final color output. The code below is written
    // to effectively yield this sum. We get:
    // 0.125*5 + 0.03125*4 + 0.0625*4 = 1
    let f1 = 0.125;
    let f2 = 0.0625;
    let f3 = 0.03125;
    let center = e * f1;
    let inner = (j + k + l + m) * f1;
    let outer = (b + d + h + f) * f2;
    let furthest = (a + c + g + i) * f3;
    let min = Vec3::splat(f32::EPSILON).extend(1.0);
    *downsample = (center + inner + outer + furthest).max(min);
}

#[cfg(feature = "bloom_upsample_fragment")]
/// This shader performs upsampling on a texture.
/// Taken from Call Of Duty method, presented at ACM Siggraph 2014.
#[spirv(fragment)]
pub fn bloom_upsample_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    // Remember to add bilinear minification filter for this texture!
    // Remember to use a floating-point texture format (for HDR)!
    // Remember to use edge clamping for this texture!
    #[spirv(descriptor_set = 0, binding = 1)] texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] filter_radius_id: Id<Vec2>,
    // frag_color
    upsample: &mut Vec4,
) {
    // The filter kernel is applied with a radius, specified in texture
    // coordinates, so that the radius will vary across mip resolutions.
    let Vec2 { x, y } = slab.read(filter_radius_id);

    // Take 9 samples around current texel:
    // a - b - c
    // d - e - f
    // g - h - i
    // === ('e' is the current texel) ===
    let a = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y + y));
    let b = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y + y));
    let c = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y + y));

    let d = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y));
    let e = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y));
    let f = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y));

    let g = texture.sample(*sampler, Vec2::new(in_uv.x - x, in_uv.y - y));
    let h = texture.sample(*sampler, Vec2::new(in_uv.x, in_uv.y - y));
    let i = texture.sample(*sampler, Vec2::new(in_uv.x + x, in_uv.y - y));

    // Apply weighted distribution, by using a 3x3 tent filter:
    //  1   | 1 2 1 |
    // -- * | 2 4 2 |
    // 16   | 1 2 1 |
    let mut sample = e * 4.0;
    sample += (b + d + f + h) * 2.0;
    sample += a + c + g + i;
    sample *= 1.0 / 16.0;
    *upsample = sample;
}

#[cfg(feature = "bloom_mix_fragment")]
#[spirv(fragment)]
pub fn bloom_mix_fragment(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    #[spirv(descriptor_set = 0, binding = 1)] hdr_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 2)] hdr_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] bloom_texture: &Image2d,
    #[spirv(descriptor_set = 0, binding = 4)] bloom_sampler: &Sampler,
    in_uv: Vec2,
    #[spirv(flat)] in_bloom_strength_id: Id<f32>,
    frag_color: &mut Vec4,
) {
    let bloom_strength = slab.read(in_bloom_strength_id);
    let hdr = hdr_texture.sample(*hdr_sampler, in_uv).xyz();
    let bloom = bloom_texture.sample(*bloom_sampler, in_uv).xyz();
    let color = hdr.lerp(bloom, bloom_strength);
    *frag_color = color.extend(1.0)
}

#[cfg(not(target_arch = "spirv"))]
mod cpu {
    use std::sync::Arc;

    use super::*;
    use crate::{
        slab::{Hybrid, HybridArray, SlabManager},
        texture,
    };

    fn create_bindgroup_layout(
        device: &wgpu::Device,
        label: Option<&str>,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        })
    }

    fn create_bloom_downsample_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let label = Some("bloom downsample");
        let bindgroup_layout = create_bindgroup_layout(device, label);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let vertex_module = crate::linkage::bloom_vertex::linkage(device);
        let fragment_module = crate::linkage::bloom_downsample_fragment::linkage(device);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_module.module,
                entry_point: &vertex_module.entry_point,
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
                module: &fragment_module.module,
                entry_point: &fragment_module.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        })
    }

    fn create_bloom_upsample_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let label = Some("bloom upsample");
        let bindgroup_layout = create_bindgroup_layout(device, label);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let vertex_module = crate::linkage::bloom_vertex::linkage(device);
        let fragment_module = crate::linkage::bloom_upsample_fragment::linkage(device);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_module.module,
                entry_point: &vertex_module.entry_point,
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
                module: &fragment_module.module,
                entry_point: &fragment_module.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        })
    }

    const MIN_MIP_DIMENSION: u32 = 16;

    fn config_resolutions(resolution: UVec2) -> impl Iterator<Item = UVec2> {
        let num_textures = resolution.x.min(resolution.y).ilog2();
        (0..=num_textures)
            .filter_map(move |i| {
                let width = resolution.x >> i;
                let height = resolution.y >> i;
                if width < MIN_MIP_DIMENSION || height < MIN_MIP_DIMENSION {
                    None
                } else {
                    Some(UVec2::new(width, height))
                }
            })
            .fuse()
    }

    fn create_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        label: Option<&str>,
        extra_usages: wgpu::TextureUsages,
    ) -> texture::Texture {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        crate::Texture::new_with(
            &device,
            &queue,
            label,
            Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | extra_usages,
            ),
            Some(sampler),
            wgpu::TextureFormat::Rgba16Float,
            4,
            2,
            width,
            height,
            1,
            &[],
        )
    }

    fn create_textures(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resolution: UVec2,
    ) -> Vec<texture::Texture> {
        let num_textures = resolution.x.min(resolution.y).ilog2();
        log::trace!("creating {num_textures} bloom textures at resolution {resolution}");
        let mut textures = vec![];
        for (
            i,
            UVec2 {
                x: width,
                y: height,
            },
        ) in config_resolutions(resolution).skip(1).enumerate()
        {
            let title = format!("bloom texture[{i}]");
            let label = Some(title.as_str());
            let texture = create_texture(
                device,
                queue,
                width,
                height,
                label,
                wgpu::TextureUsages::empty(),
            );
            textures.push(texture);
        }
        textures
    }

    fn create_bindgroup(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
        tex: &crate::Texture,
    ) -> wgpu::BindGroup {
        let label = Some("bloom");
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&tex.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&tex.sampler),
                },
            ],
        })
    }

    fn create_bindgroups(
        device: &wgpu::Device,
        pipeline: &wgpu::RenderPipeline,
        buffer: &wgpu::Buffer,
        textures: &[crate::Texture],
    ) -> Vec<wgpu::BindGroup> {
        let layout = pipeline.get_bind_group_layout(0);
        textures
            .iter()
            .map(|tex| create_bindgroup(device, &layout, buffer, tex))
            .collect()
    }

    fn create_mix_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let label = Some("bloom mix");
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let vertex_module = crate::linkage::bloom_vertex::linkage(device);
        let fragment_module = crate::linkage::bloom_mix_fragment::linkage(device);
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_module.module,
                entry_point: &vertex_module.entry_point,
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
                module: &fragment_module.module,
                entry_point: &fragment_module.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        })
    }

    fn create_mix_bindgroup(
        device: &wgpu::Device,
        pipeline: &wgpu::RenderPipeline,
        slab_buffer: &wgpu::Buffer,
        hdr_texture: &crate::Texture,
        bloom_texture: &crate::Texture,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bloom mix"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(slab_buffer.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&hdr_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&hdr_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&bloom_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&bloom_texture.sampler),
                },
            ],
        })
    }

    /// Performs a "physically based" bloom effect on a texture.
    ///
    /// Contains pipelines, down/upsampling textures, a buffer
    /// to communicate configuration to the shaders, and bindgroups.
    pub struct Bloom {
        slab: SlabManager,
        resolution: UVec2,
        downsample_pixel_sizes: HybridArray<Vec2>,
        downsample_pipeline: Arc<wgpu::RenderPipeline>,
        upsample_filter_radius: Hybrid<Vec2>,
        upsample_pipeline: Arc<wgpu::RenderPipeline>,
        textures: Vec<texture::Texture>,
        bindgroups: Vec<wgpu::BindGroup>,
        hdr_texture: texture::Texture,
        hdr_texture_downsample_bindgroup: wgpu::BindGroup,
        mix_pipeline: wgpu::RenderPipeline,
        mix_bindgroup: wgpu::BindGroup,
        mix_texture: crate::Texture,
        mix_strength: Hybrid<f32>,
    }

    impl Bloom {
        pub fn new(
            device: &crate::Device,
            queue: &crate::Queue,
            resolution: UVec2,
            hdr_texture: &crate::Texture,
        ) -> Self {
            let mut slab = SlabManager::default();

            let downsample_pixel_sizes = slab.new_hybrid_array(
                config_resolutions(resolution).map(|r| 1.0 / Vec2::new(r.x as f32, r.y as f32)),
            );
            let upsample_filter_radius = slab.new_hybrid(Vec2::ONE);
            let mix_strength = slab.new_hybrid(0.04f32);
            let slab_buffer = slab.recreate_buffer(
                device,
                queue,
                Some("new bloom slab upkeep"),
                wgpu::BufferUsages::empty(),
            );

            let textures = create_textures(device, queue, resolution);
            let downsample_pipeline = Arc::new(create_bloom_downsample_pipeline(device));
            let upsample_pipeline = Arc::new(create_bloom_upsample_pipeline(device));

            // up and downsample pipelines have the same layout, so we just choose one for the layout
            let bindgroups =
                create_bindgroups(device, &downsample_pipeline, &slab_buffer, &textures);
            let hdr_texture_downsample_bindgroup = create_bindgroup(
                device,
                &downsample_pipeline.get_bind_group_layout(0),
                &slab_buffer,
                hdr_texture,
            );
            let mix_texture = create_texture(
                device,
                queue,
                resolution.x,
                resolution.y,
                Some("bloom mix"),
                wgpu::TextureUsages::COPY_SRC,
            );
            let mix_pipeline = create_mix_pipeline(device);
            let mix_bindgroup = create_mix_bindgroup(
                device,
                &mix_pipeline,
                &slab_buffer,
                &hdr_texture,
                &textures[0],
            );

            Self {
                slab,
                resolution,
                downsample_pixel_sizes,
                downsample_pipeline,
                upsample_filter_radius,
                upsample_pipeline,
                textures,
                bindgroups,
                hdr_texture: hdr_texture.clone(),
                hdr_texture_downsample_bindgroup,
                mix_pipeline,
                mix_texture,
                mix_bindgroup,
                mix_strength,
            }
        }

        pub(crate) fn render_downsamples(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
            struct DownsampleItem<'a> {
                view: &'a wgpu::TextureView,
                bindgroup: &'a wgpu::BindGroup,
                pixel_size: Id<Vec2>,
            }
            // Get all the bindgroups (which are what we're reading from),
            // starting with the hdr frame.
            // Since `bindgroups` are one element greater (we pushed `hdr_texture_bindgroup` to the
            // front) the last bindgroup will not be used, which is good - we don't need to read from
            // the smallest texture during downsampling.
            let bindgroups = std::iter::once(&self.hdr_texture_downsample_bindgroup)
                .chain(self.bindgroups.iter());
            let items = self
                .textures
                .iter()
                .zip(bindgroups)
                .zip(self.downsample_pixel_sizes.array().iter())
                .map(|((tex, bindgroup), pixel_size)| DownsampleItem {
                    view: &tex.view,
                    bindgroup,
                    pixel_size,
                });
            for (
                i,
                DownsampleItem {
                    view,
                    bindgroup,
                    pixel_size,
                },
            ) in items.enumerate()
            {
                let title = format!("bloom downsample {i}");
                log::trace!("rendering {title}");
                let label = Some(title.as_str());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    render_pass.set_pipeline(&self.downsample_pipeline);
                    render_pass.set_bind_group(0, bindgroup, &[]);
                    let id = pixel_size.into();
                    render_pass.draw(0..6, id..id + 1);
                }
                queue.submit(std::iter::once(encoder.finish()));
            }
        }

        fn render_upsamples(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
            struct UpsampleItem<'a> {
                view: &'a wgpu::TextureView,
                bindgroup: &'a wgpu::BindGroup,
            }
            // Get all the bindgroups (which are what we're reading from),
            // starting with the last mip.
            let bindgroups = self.bindgroups.iter().rev();
            // Get all the texture views (which are what we're writing to),
            // starting with the second-to-last mip.
            let views = self.textures.iter().rev().skip(1).map(|t| &t.view);
            let items = bindgroups
                .zip(views)
                .map(|(bindgroup, view)| UpsampleItem { view, bindgroup });
            for (i, UpsampleItem { view, bindgroup }) in items.enumerate() {
                let title = format!("bloom upsample {i}");
                log::trace!("rendering {title}");
                let label = Some(title.as_str());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    render_pass.set_pipeline(&self.upsample_pipeline);
                    render_pass.set_bind_group(0, bindgroup, &[]);
                    let id = self.upsample_filter_radius.id().into();
                    render_pass.draw(0..6, id..id + 1);
                }
                queue.submit(std::iter::once(encoder.finish()));
            }
        }

        fn render_mix(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
            let label = Some("bloom mix");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.mix_texture.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                render_pass.set_pipeline(&self.mix_pipeline);
                render_pass.set_bind_group(0, &self.mix_bindgroup, &[]);
                let id = self.mix_strength.id().into();
                render_pass.draw(0..6, id..id + 1);
            }
            encoder.copy_texture_to_texture(
                wgpu::ImageCopyTexture {
                    texture: &self.mix_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyTexture {
                    texture: &self.hdr_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: self.resolution.x,
                    height: self.resolution.y,
                    depth_or_array_layers: 1,
                },
            );
            queue.submit(std::iter::once(encoder.finish()));
        }

        pub fn bloom(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
            self.slab.upkeep(
                device,
                queue,
                Some("bloom upkeep"),
                wgpu::BufferUsages::empty(),
            );
            self.render_downsamples(device, queue);
            self.render_upsamples(device, queue);
            self.render_mix(device, queue);
        }
    }

    #[cfg(test)]
    mod test {
        use glam::Vec3;

        use crate::{Camera, HdrSurface, Renderling};

        use super::*;

        #[test]
        fn bloom_texture_sizes_sanity() {
            let sizes = config_resolutions(UVec2::new(1024, 800)).collect::<Vec<_>>();
            assert_eq!(
                vec![
                    UVec2::new(1024, 800),
                    UVec2::new(512, 400),
                    UVec2::new(256, 200),
                    UVec2::new(128, 100),
                    UVec2::new(64, 50),
                    UVec2::new(32, 25)
                ],
                sizes
            );
            let pixel_sizes = config_resolutions(UVec2::new(1024, 800))
                .map(|r| 1.0 / Vec2::new(r.x as f32, r.y as f32))
                .collect::<Vec<_>>();
            assert_eq!(
                vec![
                    Vec2::new(0.0009765625, 0.00125),
                    Vec2::new(0.001953125, 0.0025),
                    Vec2::new(0.00390625, 0.005),
                    Vec2::new(0.0078125, 0.01),
                    Vec2::new(0.015625, 0.02),
                    Vec2::new(0.03125, 0.04)
                ],
                pixel_sizes
            );
        }

        #[test]
        fn bloom_sanity() {
            let width = 1024;
            let height = 800;
            let mut r = Renderling::headless(width, height);
            let mut stage = r.new_stage();
            stage.configure_graph(&mut r, true);
            let doc = stage
                .load_gltf_document_from_path("../../gltf/EmissiveStrengthTest.glb")
                .unwrap();
            let projection = crate::camera::perspective(width as f32, height as f32);
            let view = crate::camera::look_at(Vec3::new(0.0, 2.0, 18.0), Vec3::ZERO, Vec3::Y);
            let camera = stage.new_hybrid(Camera::new(projection, view));
            let skybox = stage
                .new_skybox_from_path("../../img/hdr/helipad.hdr", camera.id())
                .unwrap();
            stage.set_skybox(skybox);
            let scene_index = doc.default_scene.unwrap();
            let nodes = doc.scenes.get(scene_index).unwrap();
            let _scene = stage
                .draw_gltf_scene(&doc, nodes.into_iter().copied(), camera.id())
                .unwrap();
            let img = r.render_image().unwrap();
            img_diff::save("bloom/sanity_before.png", img);
            let (device, queue) = r.get_device_and_queue_owned();
            let mut bloom = {
                let hdr_surface = r.graph.get_resource::<HdrSurface>().unwrap().unwrap();
                Bloom::new(
                    &device,
                    &queue,
                    UVec2::new(width, height),
                    &hdr_surface.hdr_texture,
                )
            };

            fn save_bloom_texture(
                r: &mut Renderling,
                device: &wgpu::Device,
                queue: &wgpu::Queue,
                size: UVec2,
                texture: &crate::Texture,
                name: &str,
            ) {
                let copied = crate::Texture::read(
                    &texture.texture,
                    &device,
                    &queue,
                    size.x as usize,
                    size.y as usize,
                    4,
                    2,
                );
                log::info!("  done!");

                let pixels = copied.pixels(r.get_device());
                let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                    .iter()
                    .map(|p| half::f16::from_bits(*p).to_f32())
                    .collect::<Vec<_>>();
                assert_eq!((size.x * size.y * 4) as usize, pixels.len());
                let img: image::Rgba32FImage =
                    image::ImageBuffer::from_vec(size.x, size.y, pixels).unwrap();
                let img = image::DynamicImage::from(img);
                let img = img.to_rgba8();
                img_diff::save(name, img);
            }
            log::info!("rendering downsamples");
            bloom.render_downsamples(&device, &queue);
            log::info!("  done!");
            for (i, (texture, size)) in bloom
                .textures
                .iter()
                .zip(config_resolutions(UVec2::new(width, height)).skip(1))
                .enumerate()
            {
                log::info!("reading downsample {i}");
                save_bloom_texture(
                    &mut r,
                    &device,
                    &queue,
                    size,
                    texture,
                    &format!("bloom/downsample_{i}.png"),
                );
            }

            log::info!("rendering upsamples");
            bloom.render_upsamples(&device, &queue);
            log::info!("  done!");
            for (i, (texture, size)) in bloom
                .textures
                .iter()
                .zip(config_resolutions(UVec2::new(width, height)).skip(1))
                .enumerate()
            {
                log::info!("reading upsample {i}");
                save_bloom_texture(
                    &mut r,
                    &device,
                    &queue,
                    size,
                    texture,
                    &format!("bloom/upsample_{i}.png"),
                );
            }

            log::info!("rendering mix");
            bloom.render_mix(&device, &queue);
            log::info!("  done!");
            log::info!("reading mix and hdr textures");
            save_bloom_texture(
                &mut r,
                &device,
                &queue,
                UVec2::new(width, height),
                &bloom.mix_texture,
                &format!("bloom/mix.png"),
            );
            save_bloom_texture(
                &mut r,
                &device,
                &queue,
                UVec2::new(width, height),
                &bloom.hdr_texture,
                &format!("bloom/hdr.png"),
            );
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

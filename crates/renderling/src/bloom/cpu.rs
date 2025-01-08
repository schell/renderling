//! Bloom.
use core::ops::Deref;
use std::sync::{Arc, RwLock};

use craballoc::{
    prelude::{Hybrid, HybridArray, SlabAllocator},
    runtime::WgpuRuntime,
};
use crabslab::Id;
use glam::{UVec2, Vec2};

use crate::texture::{self, Texture};

fn create_bindgroup_layout(device: &wgpu::Device, label: Option<&str>) -> wgpu::BindGroupLayout {
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
            entry_point: Some(vertex_module.entry_point),
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
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &fragment_module.module,
            entry_point: Some(fragment_module.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: None,
                write_mask: wgpu::ColorWrites::all(),
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
        cache: None,
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
            entry_point: Some(vertex_module.entry_point),
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
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &fragment_module.module,
            entry_point: Some(fragment_module.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::all(),
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
        cache: None,
    })
}

fn config_resolutions(resolution: UVec2) -> impl Iterator<Item = UVec2> {
    let num_textures = resolution.x.min(resolution.y).ilog2();
    (0..=num_textures).map(move |i| UVec2::new(resolution.x >> i, resolution.y >> i))
}

fn create_texture(
    runtime: impl AsRef<WgpuRuntime>,
    width: u32,
    height: u32,
    label: Option<&str>,
    extra_usages: wgpu::TextureUsages,
) -> texture::Texture {
    let sampler = runtime
        .as_ref()
        .device
        .create_sampler(&wgpu::SamplerDescriptor {
            label,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
    Texture::new_with(
        runtime,
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

fn create_textures(runtime: impl AsRef<WgpuRuntime>, resolution: UVec2) -> Vec<texture::Texture> {
    let resolutions = config_resolutions(resolution).collect::<Vec<_>>();
    log::trace!(
        "creating {} bloom textures at resolution {resolution}",
        resolutions.len()
    );
    let mut textures = vec![];
    for (
        i,
        UVec2 {
            x: width,
            y: height,
        },
    ) in resolutions.into_iter().skip(1).enumerate()
    {
        let title = format!("bloom texture[{i}]");
        let label = Some(title.as_str());
        let texture = create_texture(
            runtime.as_ref(),
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
    tex: &Texture,
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
    textures: &[Texture],
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
            entry_point: Some(vertex_module.entry_point),
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
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &fragment_module.module,
            entry_point: Some(fragment_module.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::all(),
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
        cache: None,
    })
}

fn create_mix_bindgroup(
    device: &wgpu::Device,
    pipeline: &wgpu::RenderPipeline,
    slab_buffer: &wgpu::Buffer,
    hdr_texture: &Texture,
    bloom_texture: &Texture,
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

/// Performs a "physically based" bloom effect on a texture. CPU only.
///
/// Contains pipelines, down/upsampling textures, a buffer
/// to communicate configuration to the shaders, and bindgroups.
///
/// Clones of [`Bloom`] all point to the same resources.
#[derive(Clone)]
pub struct Bloom {
    slab: SlabAllocator<WgpuRuntime>,

    downsample_pixel_sizes: HybridArray<Vec2>,
    downsample_pipeline: Arc<wgpu::RenderPipeline>,

    upsample_filter_radius: Hybrid<Vec2>,
    upsample_pipeline: Arc<wgpu::RenderPipeline>,

    textures: Arc<RwLock<Vec<texture::Texture>>>,
    bindgroups: Arc<RwLock<Vec<wgpu::BindGroup>>>,
    hdr_texture_downsample_bindgroup: Arc<RwLock<wgpu::BindGroup>>,

    mix_pipeline: Arc<wgpu::RenderPipeline>,
    mix_bindgroup: Arc<RwLock<wgpu::BindGroup>>,
    mix_texture: Arc<RwLock<Texture>>,
    mix_strength: Hybrid<f32>,
}

impl Bloom {
    pub fn new(runtime: impl AsRef<WgpuRuntime>, hdr_texture: &Texture) -> Self {
        let runtime = runtime.as_ref();
        let resolution = UVec2::new(hdr_texture.width(), hdr_texture.height());

        let slab = SlabAllocator::new(runtime, wgpu::BufferUsages::empty());
        let downsample_pixel_sizes = slab.new_array(
            config_resolutions(resolution).map(|r| 1.0 / Vec2::new(r.x as f32, r.y as f32)),
        );
        let upsample_filter_radius =
            slab.new_value(1.0 / Vec2::new(resolution.x as f32, resolution.y as f32));
        let mix_strength = slab.new_value(0.04f32);
        let slab_buffer = slab.get_updated_buffer();

        let downsample_pipeline = Arc::new(create_bloom_downsample_pipeline(&runtime.device));
        let upsample_pipeline = Arc::new(create_bloom_upsample_pipeline(&runtime.device));
        let mix_pipeline = Arc::new(create_mix_pipeline(&runtime.device));

        let hdr_texture_downsample_bindgroup = create_bindgroup(
            &runtime.device,
            &downsample_pipeline.get_bind_group_layout(0),
            &slab_buffer,
            hdr_texture,
        );
        let mix_texture = create_texture(
            runtime,
            resolution.x,
            resolution.y,
            Some("bloom mix"),
            wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
        );

        let textures = create_textures(runtime, resolution);
        let bindgroups = create_bindgroups(
            &runtime.device,
            &downsample_pipeline,
            &slab_buffer,
            &textures,
        );

        let mix_bindgroup = create_mix_bindgroup(
            &runtime.device,
            &mix_pipeline,
            &slab_buffer,
            hdr_texture,
            &textures[0],
        );

        Self {
            slab,
            downsample_pixel_sizes,
            downsample_pipeline,
            upsample_filter_radius,
            upsample_pipeline,
            textures: Arc::new(RwLock::new(textures)),
            bindgroups: Arc::new(RwLock::new(bindgroups)),
            hdr_texture_downsample_bindgroup: Arc::new(RwLock::new(
                hdr_texture_downsample_bindgroup,
            )),
            mix_pipeline,
            mix_texture: Arc::new(RwLock::new(mix_texture)),
            mix_bindgroup: Arc::new(RwLock::new(mix_bindgroup)),
            mix_strength,
        }
    }

    pub fn set_mix_strength(&self, strength: f32) {
        self.mix_strength.set(strength);
    }

    pub fn get_mix_strength(&self) -> f32 {
        self.mix_strength.get()
    }

    /// Set the filter radius in pixels.
    pub fn set_filter_radius(&self, filter_radius: f32) {
        let size = self.get_size();
        let filter_radius = Vec2::new(filter_radius / size.x as f32, filter_radius / size.y as f32);
        self.upsample_filter_radius.set(filter_radius);
    }

    pub fn get_filter_radius(&self) -> Vec2 {
        self.upsample_filter_radius.get()
    }

    pub fn get_size(&self) -> UVec2 {
        let mix_texture = self.get_mix_texture();
        UVec2::new(mix_texture.width(), mix_texture.height())
    }

    /// Recreates this bloom using the new HDR texture.
    pub fn set_hdr_texture(&self, runtime: impl AsRef<WgpuRuntime>, hdr_texture: &Texture) {
        // UNWRAP: panic on purpose (here and on till the end of this fn)
        let slab_buffer = self.slab.get_buffer().unwrap();
        let resolution = UVec2::new(hdr_texture.width(), hdr_texture.height());
        let runtime = runtime.as_ref();
        let textures = create_textures(runtime, resolution);

        *self.bindgroups.write().unwrap() = create_bindgroups(
            &runtime.device,
            &self.downsample_pipeline,
            &slab_buffer,
            &textures,
        );
        *self.hdr_texture_downsample_bindgroup.write().unwrap() = create_bindgroup(
            &runtime.device,
            &self.downsample_pipeline.get_bind_group_layout(0),
            &slab_buffer,
            hdr_texture,
        );
        *self.mix_texture.write().unwrap() = create_texture(
            runtime,
            resolution.x,
            resolution.y,
            Some("bloom mix"),
            wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
        );
        *self.mix_bindgroup.write().unwrap() = create_mix_bindgroup(
            &runtime.device,
            &self.mix_pipeline,
            &slab_buffer,
            hdr_texture,
            &textures[0],
        );
        *self.textures.write().unwrap() = textures;
    }

    /// Returns a clone of the current mix texture.
    ///
    /// The mix texture is the result of mixing the bloom by the hdr using the
    /// mix strength.
    pub fn get_mix_texture(&self) -> Texture {
        // UNWRAP: not safe but we want to panic
        self.mix_texture.read().unwrap().clone()
    }

    pub(crate) fn render_downsamples(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        struct DownsampleItem<'a> {
            view: &'a wgpu::TextureView,
            bindgroup: &'a wgpu::BindGroup,
            pixel_size: Id<Vec2>,
        }
        // Get all the bindgroups (which are what we're reading from),
        // starting with the hdr frame.
        // Since `bindgroups` are one element greater (we pushed `hdr_texture_bindgroup`
        // to the front) the last bindgroup will not be used, which is good - we
        // don't need to read from the smallest texture during downsampling.
        // UNWRAP: not safe but we want to panic
        let textures_guard = self.textures.read().unwrap();
        let hdr_texture_downsample_bindgroup_guard =
            self.hdr_texture_downsample_bindgroup.read().unwrap();
        let hdr_texture_downsample_bindgroup: &wgpu::BindGroup =
            &hdr_texture_downsample_bindgroup_guard;
        let bindgroups_guard = self.bindgroups.read().unwrap();
        let bindgroups =
            std::iter::once(hdr_texture_downsample_bindgroup).chain(bindgroups_guard.iter());
        let items = textures_guard
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
                render_pass.set_bind_group(0, Some(bindgroup), &[]);
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
        // UNWRAP: not safe but we want to panic
        let bindgroups_guard = self.bindgroups.read().unwrap();
        let bindgroups = bindgroups_guard.iter().rev();
        // Get all the texture views (which are what we're writing to),
        // starting with the second-to-last mip.
        let textures_guard = self.textures.read().unwrap();
        let views = textures_guard.iter().rev().skip(1).map(|t| &t.view);
        let items = bindgroups
            .zip(views)
            .map(|(bindgroup, view)| UpsampleItem { view, bindgroup });
        for (i, UpsampleItem { view, bindgroup }) in items.enumerate() {
            let title = format!("bloom upsample {}", textures_guard.len() - i - 1);
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
                render_pass.set_bind_group(0, Some(bindgroup), &[]);
                let id = self.upsample_filter_radius.id().into();
                render_pass.draw(0..6, id..id + 1);
            }
            queue.submit(std::iter::once(encoder.finish()));
        }
    }

    fn render_mix(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let label = Some("bloom mix");
        // UNWRAP: not safe but we want to panic
        let mix_texture = self.mix_texture.read().unwrap();
        let mix_bindgroup = self.mix_bindgroup.read().unwrap();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &mix_texture.view,
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
            render_pass.set_bind_group(0, Some(mix_bindgroup.deref()), &[]);
            let id = self.mix_strength.id().into();
            render_pass.draw(0..6, id..id + 1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn bloom(&self, device: &wgpu::Device, queue: &wgpu::Queue) {
        assert!(
            self.slab.upkeep().is_none(),
            "bloom slab buffer should never resize"
        );
        self.render_downsamples(device, queue);
        self.render_upsamples(device, queue);
        self.render_mix(device, queue);
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;

    use crate::{camera::Camera, Context};

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
                UVec2::new(32, 25),
                UVec2::new(16, 12),
                UVec2::new(8, 6),
                UVec2::new(4, 3),
                UVec2::new(2, 1),
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
                Vec2::new(0.03125, 0.04),
                Vec2::new(0.0625, 0.083333336),
                Vec2::new(0.125, 0.16666667),
                Vec2::new(0.25, 0.33333334),
                Vec2::new(0.5, 1.0)
            ],
            pixel_sizes
        );
    }

    #[test]
    fn bloom_sanity() {
        let width = 256;
        let height = 128;
        let ctx = Context::headless(width, height);
        let mut stage = ctx.new_stage().with_bloom(false);
        // .with_frustum_culling(false)
        // .with_occlusion_culling(false);

        let projection = crate::camera::perspective(width as f32, height as f32);
        let view = crate::camera::look_at(Vec3::new(0.0, 2.0, 18.0), Vec3::ZERO, Vec3::Y);
        let camera = stage.new_value(Camera::new(projection, view));
        let skybox = stage
            .new_skybox_from_path("../../img/hdr/night.hdr", camera.id())
            .unwrap();
        stage.set_skybox(skybox);

        let _doc = stage
            .load_gltf_document_from_path("../../gltf/EmissiveStrengthTest.glb", camera.id())
            .unwrap();

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("bloom/without.png", img);
        frame.present();

        // now render the whole thing with default values
        stage.set_has_bloom(true);
        stage.set_bloom_mix_strength(0.1);
        stage.set_bloom_filter_radius(2.0);
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("bloom/with.png", img);
    }
}

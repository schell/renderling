//! Tonemapping.
use core::ops::Deref;
use craballoc::{
    prelude::{Hybrid, SlabAllocator},
    runtime::WgpuRuntime,
};
use std::sync::{Arc, RwLock};

use crate::texture::Texture;

use super::TonemapConstants;

pub fn bindgroup_layout(device: &wgpu::Device, label: Option<&str>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label,
        entries: &[
            // slab
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
            // hdr texture
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
            // hdr sampler
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

pub fn create_bindgroup(
    device: &wgpu::Device,
    label: Option<&str>,
    hdr_texture: &Texture,
    slab_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label,
        layout: &bindgroup_layout(device, label),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: slab_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&hdr_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&hdr_texture.sampler),
            },
        ],
    })
}

/// Conducts HDR tone mapping.
///
/// Writes the HDR surface texture to the (most likely) sRGB window surface.
///
/// Clones of [`Tonemapping`] all reference the same internal data.
///
/// ## Note
/// Only available on CPU. Not Available in shaders.
#[derive(Clone)]
pub struct Tonemapping {
    slab: SlabAllocator<WgpuRuntime>,
    config: Hybrid<TonemapConstants>,
    hdr_texture: Arc<RwLock<Texture>>,
    bindgroup: Arc<RwLock<wgpu::BindGroup>>,
    pipeline: Arc<wgpu::RenderPipeline>,
}

impl Tonemapping {
    pub fn new(
        runtime: &WgpuRuntime,
        frame_texture_format: wgpu::TextureFormat,
        hdr_texture: &Texture,
    ) -> Self {
        let slab = SlabAllocator::new(runtime, "tonemapping-slab", wgpu::BufferUsages::empty());
        let config = slab.new_value(TonemapConstants::default());

        let label = Some("tonemapping");
        let slab_buffer = slab.commit();
        let bindgroup = Arc::new(RwLock::new(create_bindgroup(
            &runtime.device,
            label,
            hdr_texture,
            &slab_buffer,
        )));

        let device = &runtime.device;
        let vertex_linkage = crate::linkage::tonemapping_vertex::linkage(device);
        let fragment_linkage = crate::linkage::tonemapping_fragment::linkage(device);
        let hdr_layout = bindgroup_layout(device, label);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&hdr_layout],
            push_constant_ranges: &[],
        });
        let pipeline = Arc::new(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
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
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_linkage.module,
                    entry_point: Some(fragment_linkage.entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: frame_texture_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            }),
        );
        Self {
            slab,
            config,
            hdr_texture: Arc::new(RwLock::new(hdr_texture.clone())),
            bindgroup,
            pipeline,
        }
    }

    pub fn set_hdr_texture(&self, device: &wgpu::Device, hdr_texture: &Texture) {
        // UNWRAP: safe because the buffer is created in `Self::new` and guaranteed to
        // exist
        let slab_buffer = self.slab.get_buffer().unwrap();
        let bindgroup = create_bindgroup(device, Some("tonemapping"), hdr_texture, &slab_buffer);
        // UNWRAP: not safe but we want to panic
        *self.bindgroup.write().unwrap() = bindgroup;
        *self.hdr_texture.write().unwrap() = hdr_texture.clone();
    }

    pub fn get_tonemapping_config(&self) -> TonemapConstants {
        self.config.get()
    }

    pub fn set_tonemapping_config(&self, config: TonemapConstants) {
        self.config.set(config);
    }

    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue, view: &wgpu::TextureView) {
        let label = Some("tonemapping render");
        assert!(!self.slab.commit().is_new_this_commit());

        // UNWRAP: not safe but we want to panic
        let bindgroup = self.bindgroup.read().unwrap();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, Some(bindgroup.deref()), &[]);
        let id = self.config.id().into();
        render_pass.draw(0..6, id..id + 1);
        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));
    }
}

//! CPU-side compositor for alpha-blending a source texture onto a target.
use std::sync::{Arc, RwLock};

use wgsl_rs_linkage_wgpu::{analyze_wgsl_module, WgpuLinkage};

use super::shader::WGSL_SOURCE;

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
    sampler: wgpu::Sampler,
    linkage: Arc<RwLock<WgpuLinkage>>,
}

impl Compositor {
    /// Create a new compositor targeting the given texture format.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let mut linkage = analyze_wgsl_module(&WGSL_SOURCE).expect("compositor wgsl analysis");
        let wgsl_module = linkage.shader_module(device);
        let pipeline_layout = linkage.pipeline_layout(device, Some("compositor"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            multiview_mask: None,
            label: Some("compositor"),
            layout: Some(&pipeline_layout),
            vertex: linkage
                .vertex_entry("compositor_vertex")
                .expect("compositor_vertex entry present")
                .vertex_state(&wgsl_module),
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
                module: &wgsl_module,
                entry_point: Some("compositor_fragment"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            cache: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("compositor"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            pipeline,
            sampler,
            linkage: Arc::new(RwLock::new(linkage)),
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
        let bindgroup = {
            let mut linkage = self.linkage.write().expect("compositor linkage write");
            linkage
                .create_bind_group_named(
                    0,
                    device,
                    &[
                        ("TEXTURE", wgpu::BindingResource::TextureView(source)),
                        ("SAMPLER", wgpu::BindingResource::Sampler(&self.sampler)),
                    ],
                )
                .expect("compositor bind group")
        };

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compositor"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                multiview_mask: None,
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

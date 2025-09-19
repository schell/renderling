//! CPU side of BRDF stuff.
use craballoc::runtime::WgpuRuntime;

use crate::texture;

/// Pre-computed texture of the brdf integration.
#[derive(Clone)]
pub struct BrdfLut {
    pub(crate) inner: texture::Texture,
}

impl BrdfLut {
    /// Create a new pre-computed BRDF look-up texture.
    pub fn new(runtime: impl AsRef<WgpuRuntime>) -> Self {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        let queue = &runtime.queue;
        let vertex_linkage = crate::linkage::brdf_lut_convolution_vertex::linkage(device);
        let fragment_linkage = crate::linkage::brdf_lut_convolution_fragment::linkage(device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("brdf_lut_convolution"),
            layout: None,
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
                    format: wgpu::TextureFormat::Rg16Float,
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

        let framebuffer = texture::Texture::new_with(
            runtime,
            Some("brdf_lut"),
            Some(
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
            ),
            None,
            wgpu::TextureFormat::Rg16Float,
            2,
            2,
            512,
            512,
            1,
            &[],
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("brdf_lut_convolution"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &framebuffer.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            render_pass.set_pipeline(&pipeline);
            render_pass.draw(0..6, 0..1);
        }
        queue.submit([encoder.finish()]);

        BrdfLut { inner: framebuffer }
    }

    /// Return the underlying [`Texture`](crate::texture::Texture).
    pub fn texture(&self) -> &texture::Texture {
        &self.inner
    }
}

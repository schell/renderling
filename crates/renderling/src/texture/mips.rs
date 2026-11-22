//! Mip-map generation.

use crate::texture::Texture;
use craballoc::runtime::WgpuRuntime;
use snafu::Snafu;
use wgsl_rs_linkage_wgpu::{IrModuleExt, WgpuLinkage};

use super::wgpu_texture_format_channels_and_subpixel_bytes_todo;

const LABEL: Option<&str> = Some("mip-map-generator");

#[derive(Debug, Snafu)]
pub enum MipMapError {
    #[snafu(display("Texture format does not match, expected '{expected:?}' but saw '{seen:?}'"))]
    TextureMismatch {
        expected: wgpu::TextureFormat,
        seen: wgpu::TextureFormat,
    },
}

pub struct MipMapGenerator {
    format: wgpu::TextureFormat,
    pipeline: wgpu::RenderPipeline,
    linkage: WgpuLinkage,
}

impl MipMapGenerator {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let mut linkage = crate::convolution::mipmap::instantiate().generate_linkage();
        let pp_layout = linkage.pipeline_layout(device, LABEL);
        let pipeline = {
            let module = linkage.shader_module(device);
            let vertex = linkage
                .vertex_entry("generate_mipmap_vertex")
                .expect("mipmap vertex")
                .vertex_state(&module);
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                multiview_mask: None,
                label: LABEL,
                layout: Some(&pp_layout),
                vertex,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Cw,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("generate_mipmap_fragment"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::all(),
                    })],
                    compilation_options: Default::default(),
                }),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                cache: None,
            })
        };
        Self {
            format,
            pipeline,
            linkage,
        }
    }

    /// Generate mip maps.
    ///
    /// # Errs
    /// Errors if the texture's format doesn't match the generator format.
    pub fn generate(
        &mut self,
        runtime: impl AsRef<WgpuRuntime>,
        texture: &Texture,
        mip_levels: u32,
    ) -> Result<Vec<Texture>, MipMapError> {
        snafu::ensure!(
            texture.texture.format() == self.format,
            TextureMismatchSnafu {
                expected: self.format,
                seen: texture.texture.format()
            }
        );

        let mip_levels = 1.max(mip_levels);
        let (color_channels, subpixel_bytes) =
            wgpu_texture_format_channels_and_subpixel_bytes_todo(self.format);

        let size = texture.texture.size();
        let mut mips: Vec<Texture> = vec![];

        for mip_level in 1..mip_levels {
            let mip_width = size.width >> mip_level;
            let mip_height = size.height >> mip_level;
            let mip_texture = Texture::new_with(
                runtime.as_ref(),
                Some(&format!("mip{mip_level}")),
                Some(
                    wgpu::TextureUsages::COPY_SRC
                        | wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                ),
                None,
                self.format,
                color_channels,
                subpixel_bytes,
                mip_width,
                mip_height,
                1,
                &[],
            );
            let prev_texture = if mip_level == 1 {
                texture
            } else {
                &mips[(mip_level - 2) as usize]
            };
            let bindgroup = self
                .linkage
                .create_bind_group_named(
                    0,
                    &runtime.as_ref().device,
                    &[
                        (
                            "TEXTURE",
                            wgpu::BindingResource::TextureView(&prev_texture.view),
                        ),
                        (
                            "SAMPLER",
                            wgpu::BindingResource::Sampler(&prev_texture.sampler),
                        ),
                    ],
                )
                .expect("mipmap bindgroup");

            let mut encoder = runtime
                .as_ref()
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    multiview_mask: None,
                    label: Some(&format!("mip{mip_level}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &mip_texture.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, Some(&bindgroup), &[]);
                render_pass.draw(0..6, 0..1);
            }

            runtime
                .as_ref()
                .queue
                .submit(std::iter::once(encoder.finish()));

            mips.push(mip_texture);
        }
        Ok(mips)
    }
}

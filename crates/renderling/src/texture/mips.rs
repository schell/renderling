//! Mip-map generation.

use crate::texture::Texture;
use snafu::Snafu;

use super::wgpu_texture_format_channels_and_subpixel_bytes;

const LABEL: Option<&str> = Some("mip-map-generator");

#[derive(Debug, Snafu)]
pub enum MipMapError {
    #[snafu(display("Texture format does not match, expected '{expected:?}' but saw '{seen:?}'"))]
    TextureMismatch {
        expected: wgpu::TextureFormat,
        seen: wgpu::TextureFormat,
    },
}

fn create_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    pp_layout: &wgpu::PipelineLayout,
) -> wgpu::RenderPipeline {
    let vertex_linkage = crate::linkage::generate_mipmap_vertex::linkage(device);
    let fragment_linkage = crate::linkage::generate_mipmap_fragment::linkage(device);
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: LABEL,
        layout: Some(pp_layout),
        vertex: wgpu::VertexState {
            module: &vertex_linkage.module,
            entry_point: Some(vertex_linkage.entry_point),
            buffers: &[],
            compilation_options: Default::default(),
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Cw,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_linkage.module,
            entry_point: Some(fragment_linkage.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::all(),
            })],
            compilation_options: Default::default(),
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

pub struct MipMapGenerator {
    format: wgpu::TextureFormat,
    pipeline: wgpu::RenderPipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
}

impl MipMapGenerator {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: LABEL,
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
        let pp_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: LABEL,
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        let pipeline = create_pipeline(device, format, &pp_layout);
        Self {
            format,
            pipeline,
            bindgroup_layout: bg_layout,
        }
    }

    /// Generate mip maps.
    ///
    /// # Errs
    /// Errors if the texture's format doesn't match the generator format.
    pub fn generate(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
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
            wgpu_texture_format_channels_and_subpixel_bytes(self.format);

        let size = texture.texture.size();
        let mut mips: Vec<Texture> = vec![];

        for mip_level in 1..mip_levels {
            let mip_width = size.width >> mip_level;
            let mip_height = size.height >> mip_level;
            let mip_texture = Texture::new_with(
                device,
                queue,
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
            let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: LABEL,
                layout: &self.bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&prev_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&prev_texture.sampler),
                    },
                ],
            });

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("mip{mip_level}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &mip_texture.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    ..Default::default()
                });

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, Some(&bindgroup), &[]);
                render_pass.draw(0..6, 0..1);
            }

            queue.submit(std::iter::once(encoder.finish()));

            mips.push(mip_texture);
        }
        Ok(mips)
    }
}

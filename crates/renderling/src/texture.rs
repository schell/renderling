//! Creating and using CPU-side textures.
use std::{ops::Deref, sync::Arc};

use image::{
    load_from_memory, ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
    ImageError, PixelWithColorType, Rgba32FImage,
};
use snafu::prelude::*;

use crate::atlas::{AtlasImage, AtlasImageFormat};

#[derive(Debug, Snafu)]
/// Enumeration of errors produced by [`Texture`].
pub enum TextureError {
    #[snafu(display("Unable to load '{}' image from memory: {}", label, source))]
    Loading { source: ImageError, label: String },

    #[snafu(display("Image buffer '{}' unsupported color type: {:?}", label, color_type))]
    UnsupportedColorType {
        color_type: ColorType,
        label: String,
    },

    #[snafu(display("Could not map buffer"))]
    CouldNotMapBuffer { source: wgpu::BufferAsyncError },

    #[snafu(display("Could not convert image buffer"))]
    CouldNotConvertImageBuffer,

    #[snafu(display("Could not create an image buffer"))]
    CouldNotCreateImageBuffer,

    #[snafu(display("Unsupported format"))]
    UnsupportedFormat,
}

type Result<T, E = TextureError> = std::result::Result<T, E>;

pub fn wgpu_texture_format_channels_and_subpixel_bytes(format: wgpu::TextureFormat) -> (u32, u32) {
    match format {
        wgpu::TextureFormat::Rg16Float => (2, 2),
        wgpu::TextureFormat::Rgba16Float => (4, 2),
        wgpu::TextureFormat::Rgba32Float => (4, 4),
        wgpu::TextureFormat::Rgba8UnormSrgb => (4, 1),
        _ => todo!("temporarily unsupported format '{format:?}'"),
    }
}

/// A texture living on the GPU.
#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
}

impl Texture {
    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    /// Create a cubemap texture from 6 faces.
    pub fn new_cubemap_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        texture_size: u32,
        face_textures: &[Texture],
        image_format: wgpu::TextureFormat,
        mip_levels: u32,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: texture_size,
            height: texture_size,
            depth_or_array_layers: 6,
        };
        let cubemap_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: mip_levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: image_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("texture_buffer_copy_encoder"),
        });

        for i in 0..6 {
            for mip_level in 0..mip_levels as usize {
                let mip_size = texture_size >> mip_level;
                let index = i * mip_levels as usize + mip_level;
                let texture = &face_textures[index].texture;
                encoder.copy_texture_to_texture(
                    wgpu::ImageCopyTexture {
                        texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::ImageCopyTexture {
                        texture: &cubemap_texture,
                        mip_level: mip_level as u32,
                        origin: wgpu::Origin3d {
                            x: 0,
                            y: 0,
                            z: i as u32,
                        },
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: mip_size,
                        height: mip_size,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }
        queue.submit([encoder.finish()]);

        let view = cubemap_texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            label,
            ..Default::default()
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            label,
            ..Default::default()
        });

        Texture {
            texture: cubemap_texture.into(),
            view: view.into(),
            sampler: sampler.into(),
        }
    }

    /// Create a new texture.
    pub fn new_with(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        sampler: Option<wgpu::Sampler>,
        format: wgpu::TextureFormat,
        color_channels: u32,
        color_channel_bytes: u32,
        width: u32,
        height: u32,
        mip_level_count: u32,
        data: &[u8],
    ) -> Self {
        let mip_level_count = 1.max(mip_level_count);
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: usage
                .unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
            view_formats: &[],
        });

        if !data.is_empty() {
            queue.write_texture(
                wgpu::ImageCopyTextureBase {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(color_channels * color_channel_bytes * width),
                    rows_per_image: None,
                },
                size,
            );
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = sampler.unwrap_or_else(|| {
            device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            })
        });

        Texture {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
    }

    /// Create a new texture.
    ///
    /// This defaults the format to `Rgba8UnormSrgb` and assumes a pixel is 1
    /// byte per channel.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        color_channels: u32,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Self {
        Self::new_with(
            device,
            queue,
            label,
            usage,
            None,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            color_channels,
            1,
            width,
            height,
            1,
            data,
        )
    }

    pub fn from_image_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = load_from_memory(bytes).with_context(|_| LoadingSnafu {
            label: label.to_string(),
        })?;

        match img {
            DynamicImage::ImageLuma8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None, None)
            }
            DynamicImage::ImageLumaA8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None, None)
            }
            DynamicImage::ImageRgb8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None, None)
            }
            DynamicImage::ImageRgba8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None, None)
            }
            img => Self::from_image_buffer(device, queue, &img.to_rgba8(), Some(label), None, None),
        }
    }

    pub fn from_dynamic_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        dyn_img: image::DynamicImage,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        mip_level_count: u32,
    ) -> Self {
        let mip_level_count = mip_level_count.max(1);
        let dimensions = dyn_img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let (img, format, channels) = match dyn_img {
            img @ DynamicImage::ImageLuma8(_) => (img, wgpu::TextureFormat::R8Unorm, 1),
            img @ DynamicImage::ImageRgba8(_) => (img, wgpu::TextureFormat::Rgba8UnormSrgb, 4),
            img @ DynamicImage::ImageLuma16(_) => (img, wgpu::TextureFormat::R16Unorm, 1),
            img @ DynamicImage::ImageRgba16(_) => (img, wgpu::TextureFormat::Rgba16Unorm, 4),
            img @ DynamicImage::ImageRgba32F(_) => (img, wgpu::TextureFormat::Rgba32Float, 4),
            img => {
                let rgba8 = DynamicImage::ImageRgba8(img.into_rgba8());
                (rgba8, wgpu::TextureFormat::Rgba8UnormSrgb, 4)
            }
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: usage
                .unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            img.as_bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(channels * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        Self::from_wgpu_tex(device, texture, None, None)
    }

    pub fn from_image_buffer<P>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &ImageBuffer<P, Vec<u8>>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        mip_level_count: Option<u32>,
    ) -> Result<Self>
    where
        P: PixelWithColorType,
        ImageBuffer<P, Vec<u8>>: GenericImage + Deref<Target = [u8]>,
    {
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: {
                ensure!(
                    P::COLOR_TYPE == ColorType::Rgba8,
                    UnsupportedColorTypeSnafu {
                        color_type: P::COLOR_TYPE,
                        label: label
                            .map(ToString::to_string)
                            .unwrap_or_else(|| "unknown".to_string()),
                    }
                );
                wgpu::TextureFormat::Rgba8UnormSrgb
            },
            usage: usage
                .unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            img.deref(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(P::CHANNEL_COUNT as u32 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        Ok(Self::from_wgpu_tex(device, texture, None, mip_level_count))
    }

    pub fn from_wgpu_tex(
        device: &wgpu::Device,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
        mip_level_count: Option<u32>,
    ) -> Self {
        let texture = texture.into();
        let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("texture view"),
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count,
            base_array_layer: 0,
            array_layer_count: None,
        }));
        let sampler_descriptor = sampler.unwrap_or_else(|| wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let sampler = Arc::new(device.create_sampler(&sampler_descriptor));

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
    }

    /// Read the texture from the GPU.
    ///
    /// To read the texture you must provide the width, height, the number of
    /// color/alpha channels and the number of bytes in the underlying
    /// subpixel type (usually u8=1, u16=2 or f32=4).
    pub fn read(
        texture: &wgpu::Texture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: usize,
        height: usize,
        channels: usize,
        subpixel_bytes: usize,
    ) -> CopiedTextureBuffer {
        Self::read_from(
            texture,
            device,
            queue,
            width,
            height,
            channels,
            subpixel_bytes,
            0,
            None,
        )
    }

    /// Read the texture from the GPU.
    ///
    /// To read the texture you must provide the width, height, the number of
    /// color/alpha channels and the number of bytes in the underlying
    /// subpixel type (usually u8=1, u16=2 or f32=4).
    pub fn read_from(
        texture: &wgpu::Texture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: usize,
        height: usize,
        channels: usize,
        subpixel_bytes: usize,
        mip_level: u32,
        origin: Option<wgpu::Origin3d>,
    ) -> CopiedTextureBuffer {
        let dimensions = BufferDimensions::new(channels, subpixel_bytes, width, height);
        // The output buffer lets us retrieve the self as an array
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Texture::read buffer"),
            size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("post render screen capture encoder"),
        });
        let mut source = texture.as_image_copy();
        source.mip_level = mip_level;
        if let Some(origin) = origin {
            source.origin = origin;
        }
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            source,
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(dimensions.padded_bytes_per_row as u32),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: dimensions.width as u32,
                height: dimensions.height as u32,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(std::iter::once(encoder.finish()));

        CopiedTextureBuffer {
            dimensions,
            buffer,
            format: texture.format(),
        }
    }

    pub fn read_hdr_image(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Rgba32FImage, TextureError> {
        let width = self.width();
        let height = self.height();
        let copied = Texture::read(
            &self.texture,
            &device,
            &queue,
            width as usize,
            height as usize,
            4,
            2,
        );

        let pixels = copied.pixels(device);
        let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
            .iter()
            .map(|p| half::f16::from_bits(*p).to_f32())
            .collect::<Vec<_>>();
        assert_eq!((width * height * 4) as usize, pixels.len());
        let img: image::Rgba32FImage = image::ImageBuffer::from_vec(width, height, pixels)
            .context(CouldNotCreateImageBufferSnafu)?;
        Ok(img)
    }

    /// Generate `mipmap_levels - 1` mipmaps for the given texture.
    ///
    /// ## Note
    /// Ensure that `self` only has one mip level. If not it will try to sample
    /// from an empty mip.
    pub fn generate_mips(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        mip_levels: u32,
    ) -> Vec<Self> {
        let mip_levels = 1.max(mip_levels);
        let (color_channels, subpixel_bytes) =
            wgpu_texture_format_channels_and_subpixel_bytes(self.texture.format());

        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
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
            label,
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });
        let vertex_linkage = crate::linkage::generate_mipmap_vertex::linkage(device);
        let fragment_linkage = crate::linkage::generate_mipmap_fragment::linkage(device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pp_layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: vertex_linkage.entry_point,
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Cw,
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: fragment_linkage.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.texture.format(),
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        let size = self.texture.size();
        let mut mips: Vec<Texture> = vec![];

        for mip_level in 1..mip_levels {
            let mip_width = size.width >> mip_level;
            let mip_height = size.height >> mip_level;
            let mip_texture = Self::new_with(
                device,
                queue,
                Some(&format!("mip{mip_level}")),
                Some(
                    wgpu::TextureUsages::COPY_SRC
                        | wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                ),
                None,
                self.texture.format(),
                color_channels,
                subpixel_bytes,
                mip_width,
                mip_height,
                1,
                &[],
            );
            let prev_texture = if mip_level == 1 {
                &self
            } else {
                &mips[(mip_level - 2) as usize]
            };
            let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label,
                layout: &bg_layout,
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

                render_pass.set_pipeline(&pipeline);
                render_pass.set_bind_group(0, &bindgroup, &[]);
                render_pass.draw(0..6, 0..1);
            }

            queue.submit(std::iter::once(encoder.finish()));

            mips.push(mip_texture);
        }
        mips
    }

    pub const HDR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

    /// Create a new HDR texture.
    pub fn create_hdr_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) -> Texture {
        Texture::new_with(
            &device,
            &queue,
            Some("hdr"),
            Some(
                // * The hdr texture is what we render to in most cases
                // * we also read from it to calculate bloom
                // * we also write the bloom mix result back to it
                // * we also read the texture in tests
                wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::COPY_SRC,
            ),
            Some({
                device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                })
            }),
            Self::HDR_TEXTURE_FORMAT,
            4,
            // TODO: pretty sure this should be `2`
            1,
            width,
            height,
            1,
            &[],
        )
    }
}

/// A depth texture.
pub struct DepthTexture {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,
    pub(crate) texture: Texture,
}

impl Deref for DepthTexture {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl DepthTexture {
    pub fn width(&self) -> u32 {
        self.texture.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.texture.height()
    }

    /// Converts the depth texture into an image.
    ///
    /// Assumes the format is single channel 32bit.
    pub fn read_image(&self) -> Option<image::DynamicImage> {
        let depth_copied_buffer = Texture::read(
            &self.texture.texture,
            &self.device,
            &self.queue,
            self.width() as usize,
            self.height() as usize,
            1,
            4,
        );
        let pixels = depth_copied_buffer.pixels(&self.device);
        let pixels: Vec<f32> = bytemuck::cast_slice(&pixels).to_vec();
        let img_buffer: image::ImageBuffer<image::Luma<f32>, Vec<f32>> =
            image::ImageBuffer::from_raw(self.width(), self.height(), pixels)?;
        Some(image::DynamicImage::from(img_buffer))
    }
}

/// Helper for retreiving an image from a texture.
pub struct BufferDimensions {
    pub width: usize,
    pub height: usize,
    pub unpadded_bytes_per_row: usize,
    pub padded_bytes_per_row: usize,
}

impl BufferDimensions {
    pub fn new(channels: usize, subpixel_bytes: usize, width: usize, height: usize) -> Self {
        let bytes_per_pixel = channels * subpixel_bytes;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

/// Helper for retreiving a rendered frame.
pub struct CopiedTextureBuffer {
    pub format: wgpu::TextureFormat,
    pub dimensions: BufferDimensions,
    pub buffer: wgpu::Buffer,
}

impl CopiedTextureBuffer {
    /// Access the raw unpadded pixels of the buffer.
    pub fn pixels(&self, device: &wgpu::Device) -> Vec<u8> {
        let buffer_slice = self.buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::Maintain::Wait);

        let padded_buffer = buffer_slice.get_mapped_range();
        let mut unpadded_buffer = vec![];
        // from the padded_buffer we write just the unpadded bytes into the
        // unpadded_buffer
        for chunk in padded_buffer.chunks(self.dimensions.padded_bytes_per_row) {
            unpadded_buffer.extend_from_slice(&chunk[..self.dimensions.unpadded_bytes_per_row]);
        }
        unpadded_buffer
    }

    /// Convert the post render buffer into an RgbaImage.
    pub async fn convert_to_rgba(self) -> Result<image::RgbaImage, TextureError> {
        let buffer_slice = self.buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, {
            move |result| {
                tx.send(result).unwrap();
            }
        });
        loop {
            if let Ok(result) = rx.try_recv() {
                result.context(CouldNotMapBufferSnafu)?;
                break;
            } else {
                futures_lite::future::yield_now().await;
            }
        }

        let padded_buffer = buffer_slice.get_mapped_range();
        let mut unpadded_buffer = vec![];
        // from the padded_buffer we write just the unpadded bytes into the
        // unpadded_buffer
        for chunk in padded_buffer.chunks(self.dimensions.padded_bytes_per_row) {
            unpadded_buffer.extend_from_slice(&chunk[..self.dimensions.unpadded_bytes_per_row]);
        }
        let mut img_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(
                self.dimensions.width as u32,
                self.dimensions.height as u32,
                unpadded_buffer,
            )
            .context(CouldNotConvertImageBufferSnafu)?;
        if self.format.is_srgb() {
            log::trace!("converting applying linear transfer to srgb pixels");
            // Convert back to linear
            img_buffer.pixels_mut().for_each(|p| {
                crate::color::linear_xfer_u8(&mut p.0[0]);
                crate::color::linear_xfer_u8(&mut p.0[1]);
                crate::color::linear_xfer_u8(&mut p.0[2]);
                crate::color::linear_xfer_u8(&mut p.0[3]);
            });
        }
        Ok(image::DynamicImage::ImageRgba8(img_buffer).to_rgba8())
    }

    /// Convert the post render buffer into an image.
    pub fn into_image<P>(self, device: &wgpu::Device) -> Result<image::DynamicImage, TextureError>
    where
        P: image::Pixel<Subpixel = u8>,
        image::DynamicImage: From<image::ImageBuffer<P, Vec<u8>>>,
    {
        let pixels = self.pixels(device);
        let img_buffer: image::ImageBuffer<P, Vec<u8>> = image::ImageBuffer::from_raw(
            self.dimensions.width as u32,
            self.dimensions.height as u32,
            pixels,
        )
        .context(CouldNotConvertImageBufferSnafu)?;
        Ok(image::DynamicImage::from(img_buffer))
    }

    /// Convert the post render buffer into an internal-format [`AtlasImage`].
    pub fn into_atlas_image(self, device: &wgpu::Device) -> Result<AtlasImage, TextureError> {
        let pixels = self.pixels(device);
        let img = AtlasImage {
            pixels,
            width: self.dimensions.width as u32,
            height: self.dimensions.height as u32,
            format: AtlasImageFormat::from_wgpu_texture_format(self.format)
                .context(UnsupportedFormatSnafu)?,
            apply_linear_transfer: false,
        };
        Ok(img)
    }

    /// Convert the post render buffer into an RgbaImage.
    ///
    /// Ensures that the pixels are in the given color space by applying the
    /// correct transfer function if needed.
    pub fn into_rgba(
        self,
        device: &wgpu::Device,
        // `true` - the resulting image will be in a linear color space
        // `false` - the resulting image will be in an sRGB color space
        linear: bool,
    ) -> Result<image::RgbaImage, TextureError> {
        let format = self.format;
        let mut img_buffer = self.into_image::<image::Rgba<u8>>(device)?.into_rgba8();
        let linear_xfer = format.is_srgb() && linear;
        let opto_xfer = !format.is_srgb() && !linear;
        let should_xfer = linear_xfer || opto_xfer;

        if should_xfer {
            let f = if linear_xfer {
                log::trace!(
                    "converting by applying linear transfer fn to srgb pixels (sRGB -> linear)"
                );
                crate::color::linear_xfer_u8
            } else {
                log::trace!(
                    "converting by applying opto transfer fn to linear pixels (linear -> sRGB)"
                );
                crate::color::opto_xfer_u8
            };
            // Convert back to linear
            img_buffer.pixels_mut().for_each(|p| {
                f(&mut p.0[0]);
                f(&mut p.0[1]);
                f(&mut p.0[2]);
                f(&mut p.0[3]);
            });
        }

        Ok(img_buffer)
    }

    /// Convert the post render buffer into an RgbaImage.
    ///
    /// Ensures that the pixels are in a linear color space by applying the
    /// linear transfer if the texture this buffer was copied from was sRGB.
    pub fn into_linear_rgba(self, device: &wgpu::Device) -> Result<image::RgbaImage, TextureError> {
        let format = self.format;
        let mut img_buffer = self.into_image::<image::Rgba<u8>>(device)?.into_rgba8();
        if format.is_srgb() {
            log::trace!(
                "converting by applying linear transfer fn to srgb pixels (sRGB -> linear)"
            );
            // Convert back to linear
            img_buffer.pixels_mut().for_each(|p| {
                crate::color::linear_xfer_u8(&mut p.0[0]);
                crate::color::linear_xfer_u8(&mut p.0[1]);
                crate::color::linear_xfer_u8(&mut p.0[2]);
                crate::color::linear_xfer_u8(&mut p.0[3]);
            });
        }

        Ok(img_buffer)
    }

    /// Convert the post render buffer into an RgbaImage.
    ///
    /// Ensures that the pixels are in a linear color space by applying the
    /// linear transfer if the texture this buffer was copied from was sRGB.
    pub fn into_srgba(self, device: &wgpu::Device) -> Result<image::RgbaImage, TextureError> {
        let format = self.format;
        let mut img_buffer = self.into_image::<image::Rgba<u8>>(device)?.into_rgba8();
        if !format.is_srgb() {
            log::trace!(
                "converting by applying opto transfer fn to linear pixels (linear -> sRGB)"
            );
            // Convert back to linear
            img_buffer.pixels_mut().for_each(|p| {
                crate::color::opto_xfer_u8(&mut p.0[0]);
                crate::color::opto_xfer_u8(&mut p.0[1]);
                crate::color::opto_xfer_u8(&mut p.0[2]);
                crate::color::opto_xfer_u8(&mut p.0[3]);
            });
        }

        Ok(img_buffer)
    }
}

#[cfg(test)]
mod test {
    use crate::Context;

    use super::Texture;

    #[test]
    fn generate_mipmaps() {
        let r = Context::headless(10, 10);
        let (device, queue) = r.get_device_and_queue_owned();
        let img = image::open("../../img/sandstone.png").unwrap();
        let width = img.width();
        let height = img.height();
        let mip_level_count = 5;
        let mut texture = Texture::from_dynamic_image(
            &device,
            &queue,
            img,
            Some("sandstone"),
            Some(
                wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
            ),
            1,
        );
        let mips = texture.generate_mips(&device, &queue, None, mip_level_count);

        let (channels, subpixel_bytes) =
            super::wgpu_texture_format_channels_and_subpixel_bytes(texture.texture.format());
        for (level, mip) in mips.into_iter().enumerate() {
            let mip_level = level + 1;
            let mip_width = width >> mip_level;
            let mip_height = height >> mip_level;
            // save out the mips
            let copied_buffer = Texture::read_from(
                &mip.texture,
                r.get_device(),
                r.get_queue(),
                mip_width as usize,
                mip_height as usize,
                channels as usize,
                subpixel_bytes as usize,
                0,
                None,
            );
            let pixels = copied_buffer.pixels(r.get_device());
            assert_eq!((mip_width * mip_height * 4) as usize, pixels.len());
            let img: image::RgbaImage =
                image::ImageBuffer::from_vec(mip_width, mip_height, pixels).unwrap();
            let img = image::DynamicImage::from(img);
            let img = img.to_rgba8();
            img_diff::assert_img_eq(&format!("texture/sandstone_mip{mip_level}.png"), img);
        }
    }
}

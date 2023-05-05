//! Creating and using textures.
use snafu::prelude::*;
use std::{ops::Deref, sync::Arc};

#[cfg(feature = "image")]
use image::{
    load_from_memory, ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
    ImageError, PixelWithColorType,
};

#[derive(Debug, Snafu)]
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
}

type Result<T, E = TextureError> = std::result::Result<T, E>;

/// A texture living on the GPU.
#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
}

impl Texture {
    /// Create a new texture.
    pub fn new_with(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        sampler: Option<wgpu::Sampler>,
        format: wgpu::TextureFormat,
        color_channels: u32,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
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
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(color_channels * width),
                rows_per_image: None,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = sampler.unwrap_or_else(|| {
            device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
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
    /// This defaults the format to `Rgba8UnormSrgb`.
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
            width,
            height,
            data,
        )
    }

    #[cfg(feature = "image")]
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
                Self::from_image_buffer(device, queue, &b, Some(label), None)
            }
            DynamicImage::ImageLumaA8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None)
            }
            DynamicImage::ImageRgb8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None)
            }
            DynamicImage::ImageRgba8(b) => {
                Self::from_image_buffer(device, queue, &b, Some(label), None)
            }
            img => Self::from_image_buffer(device, queue, &img.to_rgba8(), Some(label), None),
        }
    }

    #[cfg(feature = "image")]
    pub fn from_image_buffer<P>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &ImageBuffer<P, Vec<u8>>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
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

        Ok(Self::from_wgpu_tex(device, texture, None))
    }

    pub fn from_wgpu_tex(
        device: &wgpu::Device,
        texture: wgpu::Texture,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> Self {
        let texture = Arc::new(texture);
        let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor::default()));
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
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
    /// To read the texture you must provide the width and height.
    pub fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: usize,
        height: usize,
    ) -> CopiedTextureBuffer {
        let dimensions = BufferDimensions::new(width, height);
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
        // Copy the data from the surface texture to the buffer
        encoder.copy_texture_to_buffer(
            self.texture.as_image_copy(),
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

        CopiedTextureBuffer { dimensions, buffer }
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
    pub fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = std::mem::size_of::<u32>();
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
    pub dimensions: BufferDimensions,
    pub buffer: wgpu::Buffer,
}

impl CopiedTextureBuffer {
    #[cfg(feature = "image")]
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
        let img_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(
                self.dimensions.width as u32,
                self.dimensions.height as u32,
                unpadded_buffer,
            )
            .context(CouldNotConvertImageBufferSnafu)?;
        Ok(image::DynamicImage::ImageRgba8(img_buffer).to_rgba8())
    }

    #[cfg(feature = "image")]
    /// Convert the post render buffer into an RgbaImage.
    pub fn into_rgba(self, device: &wgpu::Device) -> Result<image::RgbaImage, TextureError> {
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
        let img_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(
                self.dimensions.width as u32,
                self.dimensions.height as u32,
                unpadded_buffer,
            )
            .context(CouldNotConvertImageBufferSnafu)?;
        Ok(image::DynamicImage::ImageRgba8(img_buffer).to_rgba8())
    }
}

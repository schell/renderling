//! Creating and using textures.
use snafu::prelude::*;
use std::{ops::Deref, sync::Arc};

#[cfg(feature = "image")]
use image::{
    load_from_memory, ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
    ImageError, PixelWithColorType,
};

#[derive(Debug, Snafu)]
pub enum RenderlingError {
    #[snafu(display("Unable to load '{}' image from memory: {}", label, source))]
    Loading { source: ImageError, label: String },

    #[snafu(display("Image buffer '{}' unsupported color type: {:?}", label, color_type))]
    UnsupportedColorType {
        color_type: ColorType,
        label: String,
    },
}

type Result<T, E = RenderlingError> = std::result::Result<T, E>;

/// A texture living on the GPU.
#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
}

impl Texture {
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: usage
                .unwrap_or(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
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
                bytes_per_row: std::num::NonZeroU32::new(color_channels * width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Texture {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
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
                bytes_per_row: std::num::NonZeroU32::new(P::CHANNEL_COUNT as u32 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        Ok(Self::from_wgpu_tex(texture, device))
    }

    pub fn from_wgpu_tex(texture: wgpu::Texture, device: &wgpu::Device) -> Self {
        let texture = Arc::new(texture);
        let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor::default()));
        let sampler = Arc::new(device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }));

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
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture: Arc::new(texture),
            view: Arc::new(view),
            sampler: Arc::new(sampler),
        }
    }
}

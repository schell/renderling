//! Wrapper around [`wgpu::Texture`].
use core::sync::atomic::AtomicUsize;
use std::{
    ops::Deref,
    sync::{Arc, LazyLock},
};

use craballoc::runtime::WgpuRuntime;
use glam::UVec2;
use image::{
    load_from_memory, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageError,
    PixelWithColorType, Rgba32FImage,
};
use mips::MipMapGenerator;
use snafu::prelude::*;

use crate::atlas::{AtlasImage, AtlasImageFormat};

pub mod mips;

#[derive(Debug, Snafu)]
/// Enumeration of errors produced by [`Texture`].
pub enum TextureError {
    #[snafu(display("Unable to load '{}' image from memory: {}", label, source))]
    Loading { source: ImageError, label: String },

    #[snafu(display("Image buffer '{}' unsupported color type: {:?}", label, color_type))]
    UnsupportedColorType {
        color_type: image::ExtendedColorType,
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
        wgpu::TextureFormat::Depth32Float => (1, 4),
        wgpu::TextureFormat::R32Float => (1, 4),
        wgpu::TextureFormat::Rg16Float => (2, 2),
        wgpu::TextureFormat::Rgba8Unorm => (4, 1),
        wgpu::TextureFormat::Rgba16Float => (4, 2),
        wgpu::TextureFormat::Rgba32Float => (4, 4),
        wgpu::TextureFormat::Rgba8UnormSrgb => (4, 1),
        wgpu::TextureFormat::R8Unorm => (1, 1),
        _ => todo!("temporarily unsupported format '{format:?}'"),
    }
}

static NEXT_TEXTURE_ID: LazyLock<Arc<AtomicUsize>> = LazyLock::new(|| Arc::new(0.into()));

pub(crate) fn get_next_texture_id() -> usize {
    NEXT_TEXTURE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// A texture living on the GPU.
#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub sampler: Arc<wgpu::Sampler>,
    pub(crate) id: usize,
}

impl Texture {
    /// Returns the id of this texture.
    ///
    /// The id is a monotonically increasing count of all textures created.
    ///
    /// This can be used to determine if a texture has been
    /// replaced by another, which can be used, for example, to invalidate
    /// a [`wgpu::BindGroup`].
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    pub fn height(&self) -> u32 {
        self.texture.height()
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    /// Create a cubemap texture from 6 faces.
    pub fn new_cubemap_texture(
        runtime: impl AsRef<WgpuRuntime>,
        label: Option<&str>,
        texture_size: u32,
        face_textures: &[Texture],
        image_format: wgpu::TextureFormat,
        mip_levels: u32,
    ) -> Self {
        let WgpuRuntime { device, queue } = runtime.as_ref();
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
                    wgpu::TexelCopyTextureInfo {
                        texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
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
            id: get_next_texture_id(),
        }
    }

    /// Create a new texture.
    #[allow(clippy::too_many_arguments)]
    pub fn new_with(
        runtime: impl AsRef<WgpuRuntime>,
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
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        let queue = &runtime.queue;
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
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::TexelCopyBufferLayout {
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
            id: get_next_texture_id(),
        }
    }

    /// Create a new texture.
    ///
    /// This defaults the format to `Rgba8UnormSrgb` and assumes a pixel is 1
    /// byte per channel.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        runtime: impl AsRef<WgpuRuntime>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        color_channels: u32,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Self {
        let runtime = runtime.as_ref();
        Self::new_with(
            runtime,
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
        runtime: impl AsRef<WgpuRuntime>,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = load_from_memory(bytes).with_context(|_| LoadingSnafu {
            label: label.to_string(),
        })?;

        match img {
            DynamicImage::ImageLuma8(b) => {
                Self::from_image_buffer(runtime, &b, Some(label), None, None)
            }
            DynamicImage::ImageLumaA8(b) => {
                Self::from_image_buffer(runtime, &b, Some(label), None, None)
            }
            DynamicImage::ImageRgb8(b) => {
                Self::from_image_buffer(runtime, &b, Some(label), None, None)
            }
            DynamicImage::ImageRgba8(b) => {
                Self::from_image_buffer(runtime, &b, Some(label), None, None)
            }
            img => Self::from_image_buffer(runtime, &img.to_rgba8(), Some(label), None, None),
        }
    }

    pub fn from_dynamic_image(
        runtime: impl AsRef<WgpuRuntime>,
        dyn_img: image::DynamicImage,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        mip_level_count: u32,
    ) -> Self {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        let queue = &runtime.queue;
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
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            img.as_bytes(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(channels * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        Self::from_wgpu_tex(device, texture, None, None)
    }

    pub fn from_image_buffer<P>(
        runtime: impl AsRef<WgpuRuntime>,
        img: &ImageBuffer<P, Vec<u8>>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
        mip_level_count: Option<u32>,
    ) -> Result<Self>
    where
        P: PixelWithColorType,
        ImageBuffer<P, Vec<u8>>: GenericImage + Deref<Target = [u8]>,
    {
        let runtime = runtime.as_ref();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = runtime.device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: {
                ensure!(
                    P::COLOR_TYPE == image::ExtendedColorType::Rgba8,
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

        runtime.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            img.deref(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(P::CHANNEL_COUNT as u32 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        Ok(Self::from_wgpu_tex(
            &runtime.device,
            texture,
            None,
            mip_level_count,
        ))
    }

    pub fn from_wgpu_tex(
        device: &wgpu::Device,
        texture: impl Into<Arc<wgpu::Texture>>,
        sampler: Option<wgpu::SamplerDescriptor>,
        mip_level_count: Option<u32>,
    ) -> Self {
        let texture = texture.into();
        let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor {
            mip_level_count,
            ..Default::default()
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
            id: get_next_texture_id(),
        }
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        multisample_count: u32,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: multisample_count,
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
            id: get_next_texture_id(),
        }
    }

    pub fn create_depth_texture_for_shadow_map(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        multisample_count: u32,
        label: Option<&str>,
        is_point_light: bool,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: if is_point_light { 6 } else { 1 },
        };
        let desc = wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: multisample_count,
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
            id: get_next_texture_id(),
        }
    }

    /// Read the texture from the GPU.
    ///
    /// To read the texture you must provide the width, height, the number of
    /// color/alpha channels and the number of bytes in the underlying
    /// subpixel type (usually u8=1, u16=2 or f32=4).
    // TODO: remove width and height from these calls, as they can be obtained
    // from Texture::size()
    pub fn read(
        runtime: impl AsRef<WgpuRuntime>,
        texture: &wgpu::Texture,
        width: usize,
        height: usize,
        channels: usize,
        subpixel_bytes: usize,
    ) -> CopiedTextureBuffer {
        Self::read_from(
            runtime,
            texture,
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
    #[allow(clippy::too_many_arguments)]
    pub fn read_from(
        runtime: impl AsRef<WgpuRuntime>,
        texture: &wgpu::Texture,
        width: usize,
        height: usize,
        channels: usize,
        subpixel_bytes: usize,
        mip_level: u32,
        origin: Option<wgpu::Origin3d>,
    ) -> CopiedTextureBuffer {
        let runtime = runtime.as_ref();
        let device = &runtime.device;
        let queue = &runtime.queue;
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
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
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
        runtime: impl AsRef<WgpuRuntime>,
    ) -> Result<Rgba32FImage, TextureError> {
        let runtime = runtime.as_ref();
        let width = self.width();
        let height = self.height();
        let copied = Texture::read(
            runtime,
            &self.texture,
            width as usize,
            height as usize,
            4,
            2,
        );

        let pixels = copied.pixels(&runtime.device);
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
        runtime: impl AsRef<WgpuRuntime>,
        _label: Option<&str>,
        mip_levels: u32,
    ) -> Vec<Self> {
        let runtime = runtime.as_ref();
        let generator = MipMapGenerator::new(&runtime.device, self.texture.format());
        // UNWRAP: safe because we know the formats match.
        generator.generate(runtime, self, mip_levels).unwrap()
    }

    pub const HDR_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

    /// Create a new HDR texture.
    pub fn create_hdr_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        multisample_count: u32,
    ) -> Texture {
        // * The hdr texture is what we render to in most cases
        // * we also read from it to calculate bloom
        // * we also write the bloom mix result back to it
        // * we also read the texture in tests
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC;
        let texture = Arc::new(device.create_texture(&wgpu::TextureDescriptor {
            label: Some("hdr"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: multisample_count,
            dimension: wgpu::TextureDimension::D2,
            format: Self::HDR_TEXTURE_FORMAT,
            usage,
            view_formats: &[],
        }));
        let sampler = Arc::new(device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }));
        let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor::default()));
        Texture {
            texture,
            view,
            sampler,
            id: get_next_texture_id(),
        }
    }
}

pub fn read_depth_texture_to_image(
    runtime: impl AsRef<WgpuRuntime>,
    width: usize,
    height: usize,
    texture: &wgpu::Texture,
) -> Option<image::GrayImage> {
    let depth_copied_buffer = Texture::read(runtime.as_ref(), texture, width, height, 1, 4);
    let pixels = depth_copied_buffer.pixels(&runtime.as_ref().device);
    let pixels = bytemuck::cast_slice::<u8, f32>(&pixels)
        .iter()
        .copied()
        .map(|f| {
            // Depth texture is stored as Depth32Float, but the values are normalized 0.0-1.0
            (255.0 * f) as u8
        })
        .collect::<Vec<u8>>();
    let img_buffer = image::GrayImage::from_raw(width as u32, height as u32, pixels)?;
    Some(img_buffer)
}

/// A depth texture.
pub struct DepthTexture {
    pub(crate) runtime: WgpuRuntime,
    pub(crate) texture: Arc<wgpu::Texture>,
}

impl Deref for DepthTexture {
    type Target = wgpu::Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl DepthTexture {
    pub fn new(runtime: impl AsRef<WgpuRuntime>, texture: impl Into<Arc<wgpu::Texture>>) -> Self {
        Self {
            runtime: runtime.as_ref().clone(),
            texture: texture.into(),
        }
    }

    pub fn try_new_from(
        runtime: impl AsRef<WgpuRuntime>,
        value: Texture,
    ) -> Result<Self, TextureError> {
        if value.texture.format() != wgpu::TextureFormat::Depth32Float {
            return UnsupportedFormatSnafu.fail();
        }

        Ok(Self {
            runtime: runtime.as_ref().clone(),
            texture: value.texture,
        })
    }

    /// Converts the depth texture into an image.
    ///
    /// Assumes the format is single channel 32bit.
    ///
    /// ## Panics
    /// This may panic if the depth texture has a multisample count greater than
    /// 1.
    pub fn read_image(&self) -> Option<image::GrayImage> {
        // TODO: impl AsRef<WgpuRuntime>
        read_depth_texture_to_image(
            &self.runtime,
            self.width() as usize,
            self.height() as usize,
            &self.texture,
        )
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
    ///
    /// `Sp` is the sub-pixel type. eg, `u8` or `f32`
    ///
    /// `P` is the pixel type. eg, `Rgba<u8>` or `Luma<f32>`
    pub fn into_image<Sp, P>(
        self,
        device: &wgpu::Device,
    ) -> Result<image::DynamicImage, TextureError>
    where
        Sp: bytemuck::AnyBitPattern,
        P: image::Pixel<Subpixel = Sp>,
        image::DynamicImage: From<image::ImageBuffer<P, Vec<Sp>>>,
    {
        let pixels = self.pixels(device);
        let coerced_pixels: &[Sp] = bytemuck::cast_slice(&pixels);
        let img_buffer: image::ImageBuffer<P, Vec<Sp>> = image::ImageBuffer::from_raw(
            self.dimensions.width as u32,
            self.dimensions.height as u32,
            coerced_pixels.to_vec(),
        )
        .context(CouldNotConvertImageBufferSnafu)?;
        Ok(image::DynamicImage::from(img_buffer))
    }

    /// Convert the post render buffer into an internal-format [`AtlasImage`].
    pub fn into_atlas_image(self, device: &wgpu::Device) -> Result<AtlasImage, TextureError> {
        let pixels = self.pixels(device);
        let img = AtlasImage {
            pixels,
            size: UVec2::new(self.dimensions.width as u32, self.dimensions.height as u32),
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
    ///
    /// Assumes the texture is in `Rgba8` format.
    pub fn into_rgba(
        self,
        device: &wgpu::Device,
        // `true` - the resulting image will be in a linear color space
        // `false` - the resulting image will be in an sRGB color space
        linear: bool,
    ) -> Result<image::RgbaImage, TextureError> {
        let format = self.format;
        let mut img_buffer = self.into_image::<u8, image::Rgba<u8>>(device)?.into_rgba8();
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
        let mut img_buffer = self.into_image::<u8, image::Rgba<u8>>(device)?.into_rgba8();
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
        let mut img_buffer = self.into_image::<u8, image::Rgba<u8>>(device)?.into_rgba8();
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
        let img = image::open("../../img/sandstone.png").unwrap();
        let width = img.width();
        let height = img.height();
        let mip_level_count = 5;
        let mut texture = Texture::from_dynamic_image(
            &r,
            img,
            Some("sandstone"),
            Some(
                wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
            ),
            1,
        );
        let mips = texture.generate_mips(&r, None, mip_level_count);

        let (channels, subpixel_bytes) =
            super::wgpu_texture_format_channels_and_subpixel_bytes(texture.texture.format());
        for (level, mip) in mips.into_iter().enumerate() {
            let mip_level = level + 1;
            let mip_width = width >> mip_level;
            let mip_height = height >> mip_level;
            // save out the mips
            let copied_buffer = Texture::read_from(
                &r,
                &mip.texture,
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

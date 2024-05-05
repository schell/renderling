//! Images and texture formats.
//!
//! Used to represent textures before they are sent to the GPU, in the
//! [`AtlasBuilder`].
use image::EncodableLayout;
use snafu::prelude::*;

fn cwd() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        Some("localhost".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let cwd = std::env::current_dir().ok()?;
        Some(format!("{}", cwd.display()))
    }
}

#[derive(Debug, Snafu)]
pub enum AtlasImageError {
    #[snafu(display("Cannot load image '{}' from cwd '{:?}': {source}", path.display(), cwd()))]
    CannotLoad {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[snafu(display("Image error: {source}\nCurrent dir: {:?}", cwd()))]
    Image { source: image::error::ImageError },
}

#[derive(Clone, Copy, Debug)]
pub enum AtlasImageFormat {
    R8,
    R8G8,
    R8G8B8,
    R8G8B8A8,
    R16,
    R16G16,
    R16G16B16,
    R16G16B16A16,
    R16G16B16A16FLOAT,
    R32G32B32FLOAT,
    R32G32B32A32FLOAT,
}

impl AtlasImageFormat {
    pub fn from_wgpu_texture_format(value: wgpu::TextureFormat) -> Option<Self> {
        match value {
            wgpu::TextureFormat::R8Uint => Some(AtlasImageFormat::R8),
            wgpu::TextureFormat::R16Uint => Some(AtlasImageFormat::R16),
            wgpu::TextureFormat::Rg8Uint => Some(AtlasImageFormat::R8G8),
            wgpu::TextureFormat::Rg16Uint => Some(AtlasImageFormat::R16G16),
            wgpu::TextureFormat::Rgba16Float => Some(AtlasImageFormat::R16G16B16A16FLOAT),
            _ => None,
        }
    }
}

/// Image data in transit from CPU to GPU.
#[derive(Clone, Debug)]
pub struct AtlasImage {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: AtlasImageFormat,
    // Whether or not to convert from sRGB color space into linear color space.
    pub apply_linear_transfer: bool,
}

#[cfg(feature = "gltf")]
impl From<gltf::image::Data> for AtlasImage {
    fn from(value: gltf::image::Data) -> Self {
        let pixels = value.pixels;
        let width = value.width;
        let height = value.height;
        let format = match value.format {
            gltf::image::Format::R8 => AtlasImageFormat::R8,
            gltf::image::Format::R8G8 => AtlasImageFormat::R8G8,
            gltf::image::Format::R8G8B8 => AtlasImageFormat::R8G8B8,
            gltf::image::Format::R8G8B8A8 => AtlasImageFormat::R8G8B8A8,
            gltf::image::Format::R16 => AtlasImageFormat::R16,
            gltf::image::Format::R16G16 => AtlasImageFormat::R16G16,
            gltf::image::Format::R16G16B16 => AtlasImageFormat::R16G16B16,
            gltf::image::Format::R16G16B16A16 => AtlasImageFormat::R16G16B16A16,
            gltf::image::Format::R32G32B32FLOAT => AtlasImageFormat::R32G32B32FLOAT,
            gltf::image::Format::R32G32B32A32FLOAT => AtlasImageFormat::R32G32B32A32FLOAT,
        };

        AtlasImage {
            pixels,
            format,
            // Determining this gets deferred until material construction
            apply_linear_transfer: false,
            width,
            height,
        }
    }
}

impl From<image::DynamicImage> for AtlasImage {
    fn from(value: image::DynamicImage) -> Self {
        let width = value.width();
        let height = value.height();

        use AtlasImageFormat::*;
        let (pixels, format) = match value {
            image::DynamicImage::ImageLuma8(img) => (img.into_vec(), R8),
            i @ image::DynamicImage::ImageLumaA8(_) => (i.into_rgba8().into_vec(), R8G8B8A8),
            image::DynamicImage::ImageRgb8(img) => (img.into_vec(), R8G8B8),
            image::DynamicImage::ImageRgba8(img) => (img.into_vec(), R8G8B8A8),
            image::DynamicImage::ImageLuma16(img) => (img.as_bytes().to_vec(), R16),
            i @ image::DynamicImage::ImageLumaA16(_) => {
                (i.into_rgba16().as_bytes().to_vec(), R16G16B16A16)
            }
            i @ image::DynamicImage::ImageRgb16(_) => (i.as_bytes().to_vec(), R16G16B16),
            i @ image::DynamicImage::ImageRgba16(_) => (i.as_bytes().to_vec(), R16G16B16A16),
            i @ image::DynamicImage::ImageRgb32F(_) => (i.as_bytes().to_vec(), R32G32B32FLOAT),
            i @ image::DynamicImage::ImageRgba32F(_) => (i.as_bytes().to_vec(), R32G32B32A32FLOAT),
            _ => todo!(),
        };
        AtlasImage {
            pixels,
            format,
            // Most of the time when people are using `image` to load images, those images
            // have color data that was authored in sRGB space.
            apply_linear_transfer: true,
            width,
            height,
        }
    }
}

impl TryFrom<std::path::PathBuf> for AtlasImage {
    type Error = AtlasImageError;

    fn try_from(value: std::path::PathBuf) -> Result<Self, Self::Error> {
        let img = image::open(value).context(ImageSnafu)?;
        Ok(img.into())
    }
}

impl AtlasImage {
    pub fn from_hdr_path(p: impl AsRef<std::path::Path>) -> Result<Self, AtlasImageError> {
        let bytes = std::fs::read(p.as_ref()).with_context(|_| CannotLoadSnafu {
            path: std::path::PathBuf::from(p.as_ref()),
        })?;
        Self::from_hdr_bytes(&bytes)
    }

    pub fn from_hdr_bytes(bytes: &[u8]) -> Result<Self, AtlasImageError> {
        // Decode HDR data.
        let decoder = image::codecs::hdr::HdrDecoder::new(bytes).context(ImageSnafu)?;
        let width = decoder.metadata().width;
        let height = decoder.metadata().height;
        let pixels = decoder.read_image_hdr().unwrap();

        // Add alpha data.
        let mut pixel_data: Vec<f32> = Vec::new();
        for pixel in pixels {
            pixel_data.push(pixel[0]);
            pixel_data.push(pixel[1]);
            pixel_data.push(pixel[2]);
            pixel_data.push(1.0);
        }
        let mut pixels = vec![];
        pixels.extend_from_slice(bytemuck::cast_slice(pixel_data.as_slice()));

        Ok(Self {
            pixels,
            width,
            height,
            format: AtlasImageFormat::R32G32B32A32FLOAT,
            apply_linear_transfer: false,
        })
    }

    pub fn from_path(p: impl AsRef<std::path::Path>) -> Result<Self, AtlasImageError> {
        Self::try_from(p.as_ref().to_path_buf())
    }

    pub fn into_rgba8(self) -> Option<image::RgbaImage> {
        let pixels = convert_to_rgba8_bytes(self.pixels, self.format, self.apply_linear_transfer);
        image::RgbaImage::from_vec(self.width, self.height, pixels)
    }
}

pub fn u16_to_u8(c: u16) -> u8 {
    ((c as f32 / 65535.0) * 255.0) as u8
}

pub fn f32_to_u8(c: f32) -> u8 {
    (c / 255.0) as u8
}

/// Interpret/convert the pixel data into rgba8 pixels.
///
/// This applies the linear transfer function if `apply_linear_transfer` is
/// `true`.
pub fn convert_to_rgba8_bytes(
    mut bytes: Vec<u8>,
    format: AtlasImageFormat,
    apply_linear_transfer: bool,
) -> Vec<u8> {
    use crate::color::*;
    log::trace!("converting image of format {format:?}");
    // Convert using linear transfer, if needed
    if apply_linear_transfer {
        log::trace!("  converting to linear color space (from sRGB)");
        match format {
            AtlasImageFormat::R8
            | AtlasImageFormat::R8G8
            | AtlasImageFormat::R8G8B8
            | AtlasImageFormat::R8G8B8A8 => {
                bytes.iter_mut().for_each(linear_xfer_u8);
            }
            AtlasImageFormat::R16
            | AtlasImageFormat::R16G16
            | AtlasImageFormat::R16G16B16
            | AtlasImageFormat::R16G16B16A16 => {
                let bytes: &mut [u16] = bytemuck::cast_slice_mut(&mut bytes);
                bytes.into_iter().for_each(linear_xfer_u16);
            }
            AtlasImageFormat::R16G16B16A16FLOAT => {
                let bytes: &mut [u16] = bytemuck::cast_slice_mut(&mut bytes);
                bytes.into_iter().for_each(linear_xfer_f16);
            }
            AtlasImageFormat::R32G32B32FLOAT | AtlasImageFormat::R32G32B32A32FLOAT => {
                let bytes: &mut [f32] = bytemuck::cast_slice_mut(&mut bytes);
                bytes.into_iter().for_each(linear_xfer_f32);
            }
        }
    }

    // Convert to rgba8
    match format {
        AtlasImageFormat::R8 => bytes.into_iter().flat_map(|r| [r, 0, 0, 255]).collect(),
        AtlasImageFormat::R8G8 => bytes
            .chunks_exact(2)
            .flat_map(|p| {
                if let [r, g] = p {
                    [*r, *g, 0, 255]
                } else {
                    unreachable!()
                }
            })
            .collect(),
        AtlasImageFormat::R8G8B8 => bytes
            .chunks_exact(3)
            .flat_map(|p| {
                if let [r, g, b] = p {
                    [*r, *g, *b, 255]
                } else {
                    unreachable!()
                }
            })
            .collect(),
        AtlasImageFormat::R8G8B8A8 => bytes,
        AtlasImageFormat::R16 => bytemuck::cast_slice::<u8, u16>(&bytes)
            .into_iter()
            .flat_map(|r| [u16_to_u8(*r), 0, 0, 255])
            .collect(),
        AtlasImageFormat::R16G16 => bytemuck::cast_slice::<u8, u16>(&bytes)
            .chunks_exact(2)
            .flat_map(|p| {
                if let [r, g] = p {
                    [u16_to_u8(*r), u16_to_u8(*g), 0, 255]
                } else {
                    unreachable!()
                }
            })
            .collect(),
        AtlasImageFormat::R16G16B16 => bytemuck::cast_slice::<u8, u16>(&bytes)
            .chunks_exact(3)
            .flat_map(|p| {
                if let [r, g, b] = p {
                    [u16_to_u8(*r), u16_to_u8(*g), u16_to_u8(*b), 255]
                } else {
                    unreachable!()
                }
            })
            .collect(),

        AtlasImageFormat::R16G16B16A16 => bytemuck::cast_slice::<u8, u16>(&bytes)
            .into_iter()
            .copied()
            .map(u16_to_u8)
            .collect(),
        AtlasImageFormat::R16G16B16A16FLOAT => bytemuck::cast_slice::<u8, u16>(&bytes)
            .into_iter()
            .map(|bits| half::f16::from_bits(*bits).to_f32())
            .collect::<Vec<_>>()
            .chunks_exact(4)
            .flat_map(|p| {
                if let [r, g, b, a] = p {
                    [f32_to_u8(*r), f32_to_u8(*g), f32_to_u8(*b), f32_to_u8(*a)]
                } else {
                    unreachable!()
                }
            })
            .collect(),
        AtlasImageFormat::R32G32B32FLOAT => bytemuck::cast_slice::<u8, f32>(&bytes)
            .chunks_exact(3)
            .flat_map(|p| {
                if let [r, g, b] = p {
                    [f32_to_u8(*r), f32_to_u8(*g), f32_to_u8(*b), 255]
                } else {
                    unreachable!()
                }
            })
            .collect(),
        AtlasImageFormat::R32G32B32A32FLOAT => bytemuck::cast_slice::<u8, f32>(&bytes)
            .into_iter()
            .copied()
            .map(f32_to_u8)
            .collect(),
    }
}

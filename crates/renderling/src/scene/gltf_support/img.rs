//! Image helpers for gltf support.
use snafu::prelude::*;

fn _image_data_format_to_wgpu(
    gltf_format: gltf::image::Format,
) -> Result<wgpu::TextureFormat, super::GltfLoaderError> {
    let format = match gltf_format {
        gltf::image::Format::R8 => wgpu::TextureFormat::R8Unorm,
        gltf::image::Format::R8G8 => wgpu::TextureFormat::Rg8Unorm,
        // wgpu doesn't have an rgb8unorm texture format 🤷
        gltf::image::Format::R8G8B8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        gltf::image::Format::R8G8B8A8 => wgpu::TextureFormat::Rgba8UnormSrgb,
        format => return Err(super::GltfLoaderError::UnsupportedImageFormat { format }),
    };
    Ok(format)
}

fn _image_data_format_num_channels(gltf_format: gltf::image::Format) -> u32 {
    match gltf_format {
        gltf::image::Format::R8 => 1,
        gltf::image::Format::R8G8 => 2,
        // wgpu doesn't have an rgb8unorm texture format 🤷, so we map to rgba8unormsrgb
        gltf::image::Format::R8G8B8 => 4,
        gltf::image::Format::R8G8B8A8 => 4,
        gltf::image::Format::R16 => 1,
        gltf::image::Format::R16G16 => 2,
        gltf::image::Format::R16G16B16 => 3,
        gltf::image::Format::R16G16B16A16 => 4,
        gltf::image::Format::R32G32B32FLOAT => 3,
        gltf::image::Format::R32G32B32A32FLOAT => 4,
    }
}

/// Convert a gltf image into a dynamic image.
pub fn gltf_image_data_to_dyn(
    image: gltf::image::Data,
) -> Result<image::DynamicImage, super::GltfLoaderError> {
    let img: image::DynamicImage = match image.format {
        gltf::image::Format::R8 => image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels,
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R8G8 => image::ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels,
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R8G8B8 => image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels,
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R8G8B8A8 => image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels,
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R16 => image::ImageBuffer::<image::Luma<u16>, Vec<u16>>::from_raw(
            image.width,
            image.height,
            bytemuck::cast_vec(image.pixels),
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R16G16 => image::ImageBuffer::<image::LumaA<u16>, Vec<u16>>::from_raw(
            image.width,
            image.height,
            bytemuck::cast_vec(image.pixels),
        )
        .context(super::InvalidImageSnafu)?
        .into(),
        gltf::image::Format::R16G16B16 => {
            image::ImageBuffer::<image::Rgb<u16>, Vec<u16>>::from_vec(
                image.width,
                image.height,
                bytemuck::cast_vec(image.pixels),
            )
            .context(super::InvalidImageSnafu)?
            .into()
        }
        gltf::image::Format::R16G16B16A16 => {
            image::ImageBuffer::<image::Rgba<u16>, Vec<u16>>::from_vec(
                image.width,
                image.height,
                bytemuck::cast_vec(image.pixels),
            )
            .context(super::InvalidImageSnafu)?
            .into()
        }
        gltf::image::Format::R32G32B32FLOAT => {
            let buffer: Vec<f32> = bytemuck::cast_vec(image.pixels);
            image::ImageBuffer::<image::Rgb<f32>, Vec<f32>>::from_raw(
                image.width,
                image.height,
                buffer,
            )
            .context(super::InvalidImageSnafu)?
            .into()
        }
        gltf::image::Format::R32G32B32A32FLOAT => {
            let buffer: Vec<f32> = bytemuck::cast_vec(image.pixels);
            image::ImageBuffer::<image::Rgba<f32>, Vec<f32>>::from_raw(
                image.width,
                image.height,
                buffer,
            )
            .context(super::InvalidImageSnafu)?
            .into()
        }
    };
    Ok(img)
}

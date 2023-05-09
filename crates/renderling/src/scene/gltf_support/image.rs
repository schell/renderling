//! Image helpers for gltf support.

pub fn gltf_image_data_to_dyn(image: gltf::)
let dyn_img: image::DynamicImage = match image.format {
    gltf::image::Format::R8 => {
        image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels.clone(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R8G8 => {
        image::ImageBuffer::<image::LumaA<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels.clone(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R8G8B8 => {
        image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels.clone(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R8G8B8A8 => {
        image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(
            image.width,
            image.height,
            image.pixels.clone(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R16 => {
        image::ImageBuffer::<image::Luma<u16>, Vec<u16>>::from_raw(
            image.width,
            image.height,
            bytemuck::cast_slice(&image.pixels).to_vec(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R16G16 => {
        image::ImageBuffer::<image::LumaA<u16>, Vec<u16>>::from_raw(
            image.width,
            image.height,
            bytemuck::cast_slice(&image.pixels).to_vec(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R16G16B16 => {
        image::ImageBuffer::<image::Rgb<u16>, Vec<u16>>::from_vec(
            image.width,
            image.height,
            bytemuck::cast_slice(&image.pixels).to_vec(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R16G16B16A16 => {
        image::ImageBuffer::<image::Rgba<u16>, Vec<u16>>::from_vec(
            image.width,
            image.height,
            bytemuck::cast_slice(&image.pixels).to_vec(),
        )
        .context(InvalidImageSnafu)?
        .into()
    }
    gltf::image::Format::R32G32B32FLOAT => {
        let buffer: Vec<f32> = bytemuck::cast_slice(&image.pixels).to_vec();
        let data = buffer.chunks_exact(3).map(|chunk| image::Rgb(chunk.into()));
        image::ImageBuffer::<image::Rgb<f32>, Vec<_>>::from_pixel_values(
            image.width,
            image.height,
            data,
        )
        .into()
    }
    gltf::image::Format::R32G32B32A32FLOAT => {
        let buffer: Vec<f32> = bytemuck::cast_slice(&image.pixels).to_vec();
        let data = buffer.chunks_exact(4).map(|chunk| image::Rgba(chunk.into()));
        image::ImageBuffer

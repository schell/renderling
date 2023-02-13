//! Collections of textures, materials, meshes, lights cameras and objects
//! arranged in 3d space.
//!
//! A scene is the structure that is built by importing a gltf file.
use snafu::prelude::*;

use crate::Texture;

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[cfg(feature = "gltf")]
    #[snafu(display("Unsupported gltf image format: {:?}", format))]
    UnsupportedImageFormat { format: gltf::image::Format },
}

#[cfg(feature = "gltf")]
mod gl {
    //! gltf helpers.
    use crate::SceneError;

    pub fn image_data_format_to_wgpu(
        gltf_format: gltf::image::Format,
    ) -> Result<wgpu::TextureFormat, SceneError> {
        let format = match gltf_format {
            gltf::image::Format::R8 => wgpu::TextureFormat::R8Unorm,
            gltf::image::Format::R8G8 => wgpu::TextureFormat::Rg8Unorm,
            // wgpu doesn't have an rgb8unorm texture format 🤷
            gltf::image::Format::R8G8B8 => wgpu::TextureFormat::Rgba8UnormSrgb,
            gltf::image::Format::R8G8B8A8 => wgpu::TextureFormat::Rgba8UnormSrgb,
            format => return Err(SceneError::UnsupportedImageFormat { format }),
        };
        Ok(format)
    }

    pub fn image_data_format_num_channels(gltf_format: gltf::image::Format) -> u32 {
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
}

/// Collections of textures, materials, meshes, lights cameras and objects
/// arranged in 3d space.
///
/// A scene is the structure that is built by importing a gltf file.
pub struct Scene {
    pub textures: Vec<Texture>,
}

impl Scene {
    #[cfg(feature = "gltf")]
    pub fn new_gltf(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        document: gltf::Document,
        buffers: Vec<gltf::buffer::Data>,
        images: Vec<gltf::image::Data>,
    ) -> Result<Self, SceneError> {
        log::info!("buffers.len(): {}", buffers.len());

        let mut textures = vec![];
        for dat in images.into_iter() {
            let format = gl::image_data_format_to_wgpu(dat.format)?;
            let num_channels = gl::image_data_format_num_channels(dat.format);
            let pixels = if dat.format == gltf::image::Format::R8G8B8 {
                dat.pixels
                    .as_slice()
                    .chunks(3)
                    .flat_map(|rgb| match rgb {
                        [r, g, b] => vec![*r, *g, *b, 255],
                        _ => unreachable!("not rgb"),
                    })
                    .collect::<Vec<_>>()
            } else {
                dat.pixels
            };
            textures.push(Texture::new_with_format(
                &device,
                &queue,
                None,
                None,
                format,
                num_channels,
                dat.width,
                dat.height,
                &pixels,
            ));
        }
        Ok(Scene { textures })
    }
}

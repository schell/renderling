//! Texture atlas.
//!
//! All images are packed into an atlas at scene build time.
//! Texture descriptors describing where in the atlas an image is,
//! and how callsites should sample pixels is packed into a buffer
//! on the GPU. This makes the number of texture binds _very_ low.
//!
//! An atlas should be temporary until we can use bindless techniques
//! on web.
use glam::UVec2;
use image::{EncodableLayout, RgbaImage};
use snafu::prelude::*;

use crate::SceneImage;

fn gpu_frame_from_rect(r: crunch::Rect) -> (UVec2, UVec2) {
    (
        UVec2::new(r.x as u32, r.y as u32),
        UVec2::new(r.w as u32, r.h as u32),
    )
}

#[derive(Debug, Snafu)]
pub enum AtlasError {
    #[snafu(display("Cannot pack textures. {len} textures took up too much space."))]
    CannotPackTextures { len: usize },
}

/// A texture atlas, used to store all the textures in a scene.
pub struct Atlas {
    pub texture: crate::Texture,
    pub rects: Vec<crunch::Rect>,
    pub size: UVec2,
}

impl Atlas {
    pub fn new_with_texture(texture: crate::Texture, size: UVec2) -> Self {
        Atlas {
            texture,
            rects: vec![],
            size,
        }
    }

    /// Create a new atlas.
    ///
    /// Size should be a power of two.
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, size: UVec2) -> Self {
        log::trace!("creating new atlas with dimensions {size}");
        let extent = wgpu::Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas texture"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let img = RgbaImage::from_pixel(extent.width, extent.height, image::Rgba([0, 0, 0, 0]));
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
                bytes_per_row: Some(4 * extent.width),
                rows_per_image: Some(extent.height),
            },
            extent,
        );
        let sampler_desc = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        let gpu_texture = crate::Texture::from_wgpu_tex(device, texture, Some(sampler_desc), None);
        Self::new_with_texture(gpu_texture, size)
    }

    /// Creates a new empty atlas.
    pub fn empty(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::new(device, queue, UVec2::new(1, 1))
    }

    /// Packs the atlas with the list of images.
    ///
    /// Returns a vector of ids that determine the locations of the given images
    /// within the atlas.
    ///
    /// This invalidates any pointers to previous textures in this atlas.
    pub fn pack(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: impl IntoIterator<Item = SceneImage>,
    ) -> Result<Self, AtlasError> {
        let mut images = images.into_iter().collect::<Vec<_>>();
        let len = images.len();
        let size = device.limits().max_texture_dimension_1d;
        let crunch::PackedItems { w, h, items } = crunch::pack_into_po2(
            size as usize,
            images.iter().enumerate().map(|(i, img)| {
                let w = img.width;
                let h = img.height;
                crunch::Item::new(i, w as usize, h as usize, crunch::Rotation::None)
            }),
        )
        .ok()
        .context(CannotPackTexturesSnafu { len })?;

        let mut atlas = Atlas::new(device, queue, UVec2::new(w as u32, h as u32));
        atlas.rects = vec![crunch::Rect::default(); len];

        for crunch::PackedItem { data: i, rect } in items.into_iter() {
            let img = &mut images[i];
            let bytes = crate::convert_to_rgba8_bytes(
                std::mem::take(&mut img.pixels),
                img.format,
                img.apply_linear_transfer,
            );
            queue.write_texture(
                wgpu::ImageCopyTextureBase {
                    texture: &atlas.texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: rect.x as u32,
                        y: rect.y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &bytes,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * img.width),
                    rows_per_image: Some(img.height),
                },
                wgpu::Extent3d {
                    width: img.width,
                    height: img.height,
                    depth_or_array_layers: 1,
                },
            );

            atlas.rects[i] = rect;
        }
        Ok(atlas)
    }

    pub fn frames(&self) -> impl Iterator<Item = (u32, (UVec2, UVec2))> + '_ {
        (0u32..).zip(self.rects.iter().copied().map(gpu_frame_from_rect))
    }

    pub fn get_frame(&self, index: usize) -> Option<(UVec2, UVec2)> {
        self.rects.get(index).copied().map(gpu_frame_from_rect)
    }

    pub fn merge(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        other: Atlas,
    ) -> Result<Self, AtlasError> {
        let mut images = self
            .rects
            .into_iter()
            .zip(std::iter::repeat(self.texture))
            .chain(
                other
                    .rects
                    .into_iter()
                    .zip(std::iter::repeat(other.texture)),
            )
            .collect::<Vec<_>>();
        let len = images.len();
        let size = device.limits().max_texture_dimension_1d;
        let crunch::PackedItems { w, h, items } = crunch::pack_into_po2(
            size as usize,
            images.iter().enumerate().map(|(i, (rect, _))| {
                let w = rect.w as usize;
                let h = rect.h as usize;
                crunch::Item::new(i, w, h, crunch::Rotation::None)
            }),
        )
        .ok()
        .context(CannotPackTexturesSnafu { len })?;

        let mut atlas = Atlas::new(device, queue, UVec2::new(w as u32, h as u32));
        atlas.rects = vec![crunch::Rect::default(); len];

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("merge atlas"),
        });
        for crunch::PackedItem { data: i, rect } in items.into_iter() {
            let (original_rect, texture) = &images[i];
            encoder.copy_texture_to_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: original_rect.x as u32,
                        y: original_rect.y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyTexture {
                    texture: &atlas.texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: rect.x as u32,
                        y: rect.y as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: original_rect.w as u32,
                    height: original_rect.h as u32,
                    depth_or_array_layers: 1,
                },
            );

            atlas.rects[i] = rect;
        }
        queue.submit(std::iter::once(encoder.finish()));
        Ok(atlas)
    }
}

#[cfg(test)]
mod test {
    use crate::Renderling;

    use super::*;

    impl Atlas {
        fn atlas_img(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> RgbaImage {
            let buffer = crate::Texture::read(
                &self.texture.texture,
                device,
                queue,
                self.size.x as usize,
                self.size.y as usize,
                4,
                1,
            );
            buffer.into_rgba(device).unwrap()
        }
    }

    #[test]
    fn can_merge_atlas() {
        let r = Renderling::headless(100, 100).unwrap();
        let (device, queue) = r.get_device_and_queue_owned();
        println!("{}", std::env::current_dir().unwrap().display());
        let cheetah = SceneImage::from_path("../../img/cheetah.jpg").unwrap();
        let dirt = SceneImage::from_path("../../img/dirt.jpg").unwrap();
        let happy_mac = SceneImage::from_path("../../img/happy_mac.png").unwrap();
        let sandstone = SceneImage::from_path("../../img/sandstone.png").unwrap();
        let atlas1 = Atlas::pack(&device, &queue, vec![cheetah, dirt]).unwrap();
        let atlas2 = Atlas::pack(&device, &queue, vec![happy_mac, sandstone]).unwrap();
        let atlas3 = atlas1.merge(&device, &queue, atlas2).unwrap();
        img_diff::assert_img_eq("atlas/merge3.png", atlas3.atlas_img(&device, &queue));
    }
}

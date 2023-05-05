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

fn gpu_frame_from_rect(r: crunch::Rect) -> (UVec2, UVec2) {
    (
        UVec2::new(r.x as u32, r.y as u32),
        UVec2::new(r.w as u32, r.h as u32),
    )
}

#[derive(Debug, Snafu)]
pub enum AtlasError {
    #[snafu(display("Cannot pack textures"))]
    CannotPackTextures,
}

/// A texture atlas, used to store all the textures in a scene.
pub struct Atlas {
    pub texture: crate::Texture,
    pub rects: Vec<crunch::Rect>,
    pub size: UVec2,
}

impl Atlas {
    pub fn new_with_texture(texture: crate::Texture, width: u32, height: u32) -> Self {
        Atlas {
            texture,
            rects: vec![],
            size: UVec2::new(width, height),
        }
    }

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> Self {
        log::trace!("creating new atlas with dimensions {width} {height}");
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let img = RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));
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
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
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
        let gpu_texture = crate::Texture::from_wgpu_tex(device, texture, Some(sampler_desc));
        Self::new_with_texture(gpu_texture, width, height)
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
        images: impl IntoIterator<Item = RgbaImage>,
    ) -> Result<Self, AtlasError> {
        let images = images.into_iter().collect::<Vec<_>>();
        let len = images.len();
        let crunch::PackedItems { w, h, items } = crunch::pack_into_po2(
            8192,
            images.iter().enumerate().map(|(i, img)| {
                let w = img.width();
                let h = img.height();
                crunch::Item::new(i, w as usize, h as usize, crunch::Rotation::None)
            }),
        )
        .ok()
        .context(CannotPackTexturesSnafu)?;

        let mut atlas = Atlas::new(device, queue, w as u32, h as u32);
        atlas.rects = vec![crunch::Rect::default(); len];

        for crunch::PackedItem { data: i, rect } in items.into_iter() {
            let img = &images[i];
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
                img.as_bytes(),
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * img.width()),
                    rows_per_image: Some(img.height()),
                },
                wgpu::Extent3d {
                    width: img.width(),
                    height: img.height(),
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
}

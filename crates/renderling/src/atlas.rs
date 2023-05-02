//! Texture atlas
use glam::Vec2;
use image::{EncodableLayout, RgbaImage};
use renderling_shader::scene::GpuTexture;
use snafu::prelude::*;

fn gpu_texture_from_rect(r: crunch::Rect) -> GpuTexture {
    GpuTexture {
        offset_px: Vec2::new(r.x as f32, r.y as f32),
        size_px: Vec2::new(r.w as f32, r.h as f32),
    }
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
    pub size: Vec2,
}

impl Atlas {
    pub fn new_with_texture(texture: crate::Texture, width: u32, height: u32) -> Self {
        Atlas {
            texture,
            rects: vec![],
            size: Vec2::new(width as f32, height as f32),
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
                // TODO: pad this to a multiple 256 if needed
                bytes_per_row: Some(4 * width),
                rows_per_image: None,
            },
            size,
        );

        let gpu_texture = crate::Texture::from_wgpu_tex(texture, device);
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
                    rows_per_image: None,
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

    pub fn textures(&self) -> impl Iterator< Item = (u32, GpuTexture)> + '_ {
        (0u32..)
            .zip(self.rects.iter().copied().map(gpu_texture_from_rect))
    }

    pub fn transform_uvs(uv: Vec2, rect: crunch::Rect, atlas_size: Vec2) -> Vec2 {
        let crunch::Rect { x, y, w, h } = rect;
        let img_origin = Vec2::new(x as f32, y as f32);
        let img_size = Vec2::new(w as f32, h as f32);
        // convert the uv from normalized into pixel locations in the original image
        let uv_img_pixels = uv * img_size;
        // convert those into pixel locations in the atlas image
        let uv_atlas_pixels = img_origin + uv_img_pixels;
        // normalize the uvs by the atlas size
        let uv_atlas_normalized = uv_atlas_pixels / atlas_size;
        uv_atlas_normalized
    }

    pub fn get_texture(&self, texture_id: u32) -> Option<GpuTexture> {
        self.rects.get(texture_id as usize).copied().map(gpu_texture_from_rect)
    }
}

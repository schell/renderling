use glam::UVec2;
use image::{EncodableLayout, RgbaImage};
use snafu::prelude::*;

use super::atlas_image::AtlasImage;

#[derive(Debug, Snafu)]
pub enum AtlasError {
    #[snafu(display("Cannot pack textures. {len} textures took up too much space."))]
    CannotPackTextures { len: usize },
}

fn gpu_frame_from_rect(r: crunch::Rect) -> (UVec2, UVec2) {
    (
        UVec2::new(r.x as u32, r.y as u32),
        UVec2::new(r.w as u32, r.h as u32),
    )
}

/// A texture atlas packing, before it is committed to the GPU.
#[derive(Clone)]
pub enum Packing {
    Img {
        index: usize,
        image: AtlasImage,
    },
    GpuImg {
        index: usize,
        offset_px: UVec2,
        size_px: UVec2,
    },
}

impl Packing {
    pub fn width(&self) -> u32 {
        match self {
            Packing::Img { image, .. } => image.width,
            Packing::GpuImg { size_px, .. } => size_px.x,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Packing::Img { image, .. } => image.height,
            Packing::GpuImg { size_px, .. } => size_px.y,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Packing::Img { index, .. } => *index,
            Packing::GpuImg { index, .. } => *index,
        }
    }

    pub fn set_index(&mut self, index: usize) {
        match self {
            Packing::Img { index: i, .. } => *i = index,
            Packing::GpuImg { index: i, .. } => *i = index,
        }
    }

    pub fn as_scene_img_mut(&mut self) -> Option<&mut AtlasImage> {
        match self {
            Packing::Img { image, .. } => Some(image),
            Packing::GpuImg { .. } => None,
        }
    }

    pub fn as_scene_img(&self) -> Option<&AtlasImage> {
        match self {
            Packing::Img { image, .. } => Some(image),
            Packing::GpuImg { .. } => None,
        }
    }
}

/// A preview of the packed atlas.
///
/// Using a preview of the atlas allows us to access the packed frames
/// without committing the atlas to the GPU. Since the items images
/// are still in CPU memory, we can mutate them and then commit the
/// atlas to the GPU.
#[repr(transparent)]
pub struct RepackPreview {
    pub items: crunch::PackedItems<Packing>,
}

impl RepackPreview {
    pub fn get_frame(&self, index: usize) -> Option<(UVec2, UVec2)> {
        self.items.items.get(index).map(|item| {
            let rect = item.rect;
            gpu_frame_from_rect(rect)
        })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Packing> {
        self.items.items.get_mut(index).map(|item| &mut item.data)
    }

    pub fn new_images_len(&self) -> usize {
        self.items
            .items
            .iter()
            .filter(|item| item.data.as_scene_img().is_some())
            .count()
    }
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

    pub fn is_empty(&self) -> bool {
        self.rects.is_empty()
    }

    /// Does a dry-run packing of the atlas with the list of images.
    ///
    /// Returns a vector of ids that determine the locations of the given images
    /// but doesn't send any data to the GPU.
    pub fn pack_preview<'a>(
        device: &wgpu::Device,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<crunch::PackedItems<AtlasImage>, AtlasError> {
        let images = images.into_iter().collect::<Vec<_>>();
        let len = images.len();
        let limit = device.limits().max_texture_dimension_1d;
        let items = crunch::pack_into_po2(
            limit as usize,
            images.into_iter().map(|img| {
                let w = img.width;
                let h = img.height;
                crunch::Item::new(img, w as usize, h as usize, crunch::Rotation::None)
            }),
        )
        .ok()
        .context(CannotPackTexturesSnafu { len })?;
        Ok(items)
    }

    pub fn commit_preview(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        crunch::PackedItems { w, h, items }: crunch::PackedItems<AtlasImage>,
    ) -> Result<Self, AtlasError> {
        let mut atlas = Atlas::new(device, queue, UVec2::new(w as u32, h as u32));
        atlas.rects = items
            .into_iter()
            .map(
                |crunch::PackedItem {
                     data: mut img,
                     rect,
                 }| {
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
                    rect
                },
            )
            .collect();

        Ok(atlas)
    }

    pub fn repack_preview(
        &self,
        device: &wgpu::Device,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<RepackPreview, AtlasError> {
        let mut images = images.into_iter().collect::<Vec<_>>();
        let len = images.len() + self.rects.len();
        let items = crunch::pack_into_po2(
            device.limits().max_texture_dimension_1d as usize,
            self.rects
                .iter()
                .map(|r| Packing::GpuImg {
                    index: 0,
                    offset_px: UVec2::new(r.x as u32, r.y as u32),
                    size_px: UVec2::new(r.w as u32, r.y as u32),
                })
                .chain(
                    images
                        .drain(..)
                        .map(|image| Packing::Img { index: 0, image }),
                )
                .enumerate()
                .map(|(i, mut p)| {
                    let w = p.width() as usize;
                    let h = p.height() as usize;
                    p.set_index(i);
                    crunch::Item::new(p, w, h, crunch::Rotation::None)
                }),
        )
        .ok()
        .context(CannotPackTexturesSnafu { len })?;
        debug_assert!(items
            .items
            .iter()
            .zip(0..)
            .all(|(item, i)| item.data.index() == i));
        Ok(RepackPreview { items })
    }

    pub fn commit_repack_preview(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        RepackPreview {
            items: crunch::PackedItems { w, h, items },
        }: RepackPreview,
    ) -> Result<Atlas, AtlasError> {
        let mut atlas = Atlas::new(device, queue, UVec2::new(w as u32, h as u32));
        atlas.rects = vec![crunch::Rect::default(); items.len()];

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("repack atlas"),
        });
        for crunch::PackedItem { data: p, rect } in items.into_iter() {
            match p {
                Packing::Img { index, mut image } => {
                    let bytes = crate::convert_to_rgba8_bytes(
                        std::mem::take(&mut image.pixels),
                        image.format,
                        image.apply_linear_transfer,
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
                            bytes_per_row: Some(4 * image.width),
                            rows_per_image: Some(image.height),
                        },
                        wgpu::Extent3d {
                            width: image.width,
                            height: image.height,
                            depth_or_array_layers: 1,
                        },
                    );
                    atlas.rects[index] = rect;
                }
                Packing::GpuImg {
                    index,
                    offset_px,
                    size_px,
                } => {
                    encoder.copy_texture_to_texture(
                        wgpu::ImageCopyTexture {
                            texture: &self.texture.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d {
                                x: offset_px.x,
                                y: offset_px.y,
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
                            width: size_px.x,
                            height: size_px.y,
                            depth_or_array_layers: 1,
                        },
                    );
                    atlas.rects[index] = rect;
                }
            }
        }
        queue.submit(std::iter::once(encoder.finish()));
        Ok(atlas)
    }

    /// Packs the atlas with the list of images.
    ///
    /// Returns a vector of ids that determine the locations of the given images
    /// within the atlas.
    pub fn pack(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Self, AtlasError> {
        let images = images.into_iter().collect::<Vec<_>>();
        let items = Self::pack_preview(device, images)?;
        Self::commit_preview(device, queue, items)
    }

    pub fn repack(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Self, AtlasError> {
        let images = images.into_iter().collect::<Vec<_>>();
        let items = self.repack_preview(device, images)?;
        self.commit_repack_preview(device, queue, items)
    }

    pub fn frames(&self) -> impl Iterator<Item = (u32, (UVec2, UVec2))> + '_ {
        (0u32..).zip(self.rects.iter().copied().map(gpu_frame_from_rect))
    }

    /// Return the position and size of the frame at the given index.
    pub fn get_frame(&self, index: usize) -> Option<(UVec2, UVec2)> {
        self.rects.get(index).copied().map(gpu_frame_from_rect)
    }

    pub fn merge(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        other: &Atlas,
    ) -> Result<Self, AtlasError> {
        let images = self
            .rects
            .iter()
            .zip(std::iter::repeat(&self.texture))
            .chain(other.rects.iter().zip(std::iter::repeat(&other.texture)))
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

    /// Read the atlas image from the GPU.
    ///
    /// This is primarily for testing.
    ///
    /// The resulting image will be in a **linear** color space.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn atlas_img(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> RgbaImage {
        let buffer = crate::Texture::read(
            &self.texture.texture,
            device,
            queue,
            self.size.x as usize,
            self.size.y as usize,
            4,
            1,
        );
        buffer.into_linear_rgba(device).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        atlas::{AtlasTexture, TextureAddressMode, TextureModes},
        pbr::Material,
        stage::Vertex,
        Camera, Renderlet, Renderling, Transform,
    };
    use crabslab::GrowableSlab;
    use glam::{Vec2, Vec3, Vec4};

    use super::*;

    #[test]
    fn can_merge_atlas() {
        let r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();
        println!("{}", std::env::current_dir().unwrap().display());
        let cheetah = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let happy_mac = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let atlas1 = Atlas::pack(&device, &queue, vec![cheetah, dirt]).unwrap();
        let atlas2 = Atlas::pack(&device, &queue, vec![happy_mac, sandstone]).unwrap();
        let atlas3 = atlas1.merge(&device, &queue, &atlas2).unwrap();
        img_diff::assert_img_eq("atlas/merge3.png", atlas3.atlas_img(&device, &queue));
    }

    #[test]
    // Ensures that textures are packed and rendered correctly.
    fn atlas_uv_mapping() {
        let mut r =
            Renderling::headless(32, 32).with_background_color(Vec3::splat(0.0).extend(1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = crate::camera::default_ortho2d(32.0, 32.0);
        let camera = stage.append(&Camera::new(projection, view));
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../test_img/atlas/uv_mapping.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
        let mut texels_tex = textures[2];
        texels_tex.modes.s = TextureAddressMode::ClampToEdge;
        texels_tex.modes.t = TextureAddressMode::ClampToEdge;
        let texels_tex_id = stage.append(&texels_tex);
        let material = stage.append(&Material {
            albedo_texture: texels_tex_id,
            has_lighting: false,
            ..Default::default()
        });
        let geometry = stage.append_array(&{
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(1.0, 0.0, 0.0))
                .with_uv0(Vec2::new(1.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, 1.0, 0.0))
                .with_uv0(Vec2::new(0.0, 1.0));
            let br = Vertex::default()
                .with_position(Vec3::new(1.0, 1.0, 0.0))
                .with_uv0(Vec2::splat(1.0));
            [tl, bl, br, tl, br, tr]
        });
        let transform = stage.append(&Transform {
            scale: Vec3::new(32.0, 32.0, 1.0),
            ..Default::default()
        });
        let _renderlet = stage.draw(&crate::Renderlet {
            camera,
            vertices: geometry,
            transform,
            material,
            ..Default::default()
        });

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("atlas/uv_mapping.png", img);
    }

    #[test]
    // Ensures that textures with different wrapping modes are rendered correctly.
    fn uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h).with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);
        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.append(&Camera::new(projection, view));
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
        let texel_tex = textures[2];
        let mut clamp_tex = texel_tex;
        clamp_tex.modes.s = TextureAddressMode::ClampToEdge;
        clamp_tex.modes.t = TextureAddressMode::ClampToEdge;
        let mut repeat_tex = texel_tex;
        repeat_tex.modes.s = TextureAddressMode::Repeat;
        repeat_tex.modes.t = TextureAddressMode::Repeat;
        let mut mirror_tex = texel_tex;
        mirror_tex.modes.s = TextureAddressMode::MirroredRepeat;
        mirror_tex.modes.t = TextureAddressMode::MirroredRepeat;

        let albedo_texture = stage.append(&clamp_tex);
        let clamp_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });
        let albedo_texture = stage.append(&repeat_tex);
        let repeat_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });
        let albedo_texture = stage.append(&mirror_tex);
        let mirror_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let geometry = stage.append_array(&{
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(3.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, 3.0));
            let br = Vertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(3.0));
            [tl, bl, br, tl, br, tr]
        });
        let _clamp_prim = stage.draw(&Renderlet {
            camera,
            vertices: geometry,
            material: clamp_material_id,
            ..Default::default()
        });
        let _repeat_prim = {
            let transform = stage.append(&Transform {
                translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
                ..Default::default()
            });
            stage.draw(&Renderlet {
                camera,
                vertices: geometry,
                material: repeat_material_id,
                transform,
                ..Default::default()
            })
        };
        let _mirror_prim = {
            let transform = stage.append(&Transform {
                translation: Vec3::new(sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0),
                ..Default::default()
            });
            stage.draw(&Renderlet {
                camera,
                vertices: geometry,
                material: mirror_material_id,
                transform,
                ..Default::default()
            })
        };

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("atlas/uv_wrapping.png", img);
    }

    #[test]
    // Ensures that textures with negative uv coords wrap correctly
    fn negative_uv_wrapping() {
        let icon_w = 32;
        let icon_h = 41;
        let sheet_w = icon_w * 3;
        let sheet_h = icon_h * 3;
        let w = sheet_w * 3 + 2;
        let h = sheet_h;
        let mut r = Renderling::headless(w, h).with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let mut stage = r.new_stage();
        stage.configure_graph(&mut r, true);

        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.append(&Camera {
            projection,
            view,
            ..Default::default()
        });

        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();

        let texel_tex = textures[2];
        let mut clamp_tex = texel_tex;
        clamp_tex.modes.s = TextureAddressMode::ClampToEdge;
        clamp_tex.modes.t = TextureAddressMode::ClampToEdge;
        let mut repeat_tex = texel_tex;
        repeat_tex.modes.s = TextureAddressMode::Repeat;
        repeat_tex.modes.t = TextureAddressMode::Repeat;
        let mut mirror_tex = texel_tex;
        mirror_tex.modes.s = TextureAddressMode::MirroredRepeat;
        mirror_tex.modes.t = TextureAddressMode::MirroredRepeat;

        let albedo_texture = stage.append(&clamp_tex);
        let clamp_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });

        let albedo_texture = stage.append(&repeat_tex);
        let repeat_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });

        let albedo_texture = stage.append(&mirror_tex);
        let mirror_material_id = stage.append(&Material {
            albedo_texture,
            has_lighting: false,
            ..Default::default()
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let geometry = {
            let tl = Vertex::default()
                .with_position(Vec3::ZERO)
                .with_uv0(Vec2::ZERO);
            let tr = Vertex::default()
                .with_position(Vec3::new(sheet_w, 0.0, 0.0))
                .with_uv0(Vec2::new(-3.0, 0.0));
            let bl = Vertex::default()
                .with_position(Vec3::new(0.0, sheet_h, 0.0))
                .with_uv0(Vec2::new(0.0, -3.0));
            let br = Vertex::default()
                .with_position(Vec3::new(sheet_w, sheet_h, 0.0))
                .with_uv0(Vec2::splat(-3.0));
            stage.append_array(&[tl, bl, br, tl, br, tr])
        };

        let _clamp_prim = stage.draw(&Renderlet {
            camera,
            vertices: geometry,
            material: clamp_material_id,
            ..Default::default()
        });
        let _repeat_prim = {
            let transform = stage.append(&Transform {
                translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
                ..Default::default()
            });

            stage.draw(&Renderlet {
                camera,
                vertices: geometry,
                material: repeat_material_id,
                transform,
                ..Default::default()
            })
        };
        let _mirror_prim = {
            let transform = stage.append(&Transform {
                translation: Vec3::new(sheet_w as f32 * 2.0 + 2.0, 0.0, 0.0),
                ..Default::default()
            });

            stage.draw(&Renderlet {
                camera,
                vertices: geometry,
                material: mirror_material_id,
                transform,
                ..Default::default()
            })
        };

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("atlas/negative_uv_wrapping.png", img);
    }

    #[test]
    fn transform_uvs_for_atlas() {
        let mut tex = AtlasTexture {
            offset_px: UVec2::ZERO,
            size_px: UVec2::ONE,
            modes: TextureModes::default(),
            ..Default::default()
        };
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(100)));
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(1)));
        assert_eq!(Vec2::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(256)));
        tex.offset_px = UVec2::splat(10);
        assert_eq!(Vec2::splat(0.1), tex.uv(Vec2::ZERO, UVec2::splat(100)));
    }
}

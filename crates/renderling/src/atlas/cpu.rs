use core::{ops::Deref, sync::atomic::AtomicU32};
use glam::UVec2;
use image::RgbaImage;
use snafu::prelude::*;
use std::sync::{Arc, RwLock};

use crate::{
    slab::{Hybrid, IsBuffer, SlabAllocator},
    texture::Texture,
};

use super::{
    atlas_image::{convert_to_rgba8_bytes, AtlasImage},
    AtlasTexture,
};

#[derive(Debug, Snafu)]
pub enum AtlasError {
    #[snafu(display("Cannot pack textures. {len} textures took up too much space."))]
    CannotPackTextures { len: usize },

    #[snafu(display("Missing layer {index}"))]
    MissingLayer { index: u32, images: Vec<AtlasImage> },
}

fn assert_power_of_2(n: u32) {
    assert_eq!(0, n & (n - 1), "{n} is not a power of 2")
}

fn fan_split_n<T>(n: usize, input: impl IntoIterator<Item = T>) -> Vec<Vec<T>> {
    let mut input = input.into_iter();
    let mut output = vec![];
    for _ in 0..n {
        output.push(vec![]);
    }
    let mut i = 0;
    while let Some(item) = input.next() {
        // UNWRAP: safe because i % n
        output.get_mut(i).unwrap().push(item);
        i = (i + 1) % n;
    }
    output
}

/// A texture atlas packing, before it is committed to the GPU.
#[derive(Clone)]
pub enum Packing {
    /// A new packing.
    ///
    /// This image does not yet live on the GPU
    Img {
        /// Index of the layer within the atlas.
        layer_index: u32,
        /// Index of the texture within the layer.
        texture_index: u32,
        /// Image bytes, etc
        image: AtlasImage,
    },
    /// A previous packing.
    ///
    /// This image has already been staged on the GPU.
    GpuImg { texture: Hybrid<AtlasTexture> },
}

impl Packing {
    pub fn size(&self) -> UVec2 {
        match self {
            Packing::Img { image, .. } => image.size,
            Packing::GpuImg { texture } => texture.get().size_px,
        }
    }

    pub fn texture_index(&self) -> u32 {
        match self {
            Packing::Img { texture_index, .. } => *texture_index,
            Packing::GpuImg { texture } => texture.get().texture_index,
        }
    }

    //     pub fn set_index(&mut self, index: usize) {
    //         match self {
    //             Packing::Img { index: i, .. } => *i = index,
    //             Packing::GpuImg { index: i, .. } => *i = index,
    //         }
    //     }

    //     pub fn as_scene_img_mut(&mut self) -> Option<&mut AtlasImage> {
    //         match self {
    //             Packing::Img { image, .. } => Some(image),
    //             Packing::GpuImg { .. } => None,
    //         }
    //     }

    //     pub fn as_scene_img(&self) -> Option<&AtlasImage> {
    //         match self {
    //             Packing::Img { image, .. } => Some(image),
    //             Packing::GpuImg { .. } => None,
    //         }
    //     }
}

#[derive(Clone, Default, Debug)]
pub struct Layer {
    pub textures: Vec<Hybrid<AtlasTexture>>,
}

/// A texture atlas, used to store all the textures in a scene.
///
/// Clones of `Atlas` all point to the same internal data.
#[derive(Clone)]
pub struct Atlas {
    texture_array: Arc<RwLock<Texture>>,
    layers: Arc<RwLock<Vec<Layer>>>,
    size: Arc<AtomicU32>,
}

impl Atlas {
    /// Create a new atlas with `size` and `num_layers` layers.
    ///
    /// `size` **must be a power of two**.
    ///
    /// ## Panics
    /// Panics if `size` is _not_ a power of two.
    pub fn new_with_texture(texture: Texture, size: u32, num_layers: u32) -> Self {
        assert_power_of_2(size);
        Atlas {
            texture_array: Arc::new(RwLock::new(texture)),
            layers: Arc::new(RwLock::new(vec![Layer::default(); num_layers as usize])),
            size: Arc::new(size.into()),
        }
    }

    /// Create the initial texture to use.
    fn create_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: u32,
        num_layers: u32,
    ) -> Texture {
        let extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: num_layers,
        };

        assert_power_of_2(size);
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

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("create atlas texture array"),
        });
        encoder.clear_texture(
            &texture,
            &wgpu::ImageSubresourceRange {
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            },
        );
        queue.submit(Some(encoder.finish()));

        let sampler_desc = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };

        Texture::from_wgpu_tex(device, texture, Some(sampler_desc), None)
    }

    /// Create a new atlas.
    ///
    /// Size _must_ be a power of two.
    ///
    /// ## Panics
    /// Panics if `size` is not a power of two.
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, size: u32, num_layers: u32) -> Self {
        log::trace!("creating new atlas with dimensions {size}");
        assert_power_of_2(size);
        let texture = Self::create_texture(device, queue, size, num_layers);
        Self::new_with_texture(texture, size, num_layers)
    }

    /// Creates a new empty atlas.
    pub fn empty(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::new(device, queue, 2, 1)
    }

    pub fn is_empty(&self) -> bool {
        // UNWRAP: panic on purpose
        let layers = self.layers.read().unwrap();
        let num_frames = layers
            .iter()
            .map(|layer| layer.textures.len())
            .sum::<usize>();
        num_frames == 0
    }

    /// Returns a clone of the current atlas texture.
    pub fn get_texture(&self) -> Texture {
        // UNWRAP: panic on purpose
        self.texture_array.read().unwrap().clone()
    }

    pub fn get_layers(&self) -> impl Deref<Target = Vec<Layer>> + '_ {
        // UNWRAP: panic on purpose
        self.layers.read().unwrap()
    }

    // /// Packs the atlas with the list of images.
    // ///
    // /// Returns a vector of ids that determine the locations of the given images
    // /// within the atlas.
    // pub fn from_images(
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    //     images: impl IntoIterator<Item = AtlasImage>,
    // ) -> Result<Self, AtlasError> {
    //     let images = images.into_iter().collect::<Vec<_>>();
    //     let items = Self::pack_preview(device, images)?;
    //     let AtlasPacking {
    //         texture,
    //         rects,
    //         size,
    //     } = Self::commit_preview(device, queue, items)?;
    //     Ok(Atlas {
    //         texture: Arc::new(RwLock::new(texture)),
    //         layers: Arc::new(RwLock::new(rects)),
    //         size: Arc::new(RwLock::new(size)),
    //     })
    // }

    // /// Reset this atlas with all new images.
    // ///
    // /// This invalidates the previous internal texture, creating a new one.
    // pub fn reset(
    //     &self,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    //     images: impl IntoIterator<Item = AtlasImage>,
    // ) -> Result<(), AtlasError> {
    //     let images = images.into_iter().collect::<Vec<_>>();
    //     let items = Self::pack_preview(device, images)?;
    //     let AtlasPacking {
    //         texture,
    //         rects,
    //         size,
    //     } = Self::commit_preview(device, queue, items)?;
    //     // UNWRAP: panic on purpose
    //     *self.texture.write().unwrap() = texture;
    //     *self.layers.write().unwrap() = rects;
    //     *self.size.write().unwrap() = size;
    //     Ok(())
    // }

    // pub fn frames(&self) -> Vec<(u32, (UVec2, UVec2))> {
    //     let rects_guard = self.layers.read().unwrap();
    //     let rects = rects_guard.iter().map(gpu_frame_from_rect);
    //     (0u32..).zip(rects).collect()
    // }

    // /// Return the position and size of the frame at the given index.
    // pub fn get_frame(&self, index: usize) -> Option<(UVec2, UVec2)> {
    //     self.layers
    //         .read()
    //         .unwrap()
    //         .get(index)
    //         .map(gpu_frame_from_rect)
    // }

    pub fn get_size(&self) -> UVec2 {
        let size = self.size.load(core::sync::atomic::Ordering::Relaxed);
        UVec2::splat(size)
    }

    pub fn add_images(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab: &SlabAllocator<impl IsBuffer>,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, AtlasError> {
        // UNWRAP: POP
        let mut layers = self.layers.write().unwrap();
        let images = images.into_iter().map(|i| i.into());
        let layer_additions: Vec<Vec<AtlasImage>> = fan_split_n(layers.len(), images);
        let size = self.size.load(core::sync::atomic::Ordering::Relaxed);

        // UNWRAP: POP
        let texture_array = self.texture_array.write().unwrap();
        let new_texture_array = Self::create_texture(device, queue, size, layers.len() as u32);

        let mut hybrids = vec![];
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("atlas add images"),
        });
        // for each new layer addition, attempt to repack that layer
        for (i, additions) in layer_additions.into_iter().enumerate() {
            // UNWRAP: safe because we know this index exists from our calc above
            let layer = layers.get_mut(i).unwrap();
            let images_starting_texture_index = layer.textures.len() as u32;
            let maybe_items = crunch::pack_into_po2(
                size as usize,
                layer
                    .textures
                    .iter()
                    .map(|self_texture| Packing::GpuImg {
                        texture: self_texture.clone(),
                    })
                    .chain(
                        additions
                            .into_iter()
                            .enumerate()
                            .map(|(i, image)| Packing::Img {
                                layer_index: i as u32,
                                texture_index: images_starting_texture_index + i as u32,
                                image,
                            }),
                    )
                    .map(|packing: Packing| {
                        let size = packing.size();
                        crunch::Item::new(
                            packing,
                            size.x as usize,
                            size.y as usize,
                            crunch::Rotation::None,
                        )
                    }),
            )
            .ok();
            if let Some(items) = maybe_items {
                for crunch::PackedItem { data: item, rect } in items.items.into_iter() {
                    let offset_px = UVec2::new(rect.x as u32, rect.y as u32);
                    let size_px = UVec2::new(rect.w as u32, rect.h as u32);
                    // convert into hybrids
                    match item {
                        Packing::Img {
                            layer_index,
                            texture_index,
                            mut image,
                        } => {
                            let hybrid = slab.new_value(AtlasTexture {
                                offset_px,
                                size_px,
                                layer_index,
                                texture_index,
                                ..Default::default()
                            });
                            hybrids.push(hybrid.clone());
                            layer.textures.push(hybrid);

                            let bytes = convert_to_rgba8_bytes(
                                std::mem::take(&mut image.pixels),
                                image.format,
                                image.apply_linear_transfer,
                            );

                            // write the new image from the CPU to the new texture
                            queue.write_texture(
                                wgpu::ImageCopyTextureBase {
                                    texture: &new_texture_array.texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d {
                                        x: offset_px.x,
                                        y: offset_px.y,
                                        z: layer_index,
                                    },
                                    aspect: wgpu::TextureAspect::All,
                                },
                                &bytes,
                                wgpu::ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some(4 * size_px.x),
                                    rows_per_image: Some(size_px.y),
                                },
                                wgpu::Extent3d {
                                    width: size_px.x,
                                    height: size_px.y,
                                    depth_or_array_layers: 1,
                                },
                            );
                        }
                        Packing::GpuImg { texture } => texture.modify(|t| {
                            t.offset_px = offset_px;
                            debug_assert_eq!(t.size_px, size_px);

                            // copy the frame from the old texture to the new texture
                            // in a new destination
                            encoder.copy_texture_to_texture(
                                wgpu::ImageCopyTexture {
                                    texture: &texture_array.texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d {
                                        x: offset_px.x,
                                        y: offset_px.y,
                                        z: t.layer_index,
                                    },
                                    aspect: wgpu::TextureAspect::All,
                                },
                                wgpu::ImageCopyTexture {
                                    texture: &new_texture_array.texture,
                                    mip_level: 0,
                                    origin: wgpu::Origin3d {
                                        x: size_px.x,
                                        y: size_px.y,
                                        z: t.layer_index,
                                    },
                                    aspect: wgpu::TextureAspect::All,
                                },
                                wgpu::Extent3d {
                                    width: size_px.x,
                                    height: size_px.y,
                                    depth_or_array_layers: 1,
                                },
                            );
                        }),
                    }
                }
            } else {
                // TODO: create a new layer and retry
                return CannotPackTexturesSnafu { len: layers.len() }.fail();
            }
        }

        Ok(hybrids)
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
        let size = self.get_size();
        let buffer = Texture::read(
            &self.get_texture().texture,
            device,
            queue,
            size.x as usize,
            size.y as usize,
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
        camera::Camera,
        pbr::Material,
        stage::{Renderlet, Vertex},
        transform::Transform,
        Context,
    };
    use glam::{Vec2, Vec3, Vec4};

    use super::*;

    #[test]
    // Ensures that textures are packed and rendered correctly.
    fn atlas_uv_mapping() {
        let ctx = Context::headless(32, 32);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let (projection, view) = crate::camera::default_ortho2d(32.0, 32.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../test_img/atlas/uv_mapping.png").unwrap();
        let textures = stage.set_images([dirt, sandstone, texels]).unwrap();
        let mut texels_tex = textures[2];
        texels_tex.modes.s = TextureAddressMode::ClampToEdge;
        texels_tex.modes.t = TextureAddressMode::ClampToEdge;
        let texels_tex = stage.new_value(texels_tex);
        let material = stage.new_value(Material {
            albedo_texture_id: texels_tex.id(),
            has_lighting: false,
            ..Default::default()
        });
        let geometry = stage.new_array({
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
        let transform = stage.new_value(Transform {
            scale: Vec3::new(32.0, 32.0, 1.0),
            ..Default::default()
        });
        let renderlet = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            transform_id: transform.id(),
            material_id: material.id(),
            ..Default::default()
        });
        stage.add_renderlet(&renderlet);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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
        let ctx = Context::headless(w, h);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.new_value(Camera::new(projection, view));
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

        let albedo_texture = stage.new_value(clamp_tex);
        let clamp_material = stage.new_value(Material {
            albedo_texture_id: albedo_texture.id(),
            has_lighting: false,
            ..Default::default()
        });
        let albedo_texture = stage.new_value(repeat_tex);
        let repeat_material = stage.new_value(Material {
            albedo_texture_id: albedo_texture.id(),
            has_lighting: false,
            ..Default::default()
        });
        let albedo_texture = stage.new_value(mirror_tex);
        let mirror_material = stage.new_value(Material {
            albedo_texture_id: albedo_texture.id(),
            has_lighting: false,
            ..Default::default()
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let geometry = stage.new_array({
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
        let clamp_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: clamp_material.id(),
            ..Default::default()
        });
        stage.add_renderlet(&clamp_prim);

        let repeat_transform = stage.new_value(Transform {
            translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
            ..Default::default()
        });
        let repeat_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: repeat_material.id(),
            transform_id: repeat_transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&repeat_prim);

        let mirror_transform = stage.new_value(Transform {
            translation: Vec3::new(sheet_w * 2.0 + 2.0, 0.0, 0.0),
            ..Default::default()
        });
        let mirror_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: mirror_material.id(),
            transform_id: mirror_transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&mirror_prim);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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
        let ctx = Context::headless(w, h);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));

        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let camera = stage.new_value(Camera {
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

        let clamp_albedo_texture = stage.new_value(clamp_tex);
        let clamp_material = stage.new_value(Material {
            albedo_texture_id: clamp_albedo_texture.id(),
            has_lighting: false,
            ..Default::default()
        });

        let repeat_albedo_texture = stage.new_value(repeat_tex);
        let repeat_material = stage.new_value(Material {
            albedo_texture_id: repeat_albedo_texture.id(),
            has_lighting: false,
            ..Default::default()
        });

        let mirror_albedo_texture = stage.new_value(mirror_tex);
        let mirror_material = stage.new_value(Material {
            albedo_texture_id: mirror_albedo_texture.id(),
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
            stage.new_array([tl, bl, br, tl, br, tr])
        };

        let clamp_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: clamp_material.id(),
            ..Default::default()
        });
        stage.add_renderlet(&clamp_prim);

        let repeat_transform = stage.new_value(Transform {
            translation: Vec3::new(sheet_w + 1.0, 0.0, 0.0),
            ..Default::default()
        });
        let repeat_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: repeat_material.id(),
            transform_id: repeat_transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&repeat_prim);

        let mirror_transform = stage.new_value(Transform {
            translation: Vec3::new(sheet_w * 2.0 + 2.0, 0.0, 0.0),
            ..Default::default()
        });
        let mirror_prim = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            material_id: mirror_material.id(),
            transform_id: mirror_transform.id(),
            ..Default::default()
        });
        stage.add_renderlet(&mirror_prim);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
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

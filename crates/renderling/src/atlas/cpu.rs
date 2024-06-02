use core::ops::Deref;
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
    AtlasFrame,
};

pub(crate) const ATLAS_SUGGESTED_SIZE: u32 = 2048;
pub(crate) const ATLAS_SUGGESTED_LAYERS: u32 = 8;

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
    if n == 0 {
        return vec![];
    }
    let mut input = input.into_iter();
    let mut output = vec![];
    for _ in 0..n {
        output.push(vec![]);
    }
    let mut i = 0;
    while let Some(item) = input.next() {
        // UNWRAP: safe because i % n
        output
            .get_mut(i)
            .unwrap_or_else(|| panic!("could not unwrap i:{i} n:{n}"))
            .push(item);
        i = (i + 1) % n;
    }
    output
}

/// A texture atlas packing, before it is committed to the GPU.
#[derive(Clone)]
enum Packing {
    /// A new packing.
    ///
    /// This image does not yet live on the GPU
    Img {
        /// Index of the layer within the atlas.
        layer_index: u32,
        /// Index of the frame within the layer.
        frame_index: u32,
        /// Image bytes, etc
        image: AtlasImage,
    },
    /// A previous packing.
    ///
    /// This image has already been staged on the GPU.
    GpuImg { frame: Hybrid<AtlasFrame> },
}

impl Packing {
    pub fn size(&self) -> UVec2 {
        match self {
            Packing::Img { image, .. } => image.size,
            Packing::GpuImg { frame: texture } => texture.get().size_px,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Layer {
    pub frames: Vec<Hybrid<AtlasFrame>>,
}

/// A texture atlas, used to store all the textures in a scene.
///
/// Clones of `Atlas` all point to the same internal data.
#[derive(Clone)]
pub struct Atlas {
    texture_array: Arc<RwLock<Texture>>,
    layers: Arc<RwLock<Vec<Layer>>>,
}

impl Atlas {
    /// Create a new atlas with `size` and `num_layers` layers.
    ///
    /// `size` **must be a power of two**.
    ///
    /// ## Panics
    /// Panics if `size` is _not_ a power of two.
    fn new_with_texture(texture: Texture) -> Self {
        let num_layers = texture.texture.size().depth_or_array_layers as usize;
        let layers = vec![Layer::default(); num_layers];
        log::trace!("created atlas with {num_layers} layers");
        Atlas {
            layers: Arc::new(RwLock::new(layers)),
            texture_array: Arc::new(RwLock::new(texture)),
        }
    }

    /// Create the initial texture to use.
    fn create_texture(device: &wgpu::Device, queue: &wgpu::Queue, size: wgpu::Extent3d) -> Texture {
        assert_power_of_2(size.width);
        assert_power_of_2(size.height);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas texture"),
            size,
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
        if device.features().contains(wgpu::Features::CLEAR_TEXTURE) {
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
        }
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
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, size: u32, num_layers: u32) -> Self {
        assert_power_of_2(size);
        let size = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: num_layers,
        };
        log::trace!("creating new atlas with dimensions {size:?}");
        let texture = Self::create_texture(device, queue, size);
        Self::new_with_texture(texture)
    }

    pub fn is_empty(&self) -> bool {
        // UNWRAP: panic on purpose
        let layers = self.layers.read().unwrap();
        let num_frames = layers.iter().map(|layer| layer.frames.len()).sum::<usize>();
        num_frames == 0
    }

    /// Returns a clone of the current atlas texture array.
    pub fn get_texture(&self) -> Texture {
        // UNWRAP: panic on purpose
        self.texture_array.read().unwrap().clone()
    }

    pub fn get_layers(&self) -> impl Deref<Target = Vec<Layer>> + '_ {
        // UNWRAP: panic on purpose
        self.layers.read().unwrap()
    }

    /// Reset this atlas with all new images.
    ///
    /// Any existing `Hybrid<AtlasTexture>`s will be invalidated.
    pub fn set_images(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab: &SlabAllocator<impl IsBuffer>,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasFrame>>, AtlasError> {
        log::debug!("setting images");
        {
            // UNWRAP: panic on purpose
            let texture = self.texture_array.read().unwrap();
            let mut guard = self.layers.write().unwrap();
            let layers: &mut Vec<_> = guard.as_mut();
            let new_layers =
                vec![Layer::default(); texture.texture.size().depth_or_array_layers as usize];
            let _old_layers = std::mem::replace(layers, new_layers);
        }
        self.add_images(device, queue, slab, images)
    }

    pub fn get_size(&self) -> wgpu::Extent3d {
        // UNWRAP: POP
        self.texture_array.read().unwrap().texture.size()
    }

    pub fn add_images(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        slab: &SlabAllocator<impl IsBuffer>,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasFrame>>, AtlasError> {
        // UNWRAP: POP
        let mut layers = self.layers.write().unwrap();
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        log::trace!("adding {} images to {} layers", images.len(), layers.len());
        let layer_additions: Vec<Vec<AtlasImage>> = fan_split_n(layers.len(), images);
        log::trace!(
            "extending the atlas by '{}'",
            layer_additions
                .iter()
                .map(|v| format!("{}", v.len()))
                .collect::<Vec<_>>()
                .join(",")
        );

        // UNWRAP: POP
        let mut texture_array = self.texture_array.write().unwrap();
        let extent = texture_array.texture.size();
        let new_texture_array = Self::create_texture(device, queue, extent);

        let mut hybrids = vec![];
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("atlas add images"),
        });

        // for each new layer addition, attempt to repack that layer
        for (i, additions) in layer_additions.into_iter().enumerate() {
            if additions.is_empty() {
                continue;
            }
            log::trace!("repacking layer {i} and adding {} images", additions.len());
            // UNWRAP: safe because we know this index exists from our calc above
            let layer = layers.get_mut(i).unwrap();
            let images_starting_texture_index = layer.frames.len() as u32;
            let maybe_items = crunch::pack_into_po2(
                extent.width as usize,
                layer
                    .frames
                    .iter()
                    .map(|self_texture| Packing::GpuImg {
                        frame: self_texture.clone(),
                    })
                    .chain(
                        additions
                            .into_iter()
                            .enumerate()
                            .map(|(j, image)| Packing::Img {
                                layer_index: i as u32,
                                frame_index: images_starting_texture_index + j as u32,
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
            log::trace!("  packed!");
            if let Some(items) = maybe_items {
                for crunch::PackedItem { data: item, rect } in items.items.into_iter() {
                    let offset_px = UVec2::new(rect.x as u32, rect.y as u32);
                    let size_px = UVec2::new(rect.w as u32, rect.h as u32);
                    // convert into hybrids
                    match item {
                        Packing::Img {
                            layer_index,
                            frame_index: texture_index,
                            mut image,
                        } => {
                            let frame = AtlasFrame {
                                offset_px,
                                size_px,
                                layer_index,
                                frame_index: texture_index,
                            };
                            let hybrid = slab.new_value(frame);
                            hybrids.push(hybrid.clone());
                            layer.frames.push(hybrid);

                            let bytes = convert_to_rgba8_bytes(
                                std::mem::take(&mut image.pixels),
                                image.format,
                                image.apply_linear_transfer,
                            );

                            let origin = wgpu::Origin3d {
                                x: offset_px.x,
                                y: offset_px.y,
                                z: layer_index,
                            };
                            let size = wgpu::Extent3d {
                                width: size_px.x,
                                height: size_px.y,
                                depth_or_array_layers: 1,
                            };
                            log::trace!("  writing image data to frame {texture_index} in layer {layer_index}");
                            log::trace!("    frame: {frame:?}");
                            log::trace!("    origin: {origin:?}");
                            log::trace!("    size: {size:?}");

                            // write the new image from the CPU to the new texture
                            queue.write_texture(
                                wgpu::ImageCopyTextureBase {
                                    texture: &new_texture_array.texture,
                                    mip_level: 0,
                                    origin,
                                    aspect: wgpu::TextureAspect::All,
                                },
                                &bytes,
                                wgpu::ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some(4 * size_px.x),
                                    rows_per_image: Some(size_px.y),
                                },
                                size,
                            );
                        }
                        Packing::GpuImg { frame: texture } => texture.modify(|t| {
                            t.offset_px = offset_px;
                            debug_assert_eq!(t.size_px, size_px);

                            log::trace!(
                                "  copying previous texture {} in layer {}",
                                t.frame_index,
                                t.layer_index
                            );
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
        queue.submit(Some(encoder.finish()));

        *texture_array = new_texture_array;

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
    pub fn atlas_img(&self, device: &wgpu::Device, queue: &wgpu::Queue, layer: u32) -> RgbaImage {
        let tex = self.get_texture();
        let size = tex.texture.size();
        let buffer = Texture::read_from(
            &tex.texture,
            device,
            queue,
            size.width as usize,
            size.height as usize,
            4,
            1,
            0,
            Some(wgpu::Origin3d {
                x: 0,
                y: 0,
                z: layer,
            }),
        );
        buffer.into_linear_rgba(device).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        atlas::{AtlasTexture, TextureAddressMode},
        camera::Camera,
        pbr::Material,
        stage::{Renderlet, Vertex},
        transform::Transform,
        Context,
    };
    use crabslab::{Id, Slab, SlabItem};
    use glam::{Vec2, Vec3, Vec4};

    use super::*;

    #[test]
    // Ensures that textures are packed and rendered correctly.
    fn atlas_uv_mapping() {
        log::info!("{:?}", std::env::current_dir());
        let ctx = Context::headless(32, 32);
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0));
        let (projection, view) = crate::camera::default_ortho2d(32.0, 32.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../test_img/atlas/uv_mapping.png").unwrap();
        log::info!("setting images");
        let frames = stage.set_images([dirt, sandstone, texels]).unwrap();
        log::info!("  done setting images");

        let texels_frame = &frames[2];
        let texels_tex = stage.new_value(AtlasTexture {
            frame_id: texels_frame.id(),
            ..Default::default()
        });
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

        log::info!("rendering");
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
        let frames = stage.set_images([dirt, sandstone, texels]).unwrap();
        let base_texture = AtlasTexture {
            frame_id: frames[2].id(),
            ..Default::default()
        };
        let clamp_tex = stage.new_value(base_texture);
        let mut repeat_tex = base_texture;
        repeat_tex.modes.s = TextureAddressMode::Repeat;
        repeat_tex.modes.t = TextureAddressMode::Repeat;
        let repeat_tex = stage.new_value(repeat_tex);
        let mut mirror_tex = base_texture;
        mirror_tex.modes.s = TextureAddressMode::MirroredRepeat;
        mirror_tex.modes.t = TextureAddressMode::MirroredRepeat;
        let mirror_tex = stage.new_value(mirror_tex);

        let clamp_material = stage.new_value(Material {
            albedo_texture_id: clamp_tex.id(),
            has_lighting: false,
            ..Default::default()
        });
        let repeat_material = stage.new_value(Material {
            albedo_texture_id: repeat_tex.id(),
            has_lighting: false,
            ..Default::default()
        });
        let mirror_material = stage.new_value(Material {
            albedo_texture_id: mirror_tex.id(),
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
        let frames = stage.set_images([dirt, sandstone, texels]).unwrap();

        let base_tex = AtlasTexture {
            frame_id: frames[2].id(),
            ..Default::default()
        };
        let clamp_tex = stage.new_value(base_tex);

        let mut repeat_tex = base_tex;
        repeat_tex.modes.s = TextureAddressMode::Repeat;
        repeat_tex.modes.t = TextureAddressMode::Repeat;
        let repeat_tex = stage.new_value(repeat_tex);

        let mut mirror_tex = base_tex;
        mirror_tex.modes.s = TextureAddressMode::MirroredRepeat;
        mirror_tex.modes.t = TextureAddressMode::MirroredRepeat;
        let mirror_tex = stage.new_value(mirror_tex);

        let clamp_material = stage.new_value(Material {
            albedo_texture_id: clamp_tex.id(),
            has_lighting: false,
            ..Default::default()
        });

        let repeat_material = stage.new_value(Material {
            albedo_texture_id: repeat_tex.id(),
            has_lighting: false,
            ..Default::default()
        });

        let mirror_material = stage.new_value(Material {
            albedo_texture_id: mirror_tex.id(),
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
        let mut slab = [0u32; AtlasFrame::SLAB_SIZE];
        let mut frame = AtlasFrame {
            offset_px: UVec2::ZERO,
            size_px: UVec2::ONE,
            ..Default::default()
        };
        let frame_id = Id::new(0);
        slab.write(Id::new(0), &frame);
        let tex = AtlasTexture {
            frame_id,
            ..Default::default()
        };
        assert_eq!(Vec3::ZERO, tex.uv(&slab, Vec2::ZERO, UVec2::splat(100)));
        assert_eq!(Vec3::ZERO, tex.uv(&slab, Vec2::ZERO, UVec2::splat(1)));
        assert_eq!(Vec3::ZERO, tex.uv(&slab, Vec2::ZERO, UVec2::splat(256)));
        frame.offset_px = UVec2::splat(10);
        slab.write(Id::new(0), &frame);
        assert_eq!(
            Vec2::splat(0.1).extend(0.0),
            tex.uv(&slab, Vec2::ZERO, UVec2::splat(100))
        );
    }

    #[test]
    fn can_load_and_read_atlas_texture_array() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage();
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let _frames = stage.set_images([dirt, sandstone, texels]).unwrap();

        let img = stage.atlas.atlas_img(&stage.device, &stage.queue, 0);
        img_diff::save("atlas/array0.png", img);
        let img = stage.atlas.atlas_img(&stage.device, &stage.queue, 1);
        img_diff::save("atlas/array1.png", img);
        let img = stage.atlas.atlas_img(&stage.device, &stage.queue, 2);
        img_diff::save("atlas/array2.png", img);
    }
}

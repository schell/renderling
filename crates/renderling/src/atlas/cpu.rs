use core::ops::Deref;
use std::sync::{Arc, RwLock};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WeakHybrid},
    runtime::WgpuRuntime,
};
use glam::{UVec2, UVec3};
use image::RgbaImage;
use snafu::{prelude::*, OptionExt};

use crate::{atlas::AtlasDescriptor, texture::Texture};

use super::{
    atlas_image::{convert_to_rgba8_bytes, AtlasImage},
    AtlasTexture,
};

pub(crate) const ATLAS_SUGGESTED_SIZE: u32 = 2048;
pub(crate) const ATLAS_SUGGESTED_LAYERS: u32 = 8;

#[derive(Debug, Snafu)]
pub enum AtlasError {
    #[snafu(display("Cannot pack textures.\natlas_size: {size:#?}"))]
    CannotPackTextures { size: wgpu::Extent3d },

    #[snafu(display("Missing layer {index}"))]
    MissingLayer { index: u32, images: Vec<AtlasImage> },

    #[snafu(display("Atlas size is invalid: {size:?}"))]
    Size { size: wgpu::Extent3d },

    #[snafu(display("Missing slab during staging"))]
    StagingMissingSlab,
}

/// Used to track textures internally.
#[derive(Clone, Debug)]
struct InternalAtlasTexture {
    /// Cached value.
    cache: AtlasTexture,
    weak: WeakHybrid<AtlasTexture>,
}

impl InternalAtlasTexture {
    fn from_hybrid(hat: &Hybrid<AtlasTexture>) -> Self {
        InternalAtlasTexture {
            cache: hat.get(),
            weak: WeakHybrid::from_hybrid(hat),
        }
    }

    fn has_external_references(&self) -> bool {
        self.weak.has_external_references()
    }

    fn set(&mut self, at: AtlasTexture) {
        self.cache = at;
        if let Some(hy) = self.weak.upgrade() {
            hy.set(at);
        } else if let Some(gpu) = self.weak.weak_gpu().upgrade() {
            gpu.set(at)
        } else {
            log::warn!("could not set atlas texture, lost");
        }
    }
}

pub(crate) fn check_size(size: wgpu::Extent3d) {
    let conditions = size.depth_or_array_layers >= 2
        && size.width == size.height
        && (size.width & (size.width - 1)) == 0;
    if !conditions {
        log::error!("{}", AtlasError::Size { size });
    }
}

fn fan_split_n<T>(n: usize, input: impl IntoIterator<Item = T>) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![];
    }
    let mut output = vec![];
    for _ in 0..n {
        output.push(vec![]);
    }
    let mut i = 0;
    for item in input.into_iter() {
        // UNWRAP: safe because i % n
        output
            .get_mut(i)
            .unwrap_or_else(|| panic!("could not unwrap i:{i} n:{n}"))
            .push(item);
        i = (i + 1) % n;
    }
    output
}

#[derive(Clone)]
enum AnotherPacking<'a> {
    Img {
        original_index: usize,
        image: &'a AtlasImage,
    },
    Internal(InternalAtlasTexture),
}

impl AnotherPacking<'_> {
    fn size(&self) -> UVec2 {
        match self {
            AnotherPacking::Img {
                original_index: _,
                image,
            } => image.size,
            AnotherPacking::Internal(tex) => tex.cache.size_px,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Layer {
    frames: Vec<InternalAtlasTexture>,
}

/// A texture atlas, used to store all the textures in a scene.
///
/// Clones of `Atlas` all point to the same internal data.
#[derive(Clone)]
pub struct Atlas {
    texture_array: Arc<RwLock<Texture>>,
    layers: Arc<RwLock<Vec<Layer>>>,
    label: Option<String>,
    descriptor: Hybrid<AtlasDescriptor>,
}

impl Atlas {
    const LABEL: Option<&str> = Some("atlas-texture");
    /// Create a new atlas with `size` and `num_layers` layers.
    ///
    /// `size` **must be a power of two**.
    ///
    /// ## Panics
    /// Panics if `size` is _not_ a power of two.
    pub fn new_with_texture(
        descriptor: Hybrid<AtlasDescriptor>,
        texture: Texture,
        label: Option<&str>,
    ) -> Self {
        let label = label.map(|s| s.to_owned());
        let num_layers = texture.texture.size().depth_or_array_layers as usize;
        let layers = vec![Layer::default(); num_layers];
        log::trace!("created atlas with {num_layers} layers");
        Atlas {
            layers: Arc::new(RwLock::new(layers)),
            texture_array: Arc::new(RwLock::new(texture)),
            descriptor,
            label,
        }
    }

    /// Create the initial texture to use.
    fn create_texture(
        runtime: impl AsRef<WgpuRuntime>,
        size: wgpu::Extent3d,
        format: Option<wgpu::TextureFormat>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
    ) -> Texture {
        let device = &runtime.as_ref().device;
        let queue = &runtime.as_ref().queue;
        check_size(size);
        let usage = usage.unwrap_or(wgpu::TextureUsages::empty());
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label.unwrap_or(Self::LABEL.unwrap())),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format.unwrap_or(wgpu::TextureFormat::Rgba8Unorm),
            usage: usage
                | wgpu::TextureUsages::TEXTURE_BINDING
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
    pub fn new(
        slab: &SlabAllocator<WgpuRuntime>,
        size: wgpu::Extent3d,
        format: Option<wgpu::TextureFormat>,
        label: Option<&str>,
        usage: Option<wgpu::TextureUsages>,
    ) -> Self {
        let texture = Self::create_texture(slab.runtime(), size, format, label, usage);
        let num_layers = texture.texture.size().depth_or_array_layers as usize;
        let layers = vec![Layer::default(); num_layers];
        log::trace!("creating new atlas with dimensions {size:?}, {num_layers} layers");
        let descriptor = slab.new_value(AtlasDescriptor {
            size: UVec3::new(size.width, size.height, size.depth_or_array_layers),
        });
        let label = label.map(|s| s.to_owned());

        Atlas {
            layers: Arc::new(RwLock::new(layers)),
            texture_array: Arc::new(RwLock::new(texture)),
            descriptor,
            label,
        }
    }

    pub fn len(&self) -> usize {
        // UNWRAP: panic on purpose
        let layers = self.layers.read().unwrap();
        layers.iter().map(|layer| layer.frames.len()).sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        // UNWRAP: panic on purpose
        self.len() == 0
    }

    /// Returns a reference to the current atlas texture array.
    pub fn get_texture(&self) -> impl Deref<Target = Texture> + '_ {
        // UNWRAP: panic on purpose
        self.texture_array.read().unwrap()
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
        slab: &SlabAllocator<WgpuRuntime>,
        images: &[AtlasImage],
    ) -> Result<Vec<Hybrid<AtlasTexture>>, AtlasError> {
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
        self.add_images(slab, images)
    }

    pub fn get_size(&self) -> wgpu::Extent3d {
        // UNWRAP: POP
        self.texture_array.read().unwrap().texture.size()
    }

    /// Add the given images
    // TODO: Atlas should probably clone a reference to the runtime and the slab.
    pub fn add_images<'a>(
        &self,
        slab: &SlabAllocator<WgpuRuntime>,
        images: impl IntoIterator<Item = &'a AtlasImage>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, AtlasError> {
        // UNWRAP: POP
        let mut layers = self.layers.write().unwrap();
        let mut texture_array = self.texture_array.write().unwrap();
        let extent = texture_array.texture.size();

        let newly_packed_layers = pack_images(&layers, images, extent)
            .context(CannotPackTexturesSnafu { size: extent })?;

        let mut staged = StagedResources::try_staging(
            slab.runtime(),
            extent,
            newly_packed_layers,
            Some(slab),
            &texture_array,
            self.label.as_deref(),
        )?;

        // Commit our newly staged values, now that everything is done.
        *texture_array = staged.texture;
        *layers = staged.layers;

        staged.image_additions.sort_by_key(|a| a.0);
        Ok(staged.image_additions.into_iter().map(|a| a.1).collect())
    }

    /// Resize the atlas.
    ///
    /// This also distributes the images by size among all layers in an effort to reduce
    /// the likelyhood that packing the atlas may fail.
    ///
    /// ## Errors
    /// Errors if `size` has a width or height that is not a power of two, or are unequal
    pub fn resize(
        &self,
        runtime: impl AsRef<WgpuRuntime>,
        extent: wgpu::Extent3d,
    ) -> Result<(), AtlasError> {
        let mut layers = self.layers.write().unwrap();
        let mut texture_array = self.texture_array.write().unwrap();

        let newly_packed_layers =
            pack_images(&layers, &[], extent).context(CannotPackTexturesSnafu { size: extent })?;

        let staged = StagedResources::try_staging(
            runtime,
            extent,
            newly_packed_layers,
            None::<&SlabAllocator<WgpuRuntime>>,
            &texture_array,
            self.label.as_deref(),
        )?;

        // Commit our newly staged values, now that everything is done.
        *texture_array = staged.texture;
        *layers = staged.layers;

        Ok(())
    }

    /// Perform upkeep on the atlas.
    ///
    /// This removes any `TextureFrame`s that have no references and repacks the atlas
    /// if any were removed.
    pub fn upkeep(&self, runtime: impl AsRef<WgpuRuntime>) {
        let mut total_dropped = 0;
        {
            let mut layers = self.layers.write().unwrap();
            for (i, layer) in layers.iter_mut().enumerate() {
                let mut dropped = 0;
                layer.frames.retain(|entry| {
                    if entry.has_external_references() {
                        true
                    } else {
                        dropped += 1;
                        false
                    }
                });
                total_dropped += dropped;
                if dropped > 0 {
                    log::trace!("removed {dropped} frames from layer {i}");
                }
            }

            layers.len()
        };

        if total_dropped > 0 {
            log::trace!("repacking after dropping {total_dropped} frames from the atlas");
            // UNWRAP: safe because we can only remove frames from the atlas, which should
            // only make it easier to pack.
            self.resize(runtime.as_ref(), self.get_size()).unwrap();
        }
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
    pub fn atlas_img(&self, runtime: impl AsRef<WgpuRuntime>, layer: u32) -> RgbaImage {
        let runtime = runtime.as_ref();
        let tex = self.get_texture();
        let size = tex.texture.size();
        let buffer = Texture::read_from(
            runtime,
            &tex.texture,
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
        buffer.into_linear_rgba(&runtime.device).unwrap()
    }
}

fn pack_images<'a>(
    layers: &[Layer],
    images: impl IntoIterator<Item = &'a AtlasImage>,
    extent: wgpu::Extent3d,
) -> Option<Vec<crunch::PackedItems<AnotherPacking<'a>>>> {
    let mut new_packing: Vec<AnotherPacking> = {
        let layers: Vec<_> = layers.to_vec();
        layers
            .into_iter()
            .flat_map(|layer| layer.frames)
            // Filter out any textures that have been completely dropped
            // by the user.
            .filter_map(|tex| {
                if tex.has_external_references() {
                    Some(AnotherPacking::Internal(tex))
                } else {
                    None
                }
            })
            .chain(
                images
                    .into_iter()
                    .enumerate()
                    .map(|(i, image)| AnotherPacking::Img {
                        original_index: i,
                        image,
                    }),
            )
            .collect()
    };
    new_packing.sort_by_key(|a| (a.size().length_squared()));
    let total_images = new_packing.len();
    let new_packing_layers: Vec<Vec<AnotherPacking>> =
        fan_split_n(extent.depth_or_array_layers as usize, new_packing);
    log::trace!(
        "packing {total_images} textures into {} layers",
        new_packing_layers.len()
    );
    let mut newly_packed_layers: Vec<crunch::PackedItems<_>> = vec![];
    for (i, new_layer) in new_packing_layers.into_iter().enumerate() {
        let packed = crunch::pack_into_po2(
            extent.width as usize,
            new_layer.into_iter().map(|p| {
                let size = p.size();
                crunch::Item::new(p, size.x as usize, size.y as usize, crunch::Rotation::None)
            }),
        )
        .ok()?;
        log::trace!("  layer {i} packed with {} textures", packed.items.len());
        newly_packed_layers.push(packed);
    }
    Some(newly_packed_layers)
}

/// Internal atlas resources.
struct StagedResources {
    texture: Texture,
    image_additions: Vec<(usize, Hybrid<AtlasTexture>)>,
    layers: Vec<Layer>,
}

impl StagedResources {
    /// Stage the packed images, copying them to the next texture.
    fn try_staging(
        runtime: impl AsRef<WgpuRuntime>,
        extent: wgpu::Extent3d,
        newly_packed_layers: Vec<crunch::PackedItems<AnotherPacking>>,
        slab: Option<&SlabAllocator<WgpuRuntime>>,
        old_texture_array: &Texture,
        label: Option<&str>,
    ) -> Result<Self, AtlasError> {
        let runtime = runtime.as_ref();
        let new_texture_array = Atlas::create_texture(
            runtime,
            extent,
            Some(old_texture_array.texture.format()),
            label,
            Some(old_texture_array.texture.usage()),
        );
        let mut output = vec![];
        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("atlas staging"),
            });
        let mut temporary_layers = vec![Layer::default(); extent.depth_or_array_layers as usize];
        for (layer_index, packed_items) in newly_packed_layers.into_iter().enumerate() {
            if packed_items.items.is_empty() {
                continue;
            }
            // UNWRAP: safe because we know this index exists because we created it above
            let layer = temporary_layers.get_mut(layer_index).unwrap();
            for (frame_index, crunch::PackedItem { data: item, rect }) in
                packed_items.items.into_iter().enumerate()
            {
                let offset_px = UVec2::new(rect.x as u32, rect.y as u32);
                let size_px = UVec2::new(rect.w as u32, rect.h as u32);

                match item {
                    AnotherPacking::Img {
                        original_index,
                        image,
                    } => {
                        let atlas_texture = AtlasTexture {
                            offset_px,
                            size_px,
                            frame_index: frame_index as u32,
                            layer_index: layer_index as u32,
                            ..Default::default()
                        };
                        let texture = slab
                            .context(StagingMissingSlabSnafu)?
                            .new_value(atlas_texture);
                        layer
                            .frames
                            .push(InternalAtlasTexture::from_hybrid(&texture));
                        output.push((original_index, texture));

                        let bytes = convert_to_rgba8_bytes(
                            image.pixels.clone(),
                            image.format,
                            image.apply_linear_transfer,
                        );

                        let origin = wgpu::Origin3d {
                            x: offset_px.x,
                            y: offset_px.y,
                            z: layer_index as u32,
                        };
                        let size = wgpu::Extent3d {
                            width: size_px.x,
                            height: size_px.y,
                            depth_or_array_layers: 1,
                        };
                        log::trace!(
                            "  writing image data to frame {frame_index} in layer {layer_index}"
                        );
                        log::trace!("    frame: {atlas_texture:?}");
                        log::trace!("    origin: {origin:?}");
                        log::trace!("    size: {size:?}");

                        // write the new image from the CPU to the new texture
                        runtime.queue.write_texture(
                            wgpu::ImageCopyTexture {
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
                    AnotherPacking::Internal(mut texture) => {
                        let mut t = texture.cache;
                        debug_assert_eq!(t.size_px, size_px);
                        log::trace!("  copying previous frame {t:?}",);
                        // copy the frame from the old texture to the new texture
                        // in a new destination
                        encoder.copy_texture_to_texture(
                            wgpu::ImageCopyTexture {
                                texture: &old_texture_array.texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d {
                                    x: t.offset_px.x,
                                    y: t.offset_px.y,
                                    z: t.layer_index,
                                },
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::ImageCopyTexture {
                                texture: &new_texture_array.texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d {
                                    x: offset_px.x,
                                    y: offset_px.y,
                                    z: layer_index as u32,
                                },
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::Extent3d {
                                width: size_px.x,
                                height: size_px.y,
                                depth_or_array_layers: 1,
                            },
                        );

                        t.layer_index = layer_index as u32;
                        t.frame_index = frame_index as u32;
                        t.offset_px = offset_px;
                        texture.set(t);
                        layer.frames.push(texture);
                    }
                }
            }
        }
        runtime.queue.submit(Some(encoder.finish()));

        Ok(Self {
            texture: new_texture_array,
            image_additions: output,
            layers: temporary_layers,
        })
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
    use glam::{UVec3, Vec2, Vec3, Vec4};

    use super::*;

    #[test]
    // Ensures that textures are packed and rendered correctly.
    fn atlas_uv_mapping() {
        log::info!("{:?}", std::env::current_dir());
        let ctx =
            Context::headless(32, 32).with_default_atlas_texture_size(UVec3::new(1024, 1024, 2));
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0))
            .with_bloom(false);
        let (projection, view) = crate::camera::default_ortho2d(32.0, 32.0);
        let camera = stage.new_value(Camera::new(projection, view));
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../test_img/atlas/uv_mapping.png").unwrap();
        log::info!("setting images");
        let atlas_entries = stage.set_images([dirt, sandstone, texels]).unwrap();
        log::info!("  done setting images");

        let texels_entry = &atlas_entries[2];

        let material = stage.new_value(Material {
            albedo_texture_id: texels_entry.id(),
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
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let entries = stage.set_images(std::iter::repeat(texels).take(3)).unwrap();
        let clamp_tex = &entries[0];
        let repeat_tex = &entries[1];
        repeat_tex.modify(|t| {
            t.modes.s = TextureAddressMode::Repeat;
            t.modes.t = TextureAddressMode::Repeat;
        });
        let mirror_tex = &entries[2];
        mirror_tex.modify(|t| {
            t.modes.s = TextureAddressMode::MirroredRepeat;
            t.modes.t = TextureAddressMode::MirroredRepeat;
        });

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
        let camera = stage.new_value(Camera::new(projection, view));

        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let entries = stage.set_images(std::iter::repeat(texels).take(3)).unwrap();

        let clamp_tex = &entries[0];
        let repeat_tex = &entries[1];
        repeat_tex.modify(|t| {
            t.modes.s = TextureAddressMode::Repeat;
            t.modes.t = TextureAddressMode::Repeat;
        });

        let mirror_tex = &entries[2];
        mirror_tex.modify(|t| {
            t.modes.s = TextureAddressMode::MirroredRepeat;
            t.modes.t = TextureAddressMode::MirroredRepeat;
        });

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
        let mut tex = AtlasTexture {
            offset_px: UVec2::ZERO,
            size_px: UVec2::ONE,
            ..Default::default()
        };
        assert_eq!(Vec3::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(100)));
        assert_eq!(Vec3::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(1)));
        assert_eq!(Vec3::ZERO, tex.uv(Vec2::ZERO, UVec2::splat(256)));
        tex.offset_px = UVec2::splat(10);
        assert_eq!(
            Vec2::splat(0.1).extend(0.0),
            tex.uv(Vec2::ZERO, UVec2::splat(100))
        );
    }

    #[test]
    fn can_load_and_read_atlas_texture_array() {
        // tests that the atlas lays out textures in the way we expect
        let ctx =
            Context::headless(100, 100).with_default_atlas_texture_size(UVec3::new(512, 512, 2));
        let stage = ctx.new_stage();
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let cheetah = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let _frames = stage
            .set_images([dirt, sandstone, cheetah, texels])
            .unwrap();

        let img = stage.atlas.atlas_img(&ctx, 0);
        img_diff::assert_img_eq("atlas/array0.png", img);
        let img = stage.atlas.atlas_img(&ctx, 1);
        img_diff::assert_img_eq("atlas/array1.png", img);
    }

    #[test]
    fn upkeep_trims_the_atlas() {
        // tests that Atlas::upkeep trims out unused images and repacks the atlas
        let ctx =
            Context::headless(100, 100).with_default_atlas_texture_size(UVec3::new(512, 512, 2));
        let stage = ctx.new_stage();
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let cheetah = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let mut frames = stage
            .add_images([
                dirt,
                sandstone,
                cheetah,
                texels.clone(),
                texels.clone(),
                texels.clone(),
                texels.clone(),
                texels,
            ])
            .unwrap();
        assert_eq!(8, stage.atlas.len());

        frames.pop();
        frames.pop();
        frames.pop();
        frames.pop();

        stage.atlas.upkeep(&ctx);
        assert_eq!(4, stage.atlas.len());
    }
}

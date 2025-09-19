use core::{ops::Deref, sync::atomic::AtomicUsize};
use std::sync::{Arc, Mutex, RwLock};

use craballoc::{
    prelude::{Hybrid, SlabAllocator, WeakHybrid},
    runtime::WgpuRuntime,
    slab::SlabBuffer,
};
use crabslab::Id;
use glam::{UVec2, UVec3};
use image::RgbaImage;
use snafu::{prelude::*, OptionExt};

use crate::{
    atlas::{
        shader::{AtlasBlittingDescriptor, AtlasDescriptor, AtlasTextureDescriptor},
        TextureModes,
    },
    bindgroup::ManagedBindGroup,
    texture::{self, CopiedTextureBuffer, Texture},
};

use super::atlas_image::{convert_pixels, AtlasImage};

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

    #[snafu(display("Missing bindgroup {layer}"))]
    MissingBindgroup { layer: u32 },

    #[snafu(display("{source}"))]
    Texture {
        source: crate::texture::TextureError,
    },
}

/// A staged texture in the texture atlas.
///
/// An [`AtlasTexture`] can be acquired through:
///
/// * [`Atlas::add_image`]
/// * [`Atlas::add_images`].
/// * [`Atlas::set_images`]
///
/// Clones of this type all point to the same underlying data.
///
/// Dropping all clones of this type will cause it to be unloaded from the GPU.
///
/// If a value of this type has been given to another staged resource,
/// like [`Material`](crate::material::Material), this will prevent the `AtlasTexture` from
/// being dropped and unloaded.
///
/// Internally an `AtlasTexture` holds a reference to its descriptor,
/// [`AtlasTextureDescriptor`].
#[derive(Clone)]
pub struct AtlasTexture {
    pub(crate) descriptor: Hybrid<AtlasTextureDescriptor>,
}

impl AtlasTexture {
    /// Get the GPU slab identifier of the underlying descriptor.
    ///
    /// This is for internal use.
    pub fn id(&self) -> Id<AtlasTextureDescriptor> {
        self.descriptor.id()
    }

    /// Return a copy of the underlying descriptor.
    pub fn descriptor(&self) -> AtlasTextureDescriptor {
        self.descriptor.get()
    }

    /// Return the texture modes of the underlying descriptor.
    pub fn modes(&self) -> TextureModes {
        self.descriptor.get().modes
    }

    /// Sets the texture modes of the underlying descriptor.
    ///
    /// ## Warning
    ///
    /// This also sets the modes for all clones of this value.
    pub fn set_modes(&self, modes: TextureModes) {
        self.descriptor.modify(|d| d.modes = modes);
    }
}

/// Used to track textures internally.
///
/// We need a separate struct for tracking textures because the atlas
/// reorganizes the layout (the packing) of textures each time a new
/// texture is added.
///
/// This means the textures must be updated on the GPU, but we don't
/// want these internal representations to keep unreferenced textures
/// from dropping, so we have to maintain a separate representation
/// here.
#[derive(Clone, Debug)]
struct InternalAtlasTexture {
    /// Cached value.
    cache: AtlasTextureDescriptor,
    weak: WeakHybrid<AtlasTextureDescriptor>,
}

impl InternalAtlasTexture {
    fn from_hybrid(hat: &Hybrid<AtlasTextureDescriptor>) -> Self {
        InternalAtlasTexture {
            cache: hat.get(),
            weak: WeakHybrid::from_hybrid(hat),
        }
    }

    fn has_external_references(&self) -> bool {
        self.weak.has_external_references()
    }

    fn set(&mut self, at: AtlasTextureDescriptor) {
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
    pub(crate) slab: SlabAllocator<WgpuRuntime>,
    texture_array: Arc<RwLock<Texture>>,
    layers: Arc<RwLock<Vec<Layer>>>,
    label: Option<String>,
    descriptor: Hybrid<AtlasDescriptor>,
    /// Used for user updates into the atlas by blit images into specific frames.
    blitter: AtlasBlitter,
}

impl Atlas {
    const LABEL: Option<&str> = Some("atlas-texture");

    pub fn device(&self) -> &wgpu::Device {
        self.slab.device()
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
        let blitter = AtlasBlitter::new(
            slab.device(),
            texture.texture.format(),
            wgpu::FilterMode::Linear,
        );
        Atlas {
            slab: slab.clone(),
            layers: Arc::new(RwLock::new(layers)),
            descriptor,
            label,
            blitter,
            texture_array: Arc::new(RwLock::new(texture)),
        }
    }

    pub fn descriptor_id(&self) -> Id<AtlasDescriptor> {
        self.descriptor.id()
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
    pub fn set_images(&self, images: &[AtlasImage]) -> Result<Vec<AtlasTexture>, AtlasError> {
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
        self.add_images(images)
    }

    pub fn get_size(&self) -> wgpu::Extent3d {
        // UNWRAP: POP
        self.texture_array.read().unwrap().texture.size()
    }

    /// Add the given images
    pub fn add_images<'a>(
        &self,
        images: impl IntoIterator<Item = &'a AtlasImage>,
    ) -> Result<Vec<AtlasTexture>, AtlasError> {
        // UNWRAP: POP
        let mut layers = self.layers.write().unwrap();
        let mut texture_array = self.texture_array.write().unwrap();
        let extent = texture_array.texture.size();

        let newly_packed_layers = pack_images(&layers, images, extent)
            .context(CannotPackTexturesSnafu { size: extent })?;

        let mut staged = StagedResources::try_staging(
            self.slab.runtime(),
            extent,
            newly_packed_layers,
            Some(&self.slab),
            &texture_array,
            self.label.as_deref(),
        )?;

        // Commit our newly staged values, now that everything is done.
        *texture_array = staged.texture;
        *layers = staged.layers;

        staged.image_additions.sort_by_key(|a| a.0);
        Ok(staged
            .image_additions
            .into_iter()
            .map(|a| AtlasTexture { descriptor: a.1 })
            .collect())
    }

    /// Add one image.
    ///
    /// If you have more than one image, you should use [`Atlas::add_images`], as every
    /// change in images causes a repacking, which might be expensive.
    pub fn add_image(&self, image: &AtlasImage) -> Result<AtlasTexture, AtlasError> {
        // UNWRAP: safe because we know there's at least one image
        Ok(self.add_images(Some(image))?.pop().unwrap())
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
    ///
    /// Returns `true` if the atlas texture was recreated.
    #[must_use]
    pub fn upkeep(&self, runtime: impl AsRef<WgpuRuntime>) -> bool {
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
            true
        } else {
            false
        }
    }

    /// Read the atlas image from the GPU into a [`CopiedTextureBuffer`].
    ///
    /// This is primarily for testing.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be read.
    pub fn atlas_img_buffer(
        &self,
        runtime: impl AsRef<WgpuRuntime>,
        layer: u32,
    ) -> CopiedTextureBuffer {
        let runtime = runtime.as_ref();
        let tex = self.get_texture();
        let size = tex.texture.size();
        let (channels, subpixel_bytes) =
            crate::texture::wgpu_texture_format_channels_and_subpixel_bytes_todo(
                tex.texture.format(),
            );
        log::info!("atlas_texture_format: {:#?}", tex.texture.format());
        log::info!("atlas_texture_channels: {channels:#?}");
        log::info!("atlas_texture_subpixel_bytes: {subpixel_bytes:#?}");
        CopiedTextureBuffer::read_from(
            runtime,
            &tex.texture,
            size.width as usize,
            size.height as usize,
            channels as usize,
            subpixel_bytes as usize,
            0,
            Some(wgpu::Origin3d {
                x: 0,
                y: 0,
                z: layer,
            }),
        )
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
    pub async fn atlas_img(&self, runtime: impl AsRef<WgpuRuntime>, layer: u32) -> RgbaImage {
        let runtime = runtime.as_ref();
        let buffer = self.atlas_img_buffer(runtime, layer);
        buffer.into_linear_rgba(&runtime.device).await.unwrap()
    }

    // It's ok to hold this lock because this is just for testing.
    #[allow(clippy::await_holding_lock)]
    pub async fn read_images(&self, runtime: impl AsRef<WgpuRuntime>) -> Vec<RgbaImage> {
        let mut images = vec![];
        for i in 0..self.layers.read().unwrap().len() {
            images.push(self.atlas_img(runtime.as_ref(), i as u32).await);
        }
        images
    }

    /// Update the given [`AtlasTexture`] with a [`Texture`](crate::texture::Texture).
    ///
    /// This will blit the `Texture` into the frame of the [`Atlas`] pointed to by the
    /// `AtlasTexture`.
    ///
    /// Returns a submission index that can be polled with [`wgpu::Device::poll`].
    pub fn update_texture(
        &self,
        atlas_texture: &AtlasTexture,
        source_texture: &texture::Texture,
    ) -> Result<wgpu::SubmissionIndex, AtlasError> {
        self.update_textures(Some((atlas_texture, source_texture)))
    }

    /// Update the given [`AtlasTexture`]s with [`Texture`](crate::texture::Texture)s.
    ///
    /// This will blit the `Texture` into the frame of the [`Atlas`] pointed to by the
    /// `AtlasTexture`.
    ///
    /// Returns a submission index that can be polled with [`wgpu::Device::poll`].
    pub fn update_textures<'a>(
        &self,
        updates: impl IntoIterator<Item = (&'a AtlasTexture, &'a texture::Texture)>,
    ) -> Result<wgpu::SubmissionIndex, AtlasError> {
        let updates = updates.into_iter().collect::<Vec<_>>();
        let op = AtlasBlittingOperation::new(&self.blitter, &self, updates.len());
        let runtime = self.slab.runtime();
        let mut encoder = runtime
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Atlas::update_texture"),
            });
        for (i, (atlas_texture, source_texture)) in updates.into_iter().enumerate() {
            op.run(
                &runtime,
                &mut encoder,
                source_texture,
                i as u32,
                &self,
                atlas_texture,
            )?;
        }
        Ok(runtime.queue.submit(Some(encoder.finish())))
    }

    /// Update the given [`AtlasTexture`]s with new data.
    ///
    /// This will blit the image data into the frame of the [`Atlas`] pointed to by the
    /// `AtlasTexture`.
    ///
    /// Returns a submission index that can be polled with [`wgpu::Device::poll`].
    pub fn update_images<'a>(
        &self,
        updates: impl IntoIterator<Item = (&'a AtlasTexture, impl Into<AtlasImage>)>,
    ) -> Result<wgpu::SubmissionIndex, AtlasError> {
        let (atlas_textures, images): (Vec<_>, Vec<_>) = updates.into_iter().unzip();
        let mut textures = vec![];
        for image in images.into_iter() {
            let image: AtlasImage = image.into();
            let atlas_format = self.get_texture().texture.format();
            let bytes = super::atlas_image::convert_pixels(
                image.pixels,
                image.format,
                atlas_format,
                image.apply_linear_transfer,
            );
            let (channels, subpixel_bytes) =
                texture::wgpu_texture_format_channels_and_subpixel_bytes(atlas_format)
                    .context(TextureSnafu)?;
            let texture = texture::Texture::new_with(
                self.slab.runtime(),
                Some("atlas-image-update"),
                Some(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
                None,
                atlas_format,
                channels,
                subpixel_bytes,
                image.size.x,
                image.size.y,
                1,
                &bytes,
            );
            textures.push(texture);
        }
        self.update_textures(atlas_textures.into_iter().zip(textures.iter()))
    }

    /// Update the given [`AtlasTexture`]s with new data.
    ///
    /// This will blit the image data into the frame of the [`Atlas`] pointed to by the
    /// `AtlasTexture`.
    ///
    /// Returns a submission index that can be polled with [`wgpu::Device::poll`].
    pub fn update_image(
        &self,
        atlas_texture: &AtlasTexture,
        source_image: impl Into<AtlasImage>,
    ) -> Result<wgpu::SubmissionIndex, AtlasError> {
        self.update_images(Some((atlas_texture, source_image)))
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
        log::trace!("  packing layer {i} into power of 2 {}", extent.width);
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
    image_additions: Vec<(usize, Hybrid<AtlasTextureDescriptor>)>,
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
                        let atlas_texture = AtlasTextureDescriptor {
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

                        let bytes = convert_pixels(
                            image.pixels.clone(),
                            image.format,
                            old_texture_array.texture.format(),
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
                            wgpu::TexelCopyTextureInfo {
                                texture: &new_texture_array.texture,
                                mip_level: 0,
                                origin,
                                aspect: wgpu::TextureAspect::All,
                            },
                            &bytes,
                            wgpu::TexelCopyBufferLayout {
                                offset: 0,
                                bytes_per_row: Some(4 * size_px.x),
                                rows_per_image: Some(size_px.y),
                            },
                            size,
                        );
                    }
                    AnotherPacking::Internal(mut texture) => {
                        let prev_t = texture.cache;
                        let mut t = texture.cache;
                        debug_assert_eq!(t.size_px, size_px);
                        // copy the frame from the old texture to the new texture
                        // in a new destination
                        encoder.copy_texture_to_texture(
                            wgpu::TexelCopyTextureInfo {
                                texture: &old_texture_array.texture,
                                mip_level: 0,
                                origin: t.origin(),
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::TexelCopyTextureInfo {
                                texture: &new_texture_array.texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d {
                                    x: offset_px.x,
                                    y: offset_px.y,
                                    z: layer_index as u32,
                                },
                                aspect: wgpu::TextureAspect::All,
                            },
                            t.size_as_extent(),
                        );

                        t.layer_index = layer_index as u32;
                        t.frame_index = frame_index as u32;
                        t.offset_px = offset_px;

                        log::trace!(
                            "  copied previous frame {}",
                            pretty_assertions::Comparison::new(&prev_t, &t)
                        );

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

/// A reusable blitting operation that copies a source texture into a specific
/// frame of an [`Atlas`].
#[derive(Clone)]
pub struct AtlasBlittingOperation {
    atlas_slab_buffer: Arc<Mutex<SlabBuffer<wgpu::Buffer>>>,
    pipeline: Arc<wgpu::RenderPipeline>,
    bindgroups: Arc<Vec<ManagedBindGroup>>,
    bindgroup_layout: Arc<wgpu::BindGroupLayout>,
    sampler: Arc<wgpu::Sampler>,
    from_texture_id: Arc<AtomicUsize>,
    pub(crate) desc: Hybrid<AtlasBlittingDescriptor>,
}

impl AtlasBlittingOperation {
    pub fn new(
        blitter: &AtlasBlitter,
        into_atlas: &Atlas,
        source_layers: usize,
    ) -> AtlasBlittingOperation {
        AtlasBlittingOperation {
            desc: into_atlas
                .slab
                .new_value(AtlasBlittingDescriptor::default()),
            atlas_slab_buffer: Arc::new(Mutex::new(into_atlas.slab.commit())),
            bindgroups: {
                let mut bgs = vec![];
                for _ in 0..source_layers {
                    bgs.push(ManagedBindGroup::default());
                }
                Arc::new(bgs)
            },
            pipeline: blitter.pipeline.clone(),
            sampler: blitter.sampler.clone(),
            bindgroup_layout: blitter.bind_group_layout.clone(),
            from_texture_id: Default::default(),
        }
    }

    /// Copies the data from texture this [`AtlasBlittingOperation`] was created with
    /// into the atlas.
    ///
    /// The original items used to create the inner bind group are required here, to
    /// determine whether or not the bind group needs to be invalidated.
    pub fn run(
        &self,
        runtime: impl AsRef<WgpuRuntime>,
        encoder: &mut wgpu::CommandEncoder,
        from_texture: &crate::texture::Texture,
        from_layer: u32,
        to_atlas: &Atlas,
        atlas_texture: &AtlasTexture,
    ) -> Result<(), AtlasError> {
        let runtime = runtime.as_ref();

        // update the descriptor
        self.desc.set(AtlasBlittingDescriptor {
            atlas_texture_id: atlas_texture.id(),
            atlas_desc_id: to_atlas.descriptor_id(),
        });
        // sync the update
        let _ = to_atlas.slab.commit();

        let to_atlas_texture = to_atlas.get_texture();
        let mut atlas_slab_buffer = self.atlas_slab_buffer.lock().unwrap();
        let atlas_slab_invalid = atlas_slab_buffer.update_if_invalid();
        let from_texture_has_been_replaced = {
            let prev_id = self
                .from_texture_id
                .swap(from_texture.id(), std::sync::atomic::Ordering::Relaxed);
            from_texture.id() != prev_id
        };
        let should_invalidate = atlas_slab_invalid || from_texture_has_been_replaced;
        let view = from_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("atlas-blitting"),
                base_array_layer: from_layer,
                array_layer_count: Some(1),
                dimension: Some(wgpu::TextureViewDimension::D2),
                ..Default::default()
            });
        let bindgroup = self
            .bindgroups
            .get(from_layer as usize)
            .context(MissingBindgroupSnafu { layer: from_layer })?
            .get(should_invalidate, || {
                runtime
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("atlas-blitting"),
                        layout: &self.bindgroup_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(
                                    atlas_slab_buffer.deref().as_entire_buffer_binding(),
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&self.sampler),
                            },
                        ],
                    })
            });

        let atlas_texture = atlas_texture.descriptor();
        let atlas_view = to_atlas_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("atlas-blitting"),
                format: None,
                dimension: Some(wgpu::TextureViewDimension::D2),
                usage: None,
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: atlas_texture.layer_index,
                array_layer_count: Some(1),
            });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("atlas-blitter"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &atlas_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
        let id = self.desc.id();
        pass.draw(0..6, id.inner()..id.inner() + 1);
        Ok(())
    }
}

/// A texture blitting utility.
///
/// [`AtlasBlitter`] copies textures to specific frames within the texture atlas.
#[derive(Clone)]
pub struct AtlasBlitter {
    pipeline: Arc<wgpu::RenderPipeline>,
    bind_group_layout: Arc<wgpu::BindGroupLayout>,
    sampler: Arc<wgpu::Sampler>,
}

impl AtlasBlitter {
    /// Creates a new [`AtlasBlitter`].
    ///
    /// # Arguments
    /// - `device` - A [`wgpu::Device`]
    /// - `format` - The [`wgpu::TextureFormat`] of the atlas being updated.
    /// - `mag_filter` - The filtering algorithm to use when magnifying.
    ///   This is used when the input source is smaller than the destination.
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        mag_filter: wgpu::FilterMode,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("atlas-blitter"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("atlas-blitter"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float {
                            filterable: mag_filter == wgpu::FilterMode::Linear,
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(if mag_filter == wgpu::FilterMode::Linear {
                        wgpu::SamplerBindingType::Filtering
                    } else {
                        wgpu::SamplerBindingType::NonFiltering
                    }),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("atlas-blitter"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex = crate::linkage::atlas_blit_vertex::linkage(device);
        let fragment = crate::linkage::atlas_blit_fragment::linkage(device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("atlas-blitter"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: Some(vertex.entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragment.module,
                entry_point: Some(fragment.entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline: pipeline.into(),
            bind_group_layout: bind_group_layout.into(),
            sampler: sampler.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        atlas::{shader::AtlasTextureDescriptor, TextureAddressMode},
        context::Context,
        geometry::Vertex,
        material::Materials,
        test::BlockOnFuture,
    };
    use glam::{UVec3, Vec2, Vec3, Vec4};

    use super::*;

    #[test]
    // Ensures that textures are packed and rendered correctly.
    fn atlas_uv_mapping() {
        log::info!("{:?}", std::env::current_dir());
        let ctx = Context::headless(32, 32)
            .block()
            .with_default_atlas_texture_size(UVec3::new(1024, 1024, 2));
        let stage = ctx
            .new_stage()
            .with_background_color(Vec3::splat(0.0).extend(1.0))
            .with_bloom(false);
        let (projection, view) = crate::camera::default_ortho2d(32.0, 32.0);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let texels = AtlasImage::from_path("../../test_img/atlas/uv_mapping.png").unwrap();
        log::info!("setting images");
        let atlas_entries = stage.set_images([dirt, sandstone, texels]).unwrap();
        log::info!("  done setting images");

        let texels_entry = &atlas_entries[2];

        let _rez = stage
            .new_primitive()
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(texels_entry)
                    .with_has_lighting(false),
            )
            .with_vertices(stage.new_vertices({
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
            }))
            .with_transform(stage.new_transform().with_scale(Vec3::new(32.0, 32.0, 1.0)));

        log::info!("rendering");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let ctx = Context::headless(w, h).block();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));
        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let entries = stage.set_images(std::iter::repeat_n(texels, 3)).unwrap();
        let clamp_tex = &entries[0];
        let repeat_tex = &entries[1];
        repeat_tex.set_modes(TextureModes {
            s: TextureAddressMode::Repeat,
            t: TextureAddressMode::Repeat,
        });
        let mirror_tex = &entries[2];
        mirror_tex.set_modes(TextureModes {
            s: TextureAddressMode::MirroredRepeat,
            t: TextureAddressMode::MirroredRepeat,
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let geometry = stage.new_vertices({
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
        let _clamp_rez = stage
            .new_primitive()
            .with_vertices(&geometry)
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(clamp_tex)
                    .with_has_lighting(false),
            );

        let _repeat_rez = stage
            .new_primitive()
            .with_transform(stage.new_transform().with_translation(Vec3::new(
                sheet_w + 1.0,
                0.0,
                0.0,
            )))
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(repeat_tex)
                    .with_has_lighting(false),
            )
            .with_vertices(&geometry);

        let _mirror_rez = stage
            .new_primitive()
            .with_transform(stage.new_transform().with_translation(Vec3::new(
                sheet_w * 2.0 + 2.0,
                0.0,
                0.0,
            )))
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(mirror_tex)
                    .with_has_lighting(false),
            )
            .with_vertices(geometry);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
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
        let ctx = Context::headless(w, h).block();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::new(1.0, 1.0, 0.0, 1.0));

        let (projection, view) = crate::camera::default_ortho2d(w as f32, h as f32);
        let _camera = stage
            .new_camera()
            .with_projection_and_view(projection, view);

        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let entries = stage.set_images(std::iter::repeat_n(texels, 3)).unwrap();

        let clamp_tex = &entries[0];
        let repeat_tex = &entries[1];
        repeat_tex.set_modes(TextureModes {
            s: TextureAddressMode::Repeat,
            t: TextureAddressMode::Repeat,
        });

        let mirror_tex = &entries[2];
        mirror_tex.set_modes(TextureModes {
            s: TextureAddressMode::MirroredRepeat,
            t: TextureAddressMode::MirroredRepeat,
        });

        let sheet_w = sheet_w as f32;
        let sheet_h = sheet_h as f32;
        let geometry = stage.new_vertices({
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
            [tl, bl, br, tl, br, tr]
        });
        let _clamp_prim = stage
            .new_primitive()
            .with_vertices(&geometry)
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(clamp_tex)
                    .with_has_lighting(false),
            );

        let _repeat_rez = stage
            .new_primitive()
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(repeat_tex)
                    .with_has_lighting(false),
            )
            .with_transform(stage.new_transform().with_translation(Vec3::new(
                sheet_w + 1.0,
                0.0,
                0.0,
            )))
            .with_vertices(&geometry);

        let _mirror_rez = stage
            .new_primitive()
            .with_material(
                stage
                    .new_material()
                    .with_albedo_texture(mirror_tex)
                    .with_has_lighting(false),
            )
            .with_transform(stage.new_transform().with_translation(Vec3::new(
                sheet_w * 2.0 + 2.0,
                0.0,
                0.0,
            )))
            .with_vertices(&geometry);

        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().block().unwrap();
        img_diff::assert_img_eq("atlas/negative_uv_wrapping.png", img);
    }

    #[test]
    fn transform_uvs_for_atlas() {
        let mut tex = AtlasTextureDescriptor {
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
        let ctx = Context::headless(100, 100)
            .block()
            .with_default_atlas_texture_size(UVec3::new(512, 512, 2));
        let stage = ctx.new_stage();
        let dirt = AtlasImage::from_path("../../img/dirt.jpg").unwrap();
        let sandstone = AtlasImage::from_path("../../img/sandstone.png").unwrap();
        let cheetah = AtlasImage::from_path("../../img/cheetah.jpg").unwrap();
        let texels = AtlasImage::from_path("../../img/happy_mac.png").unwrap();
        let _frames = stage
            .set_images([dirt, sandstone, cheetah, texels])
            .unwrap();
        let materials: &Materials = stage.as_ref();
        let img = materials.atlas().atlas_img(&ctx, 0).block();
        img_diff::assert_img_eq("atlas/array0.png", img);
        let img = materials.atlas().atlas_img(&ctx, 1).block();
        img_diff::assert_img_eq("atlas/array1.png", img);
    }

    #[test]
    fn upkeep_trims_the_atlas() {
        // tests that Atlas::upkeep trims out unused images and repacks the atlas
        let ctx = Context::headless(100, 100)
            .block()
            .with_default_atlas_texture_size(UVec3::new(512, 512, 2));
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
        let materials: &Materials = stage.as_ref();
        assert_eq!(8, materials.atlas().len());

        frames.pop();
        frames.pop();
        frames.pop();
        frames.pop();

        let _ = materials.atlas().upkeep(&ctx);
        assert_eq!(4, materials.atlas().len());
    }
}

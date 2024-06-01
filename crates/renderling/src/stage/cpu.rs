//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage [`Renderlet`]s for rendering.
use core::sync::atomic::{AtomicU32, Ordering};
use crabslab::{Array, Id, Slab, SlabItem};
use rustc_hash::FxHashMap;
use snafu::Snafu;
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crate::{
    atlas::{Atlas, AtlasError, AtlasImage, AtlasImageError, AtlasTexture},
    bloom::Bloom,
    camera::Camera,
    pbr::{debug::DebugMode, light::Light, PbrConfig},
    skybox::Skybox,
    slab::*,
    stage::Renderlet,
    texture::{DepthTexture, Texture},
    tonemapping::Tonemapping,
    transform::Transform,
};

use super::*;

#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

/// The count at which a `Renderlet` will be automatically removed from the
/// stage and recycled.
///
/// This constant is exposed for downstream authors to write renderers that
/// may need to store the renderlet in a system.
///
/// * +1 for the clone in [`Stage`]'s renderlets (added via
///   [`Stage::add_renderlet`]), required to draw
/// * +1 for the clone in [`Stage`]'s [`SlabAllocator`]'s update sources, needed
///   to update the GPU
pub const RENDERLET_STRONG_COUNT_LOWER_BOUND: usize = 2;

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    // TODO: Add `Indirect` to `StageDrawStrategy` which uses `RenderPass::multi_draw_indirect`
    Direct(Vec<Hybrid<Renderlet>>),
}

fn create_msaa_textureview(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("stage msaa render target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

fn create_stage_render_pipeline(
    device: &wgpu::Device,
    multisample_count: u32,
) -> wgpu::RenderPipeline {
    log::trace!("creating stage render pipeline");
    let label = Some("stage render");
    let vertex_linkage = crate::linkage::renderlet_vertex::linkage(device);
    let fragment_linkage = crate::linkage::renderlet_fragment::linkage(device);
    let stage_slab_buffers_layout = crate::linkage::slab_bindgroup_layout(device);
    let atlas_and_skybox_layout = crate::linkage::atlas_and_skybox_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&stage_slab_buffers_layout, &atlas_and_skybox_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_linkage.module,
            entry_point: vertex_linkage.entry_point,
            buffers: &[],
            compilation_options: Default::default(),
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            mask: !0,
            alpha_to_coverage_enabled: false,
            count: multisample_count,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_linkage.module,
            entry_point: fragment_linkage.entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        multiview: None,
    })
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
///
/// ## Note
/// Only available on the CPU. Not available in shaders.
#[derive(Clone)]
pub struct Stage {
    pub(crate) device: Arc<wgpu::Device>,
    pub(crate) queue: Arc<wgpu::Queue>,

    pub(crate) mngr: SlabAllocator<wgpu::Buffer>,

    pub(crate) pbr_config: Hybrid<PbrConfig>,
    pub(crate) lights: Arc<RwLock<HybridArray<Id<Light>>>>,

    pub(crate) stage_pipeline: Arc<RwLock<wgpu::RenderPipeline>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,

    pub(crate) hdr_texture: Arc<RwLock<Texture>>,
    pub(crate) depth_texture: Arc<RwLock<Texture>>,
    pub(crate) msaa_render_target: Arc<RwLock<Option<wgpu::TextureView>>>,
    pub(crate) msaa_sample_count: Arc<AtomicU32>,

    pub(crate) atlas: Atlas,
    pub(crate) bloom: Bloom,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) tonemapping: Tonemapping,
    pub(crate) background_color: Arc<RwLock<wgpu::Color>>,

    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) has_bloom: Arc<AtomicBool>,

    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,

    pub(crate) draws: Arc<RwLock<StageDrawStrategy>>,
}

impl Deref for Stage {
    type Target = SlabAllocator<wgpu::Buffer>;

    fn deref(&self) -> &Self::Target {
        &self.mngr
    }
}

impl Stage {
    /// Create a new stage.
    pub fn new(ctx: &crate::Context) -> Self {
        let (device, queue) = ctx.get_device_and_queue_owned();
        let resolution @ UVec2 { x: w, y: h } = ctx.get_size();
        let atlas = Atlas::empty(&device, &queue);
        let mngr = SlabAllocator::default();
        let pbr_config = mngr.new_value(PbrConfig {
            atlas_size: atlas.get_size(),
            resolution,
            ..Default::default()
        });
        let multisample_count = 1;
        let lights = mngr.new_array(vec![Id::<Light>::NONE; 16]);
        let hdr_texture = Arc::new(RwLock::new(Texture::create_hdr_texture(
            &device,
            w,
            h,
            multisample_count,
        )));
        let depth_texture = Arc::new(RwLock::new(Texture::create_depth_texture(
            &device,
            w,
            h,
            multisample_count,
        )));
        let msaa_render_target = Default::default();
        // UNWRAP: safe because no other references at this point (created above^)
        let bloom = Bloom::new(&device, &queue, &hdr_texture.read().unwrap());
        let tonemapping = Tonemapping::new(
            &device,
            &queue,
            ctx.get_render_target().format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );

        Self {
            mngr,
            pbr_config,
            lights: Arc::new(RwLock::new(lights)),

            stage_pipeline: Arc::new(RwLock::new(create_stage_render_pipeline(
                &device,
                multisample_count,
            ))),
            atlas,
            skybox: Arc::new(RwLock::new(Skybox::empty(&device, &queue))),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            bloom,
            tonemapping,
            has_bloom: AtomicBool::from(true).into(),
            buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            draws: Arc::new(RwLock::new(StageDrawStrategy::Direct(vec![]))),
            hdr_texture,
            depth_texture,
            msaa_render_target,
            msaa_sample_count: Arc::new(multisample_count.into()),
            background_color: Arc::new(RwLock::new(wgpu::Color::TRANSPARENT)),
            device,
            queue,
        }
    }

    pub fn get_device_and_queue_owned(&self) -> (Arc<wgpu::Device>, Arc<wgpu::Queue>) {
        (self.device.clone(), self.queue.clone())
    }

    pub fn set_background_color(&self, color: impl Into<Vec4>) {
        let color = color.into();
        *self.background_color.write().unwrap() = wgpu::Color {
            r: color.x as f64,
            g: color.y as f64,
            b: color.z as f64,
            a: color.w as f64,
        };
    }

    pub fn with_background_color(self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn set_msaa_sample_count(&self, multisample_count: u32) {
        let multisample_count = multisample_count.max(1);
        let prev_multisample_count = self
            .msaa_sample_count
            .swap(multisample_count, Ordering::Relaxed);
        if prev_multisample_count == multisample_count {
            log::warn!("set_multisample_count: multisample count is unchanged, noop");
            return;
        }

        log::debug!("setting multisample count to {multisample_count}");
        // UNWRAP: POP
        *self.stage_pipeline.write().unwrap() =
            create_stage_render_pipeline(&self.device, multisample_count);
        let size = self.get_size();
        // UNWRAP: POP
        *self.depth_texture.write().unwrap() =
            Texture::create_depth_texture(&self.device, size.x, size.y, multisample_count);
        // UNWRAP: POP
        let format = self.hdr_texture.read().unwrap().texture.format();
        *self.msaa_render_target.write().unwrap() = if multisample_count == 1 {
            None
        } else {
            Some(create_msaa_textureview(
                &self.device,
                size.x,
                size.y,
                format,
                multisample_count,
            ))
        };

        // Invalidate the textures bindgroup - it must be recreated
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    /// Set the MSAA multisample count.
    ///
    /// Set to `1` to disable MSAA. Setting to `0` will be treated the same as
    /// setting to `1`.
    pub fn with_msaa_sample_count(self, multisample_count: u32) -> Self {
        self.set_msaa_sample_count(multisample_count);
        self
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugMode) {
        self.pbr_config.modify(|cfg| cfg.debug_mode = debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&self, use_lighting: bool) {
        self.pbr_config
            .modify(|cfg| cfg.has_lighting = use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the lights to use for shading.
    pub fn set_lights(&self, lights: impl IntoIterator<Item = Id<Light>>) {
        log::trace!("setting lights");
        let lights = self.mngr.new_array(lights);
        self.pbr_config.modify(|cfg| {
            cfg.light_array = lights.array();
        });
        // UNWRAP: POP
        *self.lights.write().unwrap() = lights;
    }

    pub fn get_size(&self) -> UVec2 {
        // UNWRAP: panic on purpose
        let hdr = self.hdr_texture.read().unwrap();
        let w = hdr.width();
        let h = hdr.height();
        UVec2::new(w, h)
    }

    pub fn set_size(&self, size: UVec2) {
        if size == self.get_size() {
            return;
        }

        self.pbr_config.modify(|cfg| cfg.resolution = size);
        let hdr_texture = Texture::create_hdr_texture(&self.device, size.x, size.y, 1);
        let sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        if let Some(msaa_view) = self.msaa_render_target.write().unwrap().as_mut() {
            *msaa_view = create_msaa_textureview(
                &self.device,
                size.x,
                size.y,
                hdr_texture.texture.format(),
                sample_count,
            );
        }

        // UNWRAP: panic on purpose
        *self.depth_texture.write().unwrap() =
            Texture::create_depth_texture(&self.device, size.x, size.y, sample_count);
        self.bloom
            .set_hdr_texture(&self.device, &self.queue, &hdr_texture);
        self.tonemapping.set_hdr_texture(&self.device, &hdr_texture);
        *self.hdr_texture.write().unwrap() = hdr_texture;

        let _ = self.skybox_bindgroup.lock().unwrap().take();
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    pub fn with_size(self, size: UVec2) -> Self {
        self.set_size(size);
        self
    }

    /// Add an image to the set of atlas images.
    ///
    /// Adding an image can be quite expensive, as it requires repacking all
    /// previous images. For that reason it's better to use
    /// [`Stage::set_images`] if you have all the images beforehand.
    pub fn add_image(&self, image: impl Into<AtlasImage>) -> Result<AtlasTexture, StageError> {
        let preview = self
            .atlas
            .repack_preview(&self.device, Some(image.into()))?;
        self.atlas.repack(&self.device, &self.queue, preview)?;

        let size = self.atlas.get_size();
        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();
        // The atlas size must be reset
        self.pbr_config.modify(|cfg| cfg.atlas_size = size);

        let texture = self
            .atlas
            .frames()
            .into_iter()
            .last()
            .map(|(i, (offset_px, size_px))| AtlasTexture {
                offset_px,
                size_px,
                frame_index: i,
                ..Default::default()
            })
            .unwrap();

        Ok(texture)
    }

    /// Set the images to use for the atlas.
    ///
    /// Resets the atlas, packing it with the given images and returning a
    /// vector of the textures ready to be staged.
    ///
    /// ## WARNING
    /// This invalidates any currently staged `AtlasTexture`s.
    pub fn set_images(
        &self,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Vec<AtlasTexture>, StageError> {
        self.atlas.reset(&self.device, &self.queue, images)?;

        let size = self.atlas.get_size();

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();
        // The atlas size must be reset
        self.pbr_config.modify(|cfg| cfg.atlas_size = size);

        let textures = self
            .atlas
            .frames()
            .into_iter()
            .map(|(i, (offset_px, size_px))| AtlasTexture {
                offset_px,
                size_px,
                frame_index: i,
                ..Default::default()
            })
            .collect();
        Ok(textures)
    }

    /// Set the skybox.
    pub fn set_skybox(&self, skybox: Skybox) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.skybox.write().unwrap();
        *guard = skybox;
        self.has_skybox
            .store(true, std::sync::atomic::Ordering::Relaxed);
        *self.skybox_bindgroup.lock().unwrap() = None;
        *self.textures_bindgroup.lock().unwrap() = None;
    }

    /// Turn the bloom effect on or off.
    pub fn set_has_bloom(&self, has_bloom: bool) {
        self.has_bloom
            .store(has_bloom, std::sync::atomic::Ordering::Relaxed);
    }

    /// Turn the bloom effect on or off.
    pub fn with_bloom(self, has_bloom: bool) -> Self {
        self.set_has_bloom(has_bloom);
        self
    }

    /// Set the amount of bloom that is mixed in with the input image.
    ///
    /// Defaults to `0.04`.
    pub fn set_bloom_mix_strength(&self, strength: f32) {
        self.bloom.set_mix_strength(strength);
    }

    pub fn with_bloom_mix_strength(self, strength: f32) -> Self {
        self.set_bloom_mix_strength(strength);
        self
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn set_bloom_filter_radius(&self, filter_radius: f32) {
        self.bloom.set_filter_radius(filter_radius);
    }

    /// Sets the bloom filter radius, in pixels.
    ///
    /// Default is `1.0`.
    pub fn with_bloom_filter_radius(self, filter_radius: f32) -> Self {
        self.set_bloom_filter_radius(filter_radius);
        self
    }

    /// Return the skybox render pipeline, creating it if necessary.
    fn get_skybox_pipeline_and_bindgroup(
        &self,
        slab_buffer: &wgpu::Buffer,
    ) -> (Arc<wgpu::RenderPipeline>, Arc<wgpu::BindGroup>) {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(
                crate::skybox::create_skybox_render_pipeline(
                    &self.device,
                    Texture::HDR_TEXTURE_FORMAT,
                )
                .0,
            );
            *pipeline = Some(p.clone());
            p
        };
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.skybox_bindgroup.lock().unwrap();
        let bindgroup = if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                &self.device,
                slab_buffer,
                &self.skybox.read().unwrap().environment_cubemap,
            ));
            *bindgroup = Some(bg.clone());
            bg
        };
        (pipeline, bindgroup)
    }

    fn get_slab_buffers_bindgroup(&self, slab_buffer: &wgpu::Buffer) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.buffers_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new({
                let device: &wgpu::Device = &self.device;
                crate::linkage::slab_bindgroup(
                    device,
                    slab_buffer,
                    // UNWRAP: POP
                    &self.stage_pipeline.read().unwrap().get_bind_group_layout(0),
                )
            });
            *bindgroup = Some(b.clone());
            b
        }
    }

    fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(crate::linkage::atlas_and_skybox_bindgroup(
                &self.device,
                &{
                    let this = &self;
                    this.stage_pipeline.clone()
                }
                .read()
                // UNWRAP: POP
                .unwrap()
                .get_bind_group_layout(1),
                // UNWRAP: POP
                &self.atlas,
                &self.skybox.read().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Adds a renderlet to the internal list of renderlets to be drawn each
    /// frame.
    ///
    /// This makes an internal clone of the renderlet.
    ///
    /// If you drop the renderlet and no other references are kept, it will be
    /// removed automatically from the internal list and will cease to be
    /// drawn each frame.
    pub fn add_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.push(renderlet.clone());
            }
        }
    }

    /// Erase the given renderlet from the internal list of renderlets to be
    /// drawn each frame.
    pub fn remove_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        let id = renderlet.id();
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.retain(|hybrid| hybrid.id() != id);
            }
        }
    }

    /// Returns a clone of all the staged [`Renderlet`]s.
    pub fn get_renderlets(&self) -> Vec<Hybrid<Renderlet>> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.draws.read().unwrap();
        match draws.deref() {
            StageDrawStrategy::Direct(units) => units.clone(),
        }
    }

    /// Re-order the renderlets.
    ///
    /// This determines the order in which they are drawn each frame.
    ///
    /// If the `order` iterator is missing any renderlet ids, those missing
    /// renderlets will be drawn _before_ the ordered ones, in no particular
    /// order.
    pub fn reorder_renderlets(&self, order: impl IntoIterator<Item = Id<Renderlet>>) {
        // UNWRAP: panic on purpose
        let mut renderlets = self.draws.write().unwrap();
        match renderlets.deref_mut() {
            StageDrawStrategy::Direct(rs) => {
                let mut ordered = vec![];
                let mut m =
                    FxHashMap::from_iter(std::mem::take(rs).into_iter().map(|r| (r.id(), r)));
                for id in order.into_iter() {
                    if let Some(renderlet) = m.remove(&id) {
                        ordered.push(renderlet);
                    }
                }
                rs.extend(m.into_values());
                rs.extend(ordered);
            }
        }
    }

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            device: self.device.clone(),
            queue: self.queue.clone(),
            texture: self.depth_texture.read().unwrap().clone(),
        }
    }

    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
        camera_id: Id<Camera>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(&self.device, &self.queue, hdr, camera_id))
    }

    pub fn new_nested_transform(&self) -> NestedTransform {
        NestedTransform::new(&self.mngr)
    }

    fn tick_internal(&self) -> Arc<wgpu::Buffer> {
        {
            let mut draw_guard = self.draws.write().unwrap();
            match draw_guard.deref_mut() {
                StageDrawStrategy::Direct(draws) => {
                    draws.retain(|d| d.strong_count() > RENDERLET_STRONG_COUNT_LOWER_BOUND);
                }
            }
        }
        if let Some(new_slab_buffer) = self.mngr.upkeep((
            &self.device,
            &self.queue,
            Some("stage render upkeep"),
            wgpu::BufferUsages::empty(),
        )) {
            // invalidate our bindgroups, etc
            let _ = self.skybox_bindgroup.lock().unwrap().take();
            let _ = self.buffers_bindgroup.lock().unwrap().take();
            new_slab_buffer
        } else {
            // UNWRAP: safe because we called `SlabManager::upkeep` above^, which ensures
            // the buffer exists
            self.mngr.get_buffer().unwrap()
        }
    }

    /// Ticks the stage, synchronizing changes with the GPU.
    ///
    /// It's good to call this after dropping assets to free up space on the
    /// slab.
    pub fn tick(&self) {
        let _ = self.tick_internal();
    }

    pub fn render(&self, view: &wgpu::TextureView) {
        {
            log::trace!("rendering the stage");
            let label = Some("stage render");
            // UNWRAP: POP
            let background_color = *self.background_color.read().unwrap();
            let slab_buffer = self.tick_internal();
            // UNWRAP: POP
            let pipeline = self.stage_pipeline.read().unwrap();
            // UNWRAP: POP
            let msaa_target = self.msaa_render_target.read().unwrap();
            let slab_buffers_bindgroup = self.get_slab_buffers_bindgroup(&slab_buffer);
            let textures_bindgroup = self.get_textures_bindgroup();
            let has_skybox = self.has_skybox.load(std::sync::atomic::Ordering::Relaxed);
            let may_skybox_pipeline_and_bindgroup = if has_skybox {
                Some(self.get_skybox_pipeline_and_bindgroup(&slab_buffer))
            } else {
                None
            };

            // UNWRAP: if we can't read we want to panic.
            let draws = self.draws.read().unwrap();

            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let hdr_texture = self.hdr_texture.read().unwrap();
                let depth_texture = self.depth_texture.read().unwrap();
                let ops = wgpu::Operations {
                    load: wgpu::LoadOp::Clear(background_color),
                    store: wgpu::StoreOp::Store,
                };
                let color_attachment = if let Some(msaa_view) = msaa_target.as_ref() {
                    wgpu::RenderPassColorAttachment {
                        ops,
                        view: msaa_view,
                        resolve_target: Some(&hdr_texture.view),
                    }
                } else {
                    wgpu::RenderPassColorAttachment {
                        ops,
                        view: &hdr_texture.view,
                        resolve_target: None,
                    }
                };
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });
                render_pass.set_pipeline(&pipeline);
                render_pass.set_bind_group(0, &slab_buffers_bindgroup, &[]);
                render_pass.set_bind_group(1, &textures_bindgroup, &[]);
                match draws.deref() {
                    StageDrawStrategy::Direct(units) => {
                        for hybrid in units {
                            let rlet = hybrid.get();
                            if rlet.visible {
                                let vertex_range = if rlet.indices_array.is_null() {
                                    0..rlet.vertices_array.len() as u32
                                } else {
                                    0..rlet.indices_array.len() as u32
                                };
                                let id = hybrid.id();
                                let instance_range = id.inner()..id.inner() + 1;
                                log::trace!(
                                    "drawing vertices {vertex_range:?} and instances \
                                     {instance_range:?}"
                                );
                                render_pass.draw(vertex_range, instance_range);
                            }
                        }
                    } /* render_pass.multi_draw_indirect(&indirect_buffer, 0,
                       * stage.number_of_indirect_draws()); */
                }

                if let Some((pipeline, bindgroup)) = may_skybox_pipeline_and_bindgroup.as_ref() {
                    log::trace!("rendering skybox");
                    // UNWRAP: if we can't acquire the lock we want to panic.
                    let skybox = self.skybox.read().unwrap();
                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, bindgroup, &[]);
                    render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
                }
            }
            self.queue.submit(std::iter::once(encoder.finish()));
        }

        // then render bloom
        if self.has_bloom.load(Ordering::Relaxed) {
            log::trace!("stage bloom");
            self.bloom.bloom(&self.device, &self.queue);
        } else {
            // copy the input hdr texture to the bloom mix texture
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("no bloom copy"),
                });
            let bloom_mix_texture = self.bloom.get_mix_texture();
            encoder.copy_texture_to_texture(
                wgpu::ImageCopyTexture {
                    texture: &self.hdr_texture.read().unwrap().texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyTexture {
                    texture: &bloom_mix_texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: bloom_mix_texture.width(),
                    height: bloom_mix_texture.height(),
                    depth_or_array_layers: 1,
                },
            );
            self.queue.submit(std::iter::once(encoder.finish()));
        }

        // then render tonemapping
        log::trace!("stage tonemapping");
        self.tonemapping.render(&self.device, &self.queue, view);
    }
}

/// Manages scene heirarchy on the [`Stage`].
///
/// Clones all reference the same nested transform.
#[derive(Clone)]
pub struct NestedTransform {
    global_transform_id: Id<Transform>,
    local_transform: Arc<RwLock<Transform>>,

    notifier_index: usize,
    notify: async_channel::Sender<usize>,

    children: Arc<RwLock<Vec<NestedTransform>>>,
    parent: Arc<RwLock<Option<NestedTransform>>>,
}

impl Drop for NestedTransform {
    fn drop(&mut self) {
        let _ = self.notify.try_send(self.notifier_index);
    }
}

impl core::fmt::Debug for NestedTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let children = self
            .children
            .read()
            .unwrap()
            .iter()
            .map(|nt| nt.global_transform_id)
            .collect::<Vec<_>>();
        let parent = self
            .parent
            .read()
            .unwrap()
            .as_ref()
            .map(|nt| nt.global_transform_id);
        f.debug_struct("NestedTransform")
            .field("local_transform", &self.local_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl UpdatesSlab for NestedTransform {
    fn id(&self) -> usize {
        self.notifier_index
    }

    fn u32_array(&self) -> Array<u32> {
        Array::new(
            self.global_transform_id.inner(),
            Transform::SLAB_SIZE as u32,
        )
    }

    fn strong_count(&self) -> usize {
        Arc::strong_count(&self.local_transform)
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        let transform = self.get_global_transform();
        let array = self.u32_array();
        let mut elements = vec![0u32; Transform::SLAB_SIZE];
        elements.write_indexed(&transform, 0);
        vec![SlabUpdate { array, elements }]
    }
}

impl NestedTransform {
    pub fn new(mngr: &SlabAllocator<impl IsBuffer>) -> Self {
        let id = mngr.allocate::<Transform>();
        let notifier_index = mngr.next_update_k();

        let nested = NestedTransform {
            global_transform_id: id,
            local_transform: Arc::new(RwLock::new(Transform::default())),

            notifier_index,
            notify: mngr.notifier.0.clone(),

            children: Default::default(),
            parent: Default::default(),
        };

        mngr.insert_update_source(notifier_index, nested.clone());

        nested.mark_dirty();
        nested
    }

    fn mark_dirty(&self) {
        // UNWRAP: safe because it's unbounded
        self.notify.try_send(self.notifier_index).unwrap();
        for child in self.children.read().unwrap().iter() {
            child.mark_dirty();
        }
    }

    /// Modify the local transform.
    ///
    /// This automatically applies the local transform to the global transform.
    pub fn modify(&self, f: impl Fn(&mut Transform)) {
        // UNWRAP: panic on purpose
        let mut local_guard = self.local_transform.write().unwrap();
        f(&mut local_guard);
        self.mark_dirty();
    }

    /// Set the local transform.
    ///
    /// This automatically applies the local transform to the global transform.
    pub fn set(&self, transform: Transform) {
        self.modify(move |t| {
            *t = transform;
        });
    }

    pub fn get(&self) -> Transform {
        *self.local_transform.read().unwrap()
    }

    pub fn get_global_transform(&self) -> Transform {
        let maybe_parent_guard = self.parent.read().unwrap();
        let transform = self.get();
        let parent_transform = maybe_parent_guard
            .as_ref()
            .map(|parent| parent.get_global_transform())
            .unwrap_or_default();
        Transform::from(Mat4::from(parent_transform) * Mat4::from(transform))
    }

    pub fn global_transform_id(&self) -> Id<Transform> {
        self.global_transform_id
    }

    pub fn add_child(&self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        node.mark_dirty();
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform_id == node.global_transform_id {
                node.mark_dirty();
                let _ = node.parent.write().unwrap().take();
                false
            } else {
                true
            }
        });
    }
}

#[cfg(test)]
mod test {
    use std::sync::Mutex;

    use crabslab::{Array, Slab};
    use glam::{Mat4, Vec2, Vec3};

    use crate::{
        camera::Camera,
        stage::{
            cpu::{SlabAllocator, UpdatesSlab},
            NestedTransform, Renderlet, Vertex,
        },
        transform::Transform,
        Context,
    };

    #[test]
    fn vertex_slab_roundtrip() {
        let initial_vertices = {
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
            vec![tl, bl, br, tl, br, tr]
        };
        let mut slab = [0u32; 256];
        slab.write_indexed_slice(&initial_vertices, 0);
        let vertices = slab.read_vec(Array::<Vertex>::new(0, initial_vertices.len() as u32));
        pretty_assertions::assert_eq!(initial_vertices, vertices);
    }

    #[test]
    fn matrix_subtraction_sanity() {
        let m = Mat4::IDENTITY - Mat4::IDENTITY;
        assert_eq!(Mat4::ZERO, m);
    }

    #[test]
    fn can_global_transform_calculation() {
        let slab = SlabAllocator::<Mutex<Vec<u32>>>::default();
        // Setup a hierarchy of transforms
        let root = NestedTransform::new(&slab);
        let child = NestedTransform::new(&slab);
        child.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });
        let grandchild = NestedTransform::new(&slab);
        grandchild.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });

        // Build the hierarchy
        root.add_child(&child);
        child.add_child(&grandchild);

        // Calculate global transforms
        let grandchild_global_transform = grandchild.get_global_transform();

        // Assert that the global transform is as expected
        assert_eq!(
            grandchild_global_transform.translation.x, 2.0,
            "Grandchild's global translation should   2.0 along the x-axis"
        );
    }

    #[test]
    fn can_msaa() {
        let ctx = Context::headless(100, 100);
        let stage = ctx
            .new_stage()
            .with_background_color([1.0, 1.0, 1.0, 1.0])
            .with_lighting(false);
        let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
        let vertices = stage.new_array([
            Vertex::default()
                .with_position([10.0, 10.0, 0.0])
                .with_color([0.0, 1.0, 1.0, 1.0]),
            Vertex::default()
                .with_position([10.0, 90.0, 0.0])
                .with_color([1.0, 1.0, 0.0, 1.0]),
            Vertex::default()
                .with_position([90.0, 10.0, 0.0])
                .with_color([1.0, 0.0, 1.0, 1.0]),
        ]);
        let triangle = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: vertices.array(),
            ..Default::default()
        });
        stage.add_renderlet(&triangle);

        log::debug!("rendering without msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/without.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
        log::debug!("  all good!");

        stage.set_msaa_sample_count(4);
        log::debug!("rendering with msaa");
        let frame = ctx.get_next_frame().unwrap();
        stage.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq_cfg(
            "msaa/with.png",
            img,
            img_diff::DiffCfg {
                pixel_threshold: img_diff::LOW_PIXEL_THRESHOLD,
                ..Default::default()
            },
        );
        frame.present();
    }

    #[test]
    fn has_consistent_stage_renderlet_strong_count() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage();
        let r = stage.new_value(Renderlet::default());
        stage.add_renderlet(&r);
        let expected_strong_count = 1 // for `r` here
            + super::RENDERLET_STRONG_COUNT_LOWER_BOUND;
        assert_eq!(expected_strong_count, r.strong_count());
    }
}

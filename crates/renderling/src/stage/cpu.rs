//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage [`Renderlet`]s for rendering.
use core::sync::atomic::{AtomicU32, Ordering};
use craballoc::prelude::*;
use crabslab::Id;
use snafu::Snafu;
use std::{
    ops::Deref,
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crate::{
    atlas::{Atlas, AtlasError, AtlasImage, AtlasImageError, AtlasTexture},
    bindgroup::ManagedBindGroup,
    bloom::Bloom,
    camera::Camera,
    debug::DebugOverlay,
    draw::DrawCalls,
    light::Lighting,
    pbr::{debug::DebugChannel, PbrConfig},
    skybox::{Skybox, SkyboxRenderPipeline},
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

/// Performs a rendering of an entire scene, given the resources at hand.
pub struct StageRendering<'a> {
    // TODO: include the rest of the needed paramaters from `stage`, and then remove `stage`
    pub stage: &'a Stage,
    pub pipeline: &'a wgpu::RenderPipeline,
    pub color_attachment: wgpu::RenderPassColorAttachment<'a>,
    pub depth_stencil_attachment: wgpu::RenderPassDepthStencilAttachment<'a>,
}

impl StageRendering<'_> {
    /// Run the stage rendering.
    ///
    /// Returns the queue submission index and the indirect draw buffer, if available.
    pub fn run(self) -> (wgpu::SubmissionIndex, Option<SlabBuffer<wgpu::Buffer>>) {
        self.stage.tick_internal();
        self.stage.lighting.upkeep();

        let mut draw_calls = self.stage.draw_calls.write().unwrap();
        let depth_texture = self.stage.depth_texture.read().unwrap();
        // UNWRAP: safe because we know the depth texture format will always match
        let maybe_indirect_buffer = draw_calls.pre_draw(&depth_texture).unwrap();
        {
            log::info!("rendering");
            let label = Some("stage render");

            log::info!("getting slab buffers bindgroup");
            let slab_buffers_bindgroup = {
                log::info!("getting write lock");
                let mut stage_slab_buffer = self.stage.stage_slab_buffer.write().unwrap();
                log::info!("got write lock");
                let should_invalidate_buffers_bindgroup = stage_slab_buffer.update_if_invalid();
                self.stage
                    .buffers_bindgroup
                    .get(should_invalidate_buffers_bindgroup, || {
                        log::info!("renewing invalid stage slab buffers bindgroup");
                        crate::linkage::slab_bindgroup(
                            self.stage.device(),
                            &stage_slab_buffer,
                            // UNWRAP: POP
                            &self
                                .stage
                                .stage_pipeline
                                .read()
                                .unwrap()
                                .get_bind_group_layout(0),
                        )
                    })
            };

            log::info!("getting stage slab buffer");
            let stage_slab_buffer = self.stage.stage_slab_buffer.read().unwrap();
            let textures_bindgroup = self.stage.get_textures_bindgroup();
            log::info!("got stage slab buffer and shadow map depth texture");

            let light_bindgroup = self.stage.lighting.get_bindgroup();
            let has_skybox = self.stage.has_skybox.load(Ordering::Relaxed);
            let may_skybox_pipeline_and_bindgroup = if has_skybox {
                Some(
                    self.stage
                        .get_skybox_pipeline_and_bindgroup(&stage_slab_buffer),
                )
            } else {
                None
            };

            let mut encoder = self
                .stage
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(self.color_attachment)],
                    depth_stencil_attachment: Some(self.depth_stencil_attachment),
                    ..Default::default()
                });

                render_pass.set_pipeline(self.pipeline);
                render_pass.set_bind_group(0, Some(slab_buffers_bindgroup.as_ref()), &[]);
                render_pass.set_bind_group(1, Some(textures_bindgroup.as_ref()), &[]);
                render_pass.set_bind_group(2, Some(&light_bindgroup), &[]);
                draw_calls.draw(&mut render_pass);

                if let Some((pipeline, bindgroup)) = may_skybox_pipeline_and_bindgroup.as_ref() {
                    // UNWRAP: if we can't acquire the lock we want to panic.
                    let skybox = self.stage.skybox.read().unwrap();
                    render_pass.set_pipeline(&pipeline.pipeline);
                    render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
                    render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
                }
            }
            let sindex = self.stage.queue().submit(std::iter::once(encoder.finish()));
            (sindex, maybe_indirect_buffer)
        }
    }
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
///
/// ## Note
/// Only available on the CPU. Not available in shaders.
#[derive(Clone)]
pub struct Stage {
    pub(crate) mngr: SlabAllocator<WgpuRuntime>,

    pub(crate) pbr_config: Hybrid<PbrConfig>,

    pub(crate) stage_pipeline: Arc<RwLock<wgpu::RenderPipeline>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<SkyboxRenderPipeline>>>>,

    pub(crate) hdr_texture: Arc<RwLock<Texture>>,
    pub(crate) depth_texture: Arc<RwLock<Texture>>,
    pub(crate) msaa_render_target: Arc<RwLock<Option<wgpu::TextureView>>>,
    pub(crate) msaa_sample_count: Arc<AtomicU32>,
    pub(crate) clear_color_attachments: Arc<AtomicBool>,
    pub(crate) clear_depth_attachments: Arc<AtomicBool>,

    pub(crate) atlas: Atlas,
    pub(crate) bloom: Bloom,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) lighting: Lighting,
    pub(crate) tonemapping: Tonemapping,
    pub(crate) debug_overlay: DebugOverlay,
    pub(crate) background_color: Arc<RwLock<wgpu::Color>>,

    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) has_debug_overlay: Arc<AtomicBool>,

    pub(crate) stage_slab_buffer: Arc<RwLock<SlabBuffer<wgpu::Buffer>>>,

    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) buffers_bindgroup: ManagedBindGroup,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,

    pub(crate) draw_calls: Arc<RwLock<DrawCalls>>,
}

impl Deref for Stage {
    type Target = SlabAllocator<WgpuRuntime>;

    fn deref(&self) -> &Self::Target {
        &self.mngr
    }
}

impl Stage {
    pub fn create_stage_render_pipeline(
        device: &wgpu::Device,
        fragment_color_format: wgpu::TextureFormat,
        multisample_count: u32,
    ) -> wgpu::RenderPipeline {
        log::trace!("creating stage render pipeline");
        let label = Some("stage render");
        let vertex_linkage = crate::linkage::renderlet_vertex::linkage(device);
        let fragment_linkage = crate::linkage::renderlet_fragment::linkage(device);
        let stage_slab_buffers_layout = crate::linkage::slab_bindgroup_layout(device);
        let atlas_and_skybox_layout = crate::linkage::atlas_and_skybox_bindgroup_layout(device);
        let light_bindgroup_layout = crate::light::Lighting::create_bindgroup_layout(device);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[
                &stage_slab_buffers_layout,
                &atlas_and_skybox_layout,
                &light_bindgroup_layout,
            ],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: Some(vertex_linkage.entry_point),
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
                entry_point: Some(fragment_linkage.entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: fragment_color_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            multiview: None,
            cache: None,
        })
    }

    /// Create a new stage.
    pub fn new(ctx: &crate::Context) -> Self {
        let runtime = ctx.runtime();
        let device = &runtime.device;
        let resolution @ UVec2 { x: w, y: h } = ctx.get_size();
        let atlas_size = *ctx.atlas_size.read().unwrap();

        let mngr =
            SlabAllocator::new_with_label(runtime, wgpu::BufferUsages::empty(), Some("stage-slab"));
        let pbr_config = mngr.new_value(PbrConfig {
            atlas_size: UVec2::new(atlas_size.width, atlas_size.height),
            resolution,
            ..Default::default()
        });

        let atlas = Atlas::new(&mngr, atlas_size, None, Some("stage-atlas"), None);
        let multisample_count = 1;
        let hdr_texture = Arc::new(RwLock::new(Texture::create_hdr_texture(
            device,
            w,
            h,
            multisample_count,
        )));
        let depth_texture =
            Texture::create_depth_texture(device, w, h, multisample_count, Some("stage-depth"));
        let msaa_render_target = Default::default();
        // UNWRAP: safe because no other references at this point (created above^)
        let bloom = Bloom::new(ctx, &hdr_texture.read().unwrap());
        let tonemapping = Tonemapping::new(
            runtime,
            ctx.get_render_target().format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );
        let stage_pipeline = Self::create_stage_render_pipeline(
            device,
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let stage_slab_buffer = mngr.commit();

        let lighting = Lighting::new(&mngr);

        Self {
            draw_calls: Arc::new(RwLock::new(DrawCalls::new(
                ctx,
                true,
                &mngr.commit(),
                &depth_texture,
            ))),
            lighting,
            depth_texture: Arc::new(RwLock::new(depth_texture)),
            buffers_bindgroup: ManagedBindGroup::from(crate::linkage::slab_bindgroup(
                device,
                &stage_slab_buffer,
                &stage_pipeline.get_bind_group_layout(0),
            )),
            stage_slab_buffer: Arc::new(RwLock::new(stage_slab_buffer)),
            mngr,
            pbr_config,
            stage_pipeline: Arc::new(RwLock::new(stage_pipeline)),
            atlas,
            skybox: Arc::new(RwLock::new(Skybox::empty(runtime))),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            bloom,
            tonemapping,
            has_bloom: AtomicBool::from(true).into(),
            textures_bindgroup: Default::default(),
            debug_overlay: DebugOverlay::new(device, ctx.get_render_target().format()),
            has_debug_overlay: Arc::new(false.into()),
            hdr_texture,
            msaa_render_target,
            msaa_sample_count: Arc::new(multisample_count.into()),
            clear_color_attachments: Arc::new(true.into()),
            clear_depth_attachments: Arc::new(true.into()),
            background_color: Arc::new(RwLock::new(wgpu::Color::TRANSPARENT)),
        }
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
        *self.stage_pipeline.write().unwrap() = Self::create_stage_render_pipeline(
            self.device(),
            wgpu::TextureFormat::Rgba16Float,
            multisample_count,
        );
        let size = self.get_size();
        // UNWRAP: POP
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            multisample_count,
            Some("stage-depth"),
        );
        // UNWRAP: POP
        let format = self.hdr_texture.read().unwrap().texture.format();
        *self.msaa_render_target.write().unwrap() = if multisample_count == 1 {
            None
        } else {
            Some(create_msaa_textureview(
                self.device(),
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

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_color_attachments(&self, should_clear: bool) {
        self.clear_color_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_color_attachments(self, should_clear: bool) -> Self {
        self.set_clear_color_attachments(should_clear);
        self
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn set_clear_depth_attachments(&self, should_clear: bool) {
        self.clear_depth_attachments
            .store(should_clear, Ordering::Relaxed);
    }

    /// Set whether color attachments are cleared before rendering.
    pub fn with_clear_depth_attachments(self, should_clear: bool) -> Self {
        self.set_clear_depth_attachments(should_clear);
        self
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugChannel) {
        self.pbr_config.modify(|cfg| cfg.debug_channel = debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugChannel) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether to render the debug overlay.
    pub fn set_use_debug_overlay(&self, use_debug_overlay: bool) {
        self.has_debug_overlay
            .store(use_debug_overlay, std::sync::atomic::Ordering::Relaxed);
    }

    /// Set whether to render the debug overlay.
    pub fn with_debug_overlay(self, use_debug_overlay: bool) -> Self {
        self.set_use_debug_overlay(use_debug_overlay);
        self
    }

    /// Set whether to use frustum culling on GPU before drawing.
    ///
    /// This defaults to `true`.
    pub fn set_use_frustum_culling(&self, use_frustum_culling: bool) {
        self.pbr_config
            .modify(|cfg| cfg.perform_frustum_culling = use_frustum_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_frustum_culling(self, use_frustum_culling: bool) -> Self {
        self.set_use_frustum_culling(use_frustum_culling);
        self
    }

    /// Set whether to use occlusion culling on GPU before drawing.
    ///
    /// This defaults to `false`.
    ///
    /// ## Warning
    ///
    /// Occlusion culling is a feature in development. YMMV.
    pub fn set_use_occlusion_culling(&self, use_occlusion_culling: bool) {
        self.pbr_config
            .modify(|cfg| cfg.perform_occlusion_culling = use_occlusion_culling);
    }

    /// Set whether to render the debug overlay.
    pub fn with_occlusion_culling(self, use_occlusion_culling: bool) -> Self {
        self.set_use_occlusion_culling(use_occlusion_culling);
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

    /// Set whether to use vertex skinning.
    pub fn set_has_vertex_skinning(&self, use_skinning: bool) {
        self.pbr_config
            .modify(|cfg| cfg.has_skinning = use_skinning);
    }

    /// Set whether to use vertex skinning.
    pub fn with_vertex_skinning(self, use_skinning: bool) -> Self {
        self.set_has_vertex_skinning(use_skinning);
        self
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
        let hdr_texture = Texture::create_hdr_texture(self.device(), size.x, size.y, 1);
        let sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        if let Some(msaa_view) = self.msaa_render_target.write().unwrap().as_mut() {
            *msaa_view = create_msaa_textureview(
                self.device(),
                size.x,
                size.y,
                hdr_texture.texture.format(),
                sample_count,
            );
        }

        // UNWRAP: panic on purpose
        *self.depth_texture.write().unwrap() = Texture::create_depth_texture(
            self.device(),
            size.x,
            size.y,
            sample_count,
            Some("stage-depth"),
        );
        self.bloom.set_hdr_texture(self.runtime(), &hdr_texture);
        self.tonemapping
            .set_hdr_texture(self.device(), &hdr_texture);
        *self.hdr_texture.write().unwrap() = hdr_texture;

        let _ = self.skybox_bindgroup.lock().unwrap().take();
        let _ = self.textures_bindgroup.lock().unwrap().take();
    }

    pub fn with_size(self, size: UVec2) -> Self {
        self.set_size(size);
        self
    }

    /// Set the size of the atlas.
    ///
    /// This will cause a repacking.
    pub fn set_atlas_size(&self, size: wgpu::Extent3d) -> Result<(), StageError> {
        log::info!("resizing atlas to {size:?}");
        self.atlas.resize(self.runtime(), size)?;
        Ok(())
    }

    /// Add images to the set of atlas images.
    ///
    /// Adding an image can be quite expensive, as it requires creating a new texture
    /// array for the atlas and repacking all previous images. For that reason it is
    /// good to batch images to reduce the number of calls.
    ///
    /// This returns a vector of [`Hybrid<AtlasTexture>`](e), which
    /// represent each image in the atlas maintained on the GPU. Dropping these entries
    /// will invalidate those images and cause the atlas to be repacked, and any GPU
    /// references to the underlying [`AtlasFrame`](f) and [`AtlasTexture`](t) will also
    /// be invalidated.
    ///
    /// [e]: crate::atlas::Hybrid<AtlasTexture>
    /// [f]: crate::atlas::AtlasFrame
    /// [t]: crate::atlas::AtlasTexture
    pub fn add_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.atlas.add_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
    }

    /// Clear all images from the atlas.
    ///
    /// ## WARNING
    /// This invalidates any previously staged `AtlasFrame`s.
    pub fn clear_images(&self) -> Result<(), StageError> {
        let none = Option::<AtlasImage>::None;
        let _ = self.set_images(none)?;
        Ok(())
    }

    /// Set the images to use for the atlas.
    ///
    /// Resets the atlas, packing it with the given images and returning a
    /// vector of the frames already staged.
    ///
    /// ## WARNING
    /// This invalidates any previously staged `Hybrid<AtlasTexture>`s.
    pub fn set_images(
        &self,
        images: impl IntoIterator<Item = impl Into<AtlasImage>>,
    ) -> Result<Vec<Hybrid<AtlasTexture>>, StageError> {
        let images = images.into_iter().map(|i| i.into()).collect::<Vec<_>>();
        let frames = self.atlas.set_images(&images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();

        Ok(frames)
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
    ) -> (Arc<SkyboxRenderPipeline>, Arc<wgpu::BindGroup>) {
        let msaa_sample_count = self.msaa_sample_count.load(Ordering::Relaxed);
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline_guard = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline_guard.as_mut() {
            if pipeline.msaa_sample_count() != msaa_sample_count {
                *pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                    self.device(),
                    Texture::HDR_TEXTURE_FORMAT,
                    Some(msaa_sample_count),
                ));
            }
            pipeline.clone()
        } else {
            let pipeline = Arc::new(crate::skybox::create_skybox_render_pipeline(
                self.device(),
                Texture::HDR_TEXTURE_FORMAT,
                Some(msaa_sample_count),
            ));
            *pipeline_guard = Some(pipeline.clone());
            pipeline
        };
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.skybox_bindgroup.lock().unwrap();
        let bindgroup = if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                self.device(),
                slab_buffer,
                &self.skybox.read().unwrap().environment_cubemap,
            ));
            *bindgroup = Some(bg.clone());
            bg
        };
        (pipeline, bindgroup)
    }

    fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(crate::linkage::atlas_and_skybox_bindgroup(
                self.device(),
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
    /// If you drop the renderlet and no other references are kept, it will be
    /// removed automatically from the internal list and will cease to be
    /// drawn each frame.
    pub fn add_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draw_calls.write().unwrap();
        draws.add_renderlet(renderlet);
    }

    /// Erase the given renderlet from the internal list of renderlets to be
    /// drawn each frame.
    pub fn remove_renderlet(&self, renderlet: &Hybrid<Renderlet>) {
        let mut draws = self.draw_calls.write().unwrap();
        draws.remove_renderlet(renderlet);
    }

    /// Re-order the renderlets.
    ///
    /// This determines the order in which they are drawn each frame.
    ///
    /// If the `order` iterator is missing any renderlet ids, those missing
    /// renderlets will be drawn _before_ the ordered ones, in no particular
    /// order.
    pub fn reorder_renderlets(&self, order: impl IntoIterator<Item = Id<Renderlet>>) {
        log::trace!("reordering renderlets");
        // UNWRAP: panic on purpose
        let mut guard = self.draw_calls.write().unwrap();
        guard.reorder_renderlets(order);
    }

    /// Iterator over all staged [`Renderlet`]s.
    ///
    /// This iterator returns `Renderlets` wrapped in `WeakHybrid`, because they
    /// are stored by weak references internally.
    ///
    /// You should have references of your own, but this is here as a convenience
    /// method, and is used internally.
    pub fn renderlets_iter(&self) -> impl Iterator<Item = WeakHybrid<Renderlet>> {
        // UNWRAP: panic on purpose
        let guard = self.draw_calls.read().unwrap();
        guard.renderlets_iter().collect::<Vec<_>>().into_iter()
    }

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            runtime: self.runtime().clone(),
            texture: self.depth_texture.read().unwrap().texture.clone(),
        }
    }

    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
        camera_id: Id<Camera>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(self.runtime(), hdr, camera_id))
    }

    pub fn new_nested_transform(&self) -> NestedTransform {
        NestedTransform::new(&self.mngr)
    }

    fn tick_internal(&self) {
        self.draw_calls.write().unwrap().upkeep();

        let stage_slab_buffer = self.mngr.commit();
        if stage_slab_buffer.is_new_this_commit() {
            // invalidate our bindgroups, etc
            // TODO: we shouldn't have to invalidate skybox and other bindgroups,
            // they can do this in their own `run` functions
            let _ = self.skybox_bindgroup.lock().unwrap().take();
        }
    }

    /// Ticks the stage, synchronizing changes with the GPU.
    ///
    /// It's good to call this after dropping assets to free up space on the
    /// slab.
    pub fn tick(&self) {
        self.atlas.upkeep(self.runtime());
        self.tick_internal();
    }

    pub fn lighting(&self) -> &Lighting {
        &self.lighting
    }

    pub fn render(&self, view: &wgpu::TextureView) {
        // UNWRAP: POP
        let background_color = *self.background_color.read().unwrap();
        // UNWRAP: POP
        let msaa_target = self.msaa_render_target.read().unwrap();
        let clear_colors = self.clear_color_attachments.load(Ordering::Relaxed);
        let hdr_texture = self.hdr_texture.read().unwrap();

        let mk_ops = |store| wgpu::Operations {
            load: if clear_colors {
                wgpu::LoadOp::Clear(background_color)
            } else {
                wgpu::LoadOp::Load
            },
            store,
        };
        let render_pass_color_attachment = if let Some(msaa_view) = msaa_target.as_ref() {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Discard),
                view: msaa_view,
                resolve_target: Some(&hdr_texture.view),
            }
        } else {
            wgpu::RenderPassColorAttachment {
                ops: mk_ops(wgpu::StoreOp::Store),
                view: &hdr_texture.view,
                resolve_target: None,
            }
        };

        let depth_texture = self.depth_texture.read().unwrap();
        let clear_depth = self.clear_depth_attachments.load(Ordering::Relaxed);
        let render_pass_depth_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: if clear_depth {
                    wgpu::LoadOp::Clear(1.0)
                } else {
                    wgpu::LoadOp::Load
                },
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        };
        let pipeline_guard = self.stage_pipeline.read().unwrap();
        let (_submission_index, maybe_indirect_buffer) = StageRendering {
            pipeline: &pipeline_guard,
            stage: self,
            color_attachment: render_pass_color_attachment,
            depth_stencil_attachment: render_pass_depth_attachment,
        }
        .run();

        // then render bloom
        if self.has_bloom.load(Ordering::Relaxed) {
            self.bloom.bloom(self.device(), self.queue());
        } else {
            // copy the input hdr texture to the bloom mix texture
            let mut encoder =
                self.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("no bloom copy"),
                    });
            let bloom_mix_texture = self.bloom.get_mix_texture();
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.hdr_texture.read().unwrap().texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
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
            self.queue().submit(std::iter::once(encoder.finish()));
        }

        // then render tonemapping
        self.tonemapping.render(self.device(), self.queue(), view);

        // then render the debug overlay
        if self.has_debug_overlay.load(Ordering::Relaxed) {
            if let Some(indirect_draw_buffer) = maybe_indirect_buffer {
                self.debug_overlay.render(
                    self.device(),
                    self.queue(),
                    view,
                    &self.stage_slab_buffer.read().unwrap(),
                    &indirect_draw_buffer,
                );
            }
        }
    }

    pub fn render_old(&self, view: &wgpu::TextureView) {
        self.tick_internal();
        self.lighting.upkeep();

        let mut draw_calls = self.draw_calls.write().unwrap();
        let depth_texture = self.depth_texture.read().unwrap();
        // UNWRAP: safe because we know the depth texture format will always match
        let maybe_indirect_buffer = draw_calls.pre_draw(&depth_texture).unwrap();
        {
            log::info!("rendering");
            let label = Some("stage render");

            log::info!("getting slab buffers bindgroup");
            let slab_buffers_bindgroup = {
                log::info!("getting write lock");
                let mut stage_slab_buffer = self.stage_slab_buffer.write().unwrap();
                log::info!("got write lock");
                let should_invalidate_buffers_bindgroup = stage_slab_buffer.update_if_invalid();
                self.buffers_bindgroup
                    .get(should_invalidate_buffers_bindgroup, || {
                        log::info!("renewing invalid stage slab buffers bindgroup");
                        crate::linkage::slab_bindgroup(
                            self.device(),
                            &stage_slab_buffer,
                            // UNWRAP: POP
                            &self.stage_pipeline.read().unwrap().get_bind_group_layout(0),
                        )
                    })
            };

            log::info!("getting stage slab buffer");
            let stage_slab_buffer = self.stage_slab_buffer.read().unwrap();
            let textures_bindgroup = self.get_textures_bindgroup();
            log::info!("got stage slab buffer and shadow map depth texture");

            // UNWRAP: POP
            let background_color = *self.background_color.read().unwrap();
            // UNWRAP: POP
            let pipeline = self.stage_pipeline.read().unwrap();
            // UNWRAP: POP
            let msaa_target = self.msaa_render_target.read().unwrap();

            let light_bindgroup = self.lighting.get_bindgroup();
            let has_skybox = self.has_skybox.load(Ordering::Relaxed);
            let may_skybox_pipeline_and_bindgroup = if has_skybox {
                Some(self.get_skybox_pipeline_and_bindgroup(&stage_slab_buffer))
            } else {
                None
            };
            let clear_colors = self.clear_color_attachments.load(Ordering::Relaxed);
            let clear_depth = self.clear_depth_attachments.load(Ordering::Relaxed);

            let mut encoder = self
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let hdr_texture = self.hdr_texture.read().unwrap();

                let mk_ops = |store| wgpu::Operations {
                    load: if clear_colors {
                        wgpu::LoadOp::Clear(background_color)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store,
                };
                let color_attachment = if let Some(msaa_view) = msaa_target.as_ref() {
                    wgpu::RenderPassColorAttachment {
                        ops: mk_ops(wgpu::StoreOp::Discard),
                        view: msaa_view,
                        resolve_target: Some(&hdr_texture.view),
                    }
                } else {
                    wgpu::RenderPassColorAttachment {
                        ops: mk_ops(wgpu::StoreOp::Store),
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
                            load: if clear_depth {
                                wgpu::LoadOp::Clear(1.0)
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });

                render_pass.set_pipeline(&pipeline);
                render_pass.set_bind_group(0, Some(slab_buffers_bindgroup.as_ref()), &[]);
                render_pass.set_bind_group(1, Some(textures_bindgroup.as_ref()), &[]);
                render_pass.set_bind_group(2, Some(&light_bindgroup), &[]);
                draw_calls.draw(&mut render_pass);

                if let Some((pipeline, bindgroup)) = may_skybox_pipeline_and_bindgroup.as_ref() {
                    // UNWRAP: if we can't acquire the lock we want to panic.
                    let skybox = self.skybox.read().unwrap();
                    render_pass.set_pipeline(&pipeline.pipeline);
                    render_pass.set_bind_group(0, Some(bindgroup.as_ref()), &[]);
                    render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
                }
            }
            self.queue().submit(std::iter::once(encoder.finish()));
        }

        // then render bloom
        if self.has_bloom.load(Ordering::Relaxed) {
            self.bloom.bloom(self.device(), self.queue());
        } else {
            // copy the input hdr texture to the bloom mix texture
            let mut encoder =
                self.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("no bloom copy"),
                    });
            let bloom_mix_texture = self.bloom.get_mix_texture();
            encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.hdr_texture.read().unwrap().texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
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
            self.queue().submit(std::iter::once(encoder.finish()));
        }

        self.tonemapping.render(self.device(), self.queue(), view);

        if self.has_debug_overlay.load(Ordering::Relaxed) {
            if let Some(indirect_draw_buffer) = maybe_indirect_buffer {
                self.debug_overlay.render(
                    self.device(),
                    self.queue(),
                    view,
                    &self.stage_slab_buffer.read().unwrap(),
                    &indirect_draw_buffer,
                );
            }
        }
    }
}

/// Manages scene heirarchy on the [`Stage`].
///
/// Clones all reference the same nested transform.
///
/// Only available on CPU.
#[derive(Clone)]
pub struct NestedTransform<Ct: IsContainer = HybridContainer> {
    global_transform: Ct::Container<Transform>,
    local_transform: Arc<RwLock<Transform>>,
    children: Arc<RwLock<Vec<NestedTransform>>>,
    parent: Arc<RwLock<Option<NestedTransform>>>,
}

impl core::fmt::Debug for NestedTransform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let children = self
            .children
            .read()
            .unwrap()
            .iter()
            .map(|nt| nt.global_transform.id())
            .collect::<Vec<_>>();
        let parent = self
            .parent
            .read()
            .unwrap()
            .as_ref()
            .map(|nt| nt.global_transform.id());
        f.debug_struct("NestedTransform")
            .field("local_transform", &self.local_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl NestedTransform<WeakContainer> {
    pub fn from_hybrid(hybrid: &NestedTransform) -> Self {
        Self {
            global_transform: WeakHybrid::from_hybrid(&hybrid.global_transform),
            local_transform: hybrid.local_transform.clone(),
            children: hybrid.children.clone(),
            parent: hybrid.parent.clone(),
        }
    }

    pub(crate) fn upgrade(&self) -> Option<NestedTransform> {
        Some(NestedTransform {
            global_transform: self.global_transform.upgrade()?,
            local_transform: self.local_transform.clone(),
            children: self.children.clone(),
            parent: self.parent.clone(),
        })
    }
}

impl NestedTransform {
    pub fn new(mngr: &SlabAllocator<impl IsRuntime>) -> Self {
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(Transform::default())),
            global_transform: mngr.new_value(Transform::default()),
            children: Default::default(),
            parent: Default::default(),
        };
        nested.mark_dirty();
        nested
    }

    /// Moves the inner `Gpu<Transform>` of the global transform to a different
    /// slab.
    ///
    /// This is used by the GLTF parser to move light's node transforms to the
    /// light slab after they are created, which keeping any geometry's node
    /// transforms untouched.
    pub(crate) fn move_gpu_to_slab(&mut self, slab: &SlabAllocator<impl IsRuntime>) {
        self.global_transform = slab.new_value(Transform::default());
        self.mark_dirty();
    }

    pub fn get_notifier_index(&self) -> usize {
        self.global_transform.notifier_index()
    }

    fn mark_dirty(&self) {
        self.global_transform.set(self.get_global_transform());
        for child in self.children.read().unwrap().iter() {
            child.mark_dirty();
        }
    }

    /// Modify the local transform.
    ///
    /// This automatically applies the local transform to the global transform.
    pub fn modify(&self, f: impl Fn(&mut Transform)) {
        {
            // UNWRAP: panic on purpose
            let mut local_guard = self.local_transform.write().unwrap();
            f(&mut local_guard);
        }
        self.mark_dirty()
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
        self.global_transform.id()
    }

    pub fn add_child(&self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        node.mark_dirty();
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform.id() == node.global_transform.id() {
                node.mark_dirty();
                let _ = node.parent.write().unwrap().take();
                false
            } else {
                true
            }
        });
    }

    pub fn parent(&self) -> Option<NestedTransform> {
        self.parent.read().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use craballoc::runtime::CpuRuntime;
    use crabslab::{Array, Id, Slab};
    use glam::{Mat4, Vec2, Vec3};

    use crate::{
        camera::Camera,
        pbr::PbrConfig,
        stage::{cpu::SlabAllocator, NestedTransform, Renderlet, Vertex},
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
        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "This is just riff-raff, as it doesn't compile without the borrow."
        )]
        let slab = SlabAllocator::<CpuRuntime>::new(&CpuRuntime, ());
        // Setup a hierarchy of transforms
        log::info!("new");
        let root = NestedTransform::new(&slab);
        let child = NestedTransform::new(&slab);
        log::info!("set");
        child.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });
        log::info!("grandchild");
        let grandchild = NestedTransform::new(&slab);
        grandchild.set(Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            ..Default::default()
        });

        log::info!("hierarchy");
        // Build the hierarchy
        root.add_child(&child);
        child.add_child(&grandchild);

        log::info!("get_global_transform");
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
        assert_eq!(1, r.ref_count());

        stage.add_renderlet(&r);
        assert_eq!(1, r.ref_count());
    }

    #[test]
    /// Tests that the PBR descriptor is written to slot 0 of the geometry buffer,
    /// and that it contains what we think it contains.
    fn stage_pbr_desc_sanity() {
        let ctx = Context::headless(100, 100);
        let stage = ctx.new_stage();
        stage.commit();

        let slab = futures_lite::future::block_on(stage.read(..)).unwrap();
        let pbr_desc = slab.read_unchecked(Id::<PbrConfig>::new(0));
        pretty_assertions::assert_eq!(stage.pbr_config.get(), pbr_desc);
    }
}

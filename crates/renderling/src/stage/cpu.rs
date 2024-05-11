//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage [`Renderlet`]s for rendering.
use core::sync::atomic::Ordering;
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
use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use snafu::Snafu;

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

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    // TODO: Add `Indirect` to `StageDrawStrategy` which uses `RenderPass::multi_draw_indirect`
    Direct(Vec<Hybrid<Renderlet>>),
}

fn create_stage_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
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
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &vertex_linkage.module,
            entry_point: vertex_linkage.entry_point,
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
            count: 1,
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_linkage.module,
            entry_point: fragment_linkage.entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba16Float,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    pipeline
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

    pub(crate) mngr: SlabAllocator,

    pub(crate) pbr_config: Hybrid<PbrConfig>,
    pub(crate) lights: HybridArray<Id<Light>>,

    pub(crate) vertex_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) fragment_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,

    pub(crate) stage_pipeline: Arc<wgpu::RenderPipeline>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,

    pub(crate) hdr_texture: Texture,
    pub(crate) depth_texture: Texture,

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
    type Target = SlabAllocator;

    fn deref(&self) -> &Self::Target {
        &self.mngr
    }
}

impl DerefMut for Stage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mngr
    }
}

impl Stage {
    /// Create a new stage.
    pub fn new(ctx: &crate::Context) -> Self {
        let (device, queue) = ctx.get_device_and_queue_owned();
        let resolution @ UVec2 { x: w, y: h } = ctx.get_size();
        let atlas = Atlas::empty(&device, &queue);
        let mut mngr = SlabAllocator::default();
        let pbr_config = mngr.new_value(PbrConfig {
            atlas_size: atlas.get_size(),
            resolution,
            ..Default::default()
        });
        let lights = mngr.new_array(vec![Id::<Light>::NONE; 16]);
        let hdr_texture = Texture::create_hdr_texture(&device, &queue, w, h);
        let depth_texture = Texture::create_depth_texture(&device, w, h);
        let bloom = Bloom::new(device.clone(), queue.clone(), resolution, &hdr_texture);
        let tonemapping = Tonemapping::new(
            &device,
            &queue,
            ctx.get_render_target().format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );

        Self {
            mngr,
            pbr_config,
            lights,

            vertex_debug: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device.clone(),
                queue.clone(),
                256,
            )))),
            fragment_debug: Arc::new(RwLock::new({
                let len = (resolution.x * resolution.y) as usize;
                let mut debug = CpuSlab::new(WgpuBuffer::new(
                    device.clone(),
                    queue.clone(),
                    RenderletFragmentLog::SLAB_SIZE * len,
                ));
                debug.append_array(&vec![RenderletFragmentLog::default(); len]);
                debug
            })),
            stage_pipeline: create_stage_render_pipeline(&device).into(),
            atlas,
            skybox: Arc::new(RwLock::new(Skybox::empty(device.clone(), queue.clone()))),
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
            background_color: Arc::new(RwLock::new(wgpu::Color::TRANSPARENT)),
            device,
            queue,
        }
    }

    pub fn get_device_and_queue_owned(&self) -> (Arc<wgpu::Device>, Arc<wgpu::Queue>) {
        (self.device.clone(), self.queue.clone())
    }

    pub fn set_background_color(&self, color: impl Into<Vec4>) {
        let Vec4 { x, y, z, w } = color.into();
        *self.background_color.write().unwrap() = wgpu::Color {
            r: x as f64,
            g: y as f64,
            b: z as f64,
            a: w as f64,
        };
    }

    pub fn with_background_color(self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        self.pbr_config.modify(|cfg| cfg.debug_mode = debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&mut self, use_lighting: bool) {
        self.pbr_config
            .modify(|cfg| cfg.has_lighting = use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the lights to use for shading.
    pub fn set_lights(&mut self, lights: impl IntoIterator<Item = Id<Light>>) {
        log::trace!("setting lights");
        self.lights = self.mngr.new_array(lights);
        self.pbr_config.modify(|cfg| {
            cfg.light_array = self.lights.array();
        });
    }

    pub fn get_size(&self) -> UVec2 {
        let w = self.hdr_texture.width();
        let h = self.hdr_texture.height();
        UVec2::new(w, h)
    }

    pub fn set_size(&self, size: UVec2) {
        if size == self.get_size() {
            return;
        }

        self.pbr_config.modify(|cfg| cfg.resolution = size);

        // Allocate space for our fragment shader logs...
        let len = size.x * size.y;
        // UNWRAP: if we can't write we want to panic.
        let mut debug = self.fragment_debug.write().unwrap();
        let array = Array::<RenderletFragmentLog>::new(0, len);
        debug.maybe_expand_to_fit::<RenderletFragmentLog>(len as usize);
        debug.write_array(
            array,
            vec![RenderletFragmentLog::default(); len as usize].as_slice(),
        );

        todo!("need to resize textures, recreate bloom, etc");
    }

    pub fn with_size(self, size: UVec2) -> Self {
        self.set_size(size);
        self
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
                atlas_index: i,
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

    /// Return the main render pipeline, creating it if necessary.
    fn get_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        self.stage_pipeline.clone()
    }

    fn get_slab_buffers_bindgroup(&self, slab_buffer: &wgpu::Buffer) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.buffers_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new({
                let device: &wgpu::Device = &self.device;
                let pipeline: &wgpu::RenderPipeline = &self.get_pipeline();
                crate::linkage::slab_bindgroup(
                    device,
                    slab_buffer,
                    self.vertex_debug.read().unwrap().as_ref().get_buffer(),
                    self.fragment_debug.read().unwrap().as_ref().get_buffer(),
                    &pipeline.get_bind_group_layout(0),
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
                &self.get_pipeline().get_bind_group_layout(1),
                // UNWRAP: if we can't acquire locks we want to panic
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
    pub fn add_renderlet(&mut self, renderlet: &Hybrid<Renderlet>) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.push(renderlet.clone());
            }
        }

        // Ensure we have space to write GPU debugging info to our vertex debug buffer
        let vertex_debug_data =
            vec![RenderletVertexLog::default(); renderlet.get().vertices_array.len()];
        // UNWRAP: if we can't read we want to panic.
        let mut debug = self.vertex_debug.write().unwrap();
        debug.append_array(&vertex_debug_data);
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

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            device: self.device.clone(),
            queue: self.queue.clone(),
            texture: self.depth_texture.clone(),
        }
    }

    /// Read the vertex debug messages.
    pub fn read_vertex_debug_logs(&self) -> Vec<RenderletVertexLog> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.get_renderlets();
        let len = draws
            .into_iter()
            .map(|r| r.get().vertices_array.len())
            .sum::<usize>();
        let array = Array::<RenderletVertexLog>::new(0, len as u32);
        let logs = self.vertex_debug.read().unwrap().as_ref().read_vec(array);
        logs
    }

    /// Clear the vertex debug messages.
    pub fn clear_vertex_debug_logs(&self) {
        let len = {
            // UNWRAP: if we can't read we want to panic.
            let draws = self.draws.read().unwrap();
            match draws.deref() {
                StageDrawStrategy::Direct(units) => {
                    let len = units
                        .iter()
                        .map(|r| r.get().vertices_array.len())
                        .sum::<usize>();
                    len
                }
            }
        };
        // UNWRAP: if we can't read we want to panic.
        let mut debug = self.vertex_debug.write().unwrap();
        let array = Array::<RenderletVertexLog>::new(0, len as u32);
        debug.write_array(array, vec![RenderletVertexLog::default(); len].as_slice());
    }

    /// Read the fragment debug messages.
    pub fn read_fragment_debug_logs(&self) -> Vec<RenderletFragmentLog> {
        let resolution = self.pbr_config.get().resolution;
        let len = resolution.x * resolution.y;
        let array = Array::<RenderletFragmentLog>::new(0, len);
        log::trace!("reading {len} logs from fragment_debug buffer");
        // UNWRAP: if we can't read we want to panic.
        self.fragment_debug.read().unwrap().read_vec(array)
    }

    /// Clear the fragment debug messages.
    pub fn clear_fragment_debug_logs(&self) {
        let resolution = self.pbr_config.get().resolution;
        let len = resolution.x * resolution.y;
        // UNWRAP: if we can't write we want to panic.
        let mut debug = self.fragment_debug.write().unwrap();
        let array = Array::<RenderletFragmentLog>::new(0, len);
        debug.write_array(
            array,
            vec![RenderletFragmentLog::default(); len as usize].as_slice(),
        );
    }

    pub fn new_skybox_from_path(
        &self,
        path: impl AsRef<std::path::Path>,
        camera: Id<Camera>,
    ) -> Result<Skybox, AtlasImageError> {
        let hdr = AtlasImage::from_hdr_path(path)?;
        Ok(Skybox::new(
            self.device.clone(),
            self.queue.clone(),
            hdr,
            camera,
        ))
    }

    pub fn new_nested_transform(&mut self) -> NestedTransform {
        NestedTransform::new(&mut self.mngr)
    }

    fn tick_internal(&mut self) -> Arc<wgpu::Buffer> {
        {
            let mut draw_guard = self.draws.write().unwrap();
            match draw_guard.deref_mut() {
                StageDrawStrategy::Direct(draws) => {
                    draws.retain(|d| d.strong_count() > 2);
                }
            }
        }
        if let Some(new_slab_buffer) = self.mngr.upkeep(
            &self.device,
            &self.queue,
            Some("stage render upkeep"),
            wgpu::BufferUsages::empty(),
        ) {
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
    pub fn tick(&mut self) {
        let _ = self.tick_internal();
    }

    pub fn render(&mut self, view: &wgpu::TextureView) {
        log::trace!("clearing pass");
        crate::conduct_clear_pass(
            &self.device,
            &self.queue,
            Some("stage clear pass"),
            vec![view, &self.hdr_texture.view],
            Some(&self.depth_texture.view),
            *self.background_color.read().unwrap(),
        );

        {
            log::trace!("rendering the stage");
            let label = Some("stage render");
            let pipeline = self.get_pipeline();
            let slab_buffer = self.tick_internal();
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
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.hdr_texture.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
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
                    texture: &self.hdr_texture.texture,
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
    global_transform: Gpu<Transform>,
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

impl UpdatesSlab for NestedTransform {
    fn u32_array(&self) -> Array<u32> {
        self.global_transform.u32_array()
    }

    fn strong_count(&self) -> usize {
        self.global_transform.strong_count()
    }

    fn type_name(&self) -> &'static str {
        self.global_transform.type_name()
    }

    fn get_update(&self) -> Vec<SlabUpdate> {
        let transform = self.get_global_transform();
        self.global_transform.set(transform);
        self.global_transform.get_update()
    }

    fn forgotten(&self) -> bool {
        self.global_transform.forgotten()
    }
}

impl NestedTransform {
    pub fn new(mngr: &mut SlabAllocator) -> Self {
        let id = mngr.allocate::<Transform>();
        let mut update_sources = mngr.update_sources.write().unwrap();
        let global_transform = Gpu {
            id,
            notifier_index: update_sources.len(),
            notify: mngr.notifier.0.clone(),
            update: Default::default(),
            forgotten: Default::default(),
        };
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(Transform::default())),
            global_transform,
            children: Default::default(),
            parent: Default::default(),
        };

        update_sources.push(Box::new(nested.clone()));

        nested
    }

    pub fn modify_local_transform(&self, f: impl Fn(&mut Transform)) {
        // UNWRAP: panic on purpose
        let mut local_guard = self.local_transform.write().unwrap();
        f(&mut local_guard);
        // UNWRAP: safe because it's unbounded
        self.global_transform
            .notify
            .try_send(self.global_transform.notifier_index)
            .unwrap();
    }

    pub fn set_local_transform(&self, transform: Transform) {
        self.modify_local_transform(move |t| {
            *t = transform;
        });
    }

    pub fn get_local_transform(&self) -> Transform {
        *self.local_transform.read().unwrap()
    }

    pub fn get_global_transform(&self) -> Transform {
        let maybe_parent_guard = self.parent.read().unwrap();
        let transform = self.get_local_transform();
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
        let global_transform = node.get_global_transform();
        node.global_transform.set(global_transform);
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&self, node: &NestedTransform) {
        self.children.write().unwrap().retain_mut(|child| {
            if child.global_transform.id() == node.global_transform.id() {
                let local_transform = node.get_local_transform();
                node.global_transform.set(local_transform);
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
    use crabslab::{Array, Slab};
    use glam::{Mat4, UVec2, Vec2, Vec3};

    use crate::{
        camera::Camera,
        stage::{Renderlet, RenderletVertexLog, Vertex},
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
    fn can_read_shader_debug_logs() {
        let ctx = Context::headless(10, 10);
        let mut stage = ctx.new_stage();
        let camera = stage.new_value(Camera::default_ortho2d(100.0, 100.0));
        let geometry = stage.new_array(crate::test::right_tri_vertices());
        let tri = stage.new_value(Renderlet {
            camera_id: camera.id(),
            vertices_array: geometry.array(),
            ..Default::default()
        });
        stage.add_renderlet(&tri);
        stage.render(&ctx.get_next_frame().unwrap().view());

        // read vertex logs
        {
            let vertex_logs = stage.read_vertex_debug_logs();
            for (i, log) in vertex_logs.iter().enumerate() {
                println!("log_{i}:{log:#?}");
                assert_eq!(tri.id(), log.renderlet_id);
                assert!(log.started);
                assert!(log.completed);
            }

            stage.clear_vertex_debug_logs();
            let blank_logs = stage.read_vertex_debug_logs();
            assert_ne!(vertex_logs, blank_logs);
            for log in blank_logs.iter() {
                assert_eq!(&RenderletVertexLog::default(), log);
            }
        }

        // read fragment logs
        {
            let fragment_logs = stage.read_fragment_debug_logs();
            let UVec2 { x: w, y: h } = ctx.get_size();
            assert_eq!((w * h) as usize, fragment_logs.len());
            for (i, log) in fragment_logs.iter().enumerate() {
                // only those fragments covered by the vertex shader's output are
                // evaluated by the fragment shader, so only check those ones...
                // @see DEVLOG.md's "Tue Apr 9 - Better debugging"
                if log.started {
                    assert!(log.completed);
                    println!("log_{i}:{log:#?}");
                }
            }
            stage.clear_fragment_debug_logs();
        }
    }

    #[test]
    fn matrix_subtraction_sanity() {
        let m = Mat4::IDENTITY - Mat4::IDENTITY;
        assert_eq!(Mat4::ZERO, m);
    }
}

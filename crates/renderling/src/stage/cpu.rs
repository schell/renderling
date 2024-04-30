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
    bloom::Bloom,
    pbr::{debug::DebugMode, light::Light, PbrConfig},
    slab::*,
    tonemapping::Tonemapping,
    Atlas, AtlasError, AtlasImage, AtlasImageError, AtlasTexture, Camera, CpuCubemap, DepthTexture,
    Device, Queue, Skybox, WgpuSlabError,
};
use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use snafu::Snafu;

use super::*;

#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("{source}"))]
    Slab { source: WgpuSlabError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

impl From<WgpuSlabError> for StageError {
    fn from(source: WgpuSlabError) -> Self {
        Self::Slab { source }
    }
}

impl From<StageError> for moongraph::GraphError {
    fn from(value: StageError) -> Self {
        moongraph::GraphError::other(value)
    }
}

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    // TODO: Add `Indirect` to `StageDrawStrategy` which uses `RenderPass::multi_draw_indirect`
    Direct(Vec<Hybrid<Renderlet>>),
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
///
/// ## Note
/// Only available on the CPU. Not available in shaders.
#[derive(Clone)]
pub struct Stage {
    pub(crate) device: Device,
    pub(crate) queue: Queue,

    pub(crate) mngr: SlabManager,

    pub pbr_config: Hybrid<PbrConfig>,
    pub lights: HybridArray<Id<Light>>,

    pub(crate) vertex_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) fragment_debug: Arc<RwLock<CpuSlab<WgpuBuffer>>>,

    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,

    pub(crate) hdr_texture: crate::Texture,
    pub(crate) depth_texture: crate::Texture,

    pub(crate) atlas: Arc<RwLock<Atlas>>,
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
    type Target = SlabManager;

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
    pub fn new(r: &crate::Context) -> Self {
        let (device, queue) = r.get_device_and_queue_owned();
        let resolution @ UVec2 { x: w, y: h } = r.get_size();
        let atlas = Atlas::empty(&device, &queue);
        let mut mngr = SlabManager::default();
        let pbr_config = mngr.new_hybrid(PbrConfig {
            atlas_size: atlas.size,
            resolution,
            ..Default::default()
        });
        let lights = mngr.new_hybrid_array(vec![Id::<Light>::NONE; 16]);
        let hdr_texture = crate::Texture::create_hdr_texture(&device, &queue, w, h);
        let depth_texture = crate::Texture::create_depth_texture(&device, w, h);
        let bloom = Bloom::new(&device, &queue, resolution, &hdr_texture);
        let tonemapping = Tonemapping::new(
            &device,
            &queue,
            r.render_target.format().add_srgb_suffix(),
            &bloom.get_mix_texture(),
        );

        Self {
            mngr,
            pbr_config,
            lights,

            vertex_debug: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device.0.clone(),
                queue.0.clone(),
                256,
            )))),
            fragment_debug: Arc::new(RwLock::new({
                let len = (resolution.x * resolution.y) as usize;
                let mut debug = CpuSlab::new(WgpuBuffer::new(
                    device.0.clone(),
                    queue.0.clone(),
                    RenderletFragmentLog::SLAB_SIZE * len,
                ));
                debug.append_array(&vec![RenderletFragmentLog::default(); len]);
                debug
            })),
            pipeline: Default::default(),
            atlas: Arc::new(RwLock::new(atlas)),
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
        self.lights = self.mngr.new_hybrid_array(lights);
        self.pbr_config.modify(|cfg| {
            cfg.light_array = self.lights.array();
        });
    }

    pub fn set_size(&self, size: UVec2) {
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
    /// This invalidates any currently staged `GpuTextures`.
    pub fn set_images(
        &mut self,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Vec<AtlasTexture>, StageError> {
        // UNWRAP: if we can't write the atlas we want to panic
        let mut atlas = self.atlas.write().unwrap();
        *atlas = Atlas::pack(&self.device, &self.queue, images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();
        // The atlas size must be reset
        self.pbr_config.modify(|cfg| cfg.atlas_size = atlas.size);

        let textures = atlas
            .frames()
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

    /// Return the skybox render pipeline, creating it if necessary.
    pub fn get_skybox_pipeline_and_bindgroup(
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
                    crate::Texture::HDR_TEXTURE_FORMAT,
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
    pub fn get_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        fn create_stage_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
            log::trace!("creating stage render pipeline");
            let label = Some("stage render pipeline");
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

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.pipeline.lock().unwrap();
        if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(create_stage_render_pipeline(&self.device));
            *pipeline = Some(p.clone());
            p
        }
    }

    pub fn get_slab_buffers_bindgroup(&self, slab_buffer: &wgpu::Buffer) -> Arc<wgpu::BindGroup> {
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

    pub fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(crate::linkage::atlas_and_skybox_bindgroup(
                &self.device,
                &self.get_pipeline().get_bind_group_layout(1),
                // UNWRAP: if we can't acquire locks we want to panic
                &self.atlas.read().unwrap(),
                &self.skybox.read().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Draw a [`Renderlet`] each frame.
    ///
    /// Returns a CPU/GPU hybrid `Renderlet`.
    ///
    /// Updates the input `renderlet` with a `debug_index`, which can be used to retrieve
    /// debugging information after running a shader.
    pub fn draw(&mut self, mut renderlet: crate::Renderlet) -> Hybrid<crate::Renderlet> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        let hybrid;
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                renderlet.debug_index = units.len() as u32;
                hybrid = self.mngr.new_hybrid(renderlet);
                units.push(hybrid.clone());
            }
        }

        // Ensure we have space to write GPU debugging info to our vertex debug buffer
        {
            let vertex_debug_data = vec![RenderletVertexLog::default(); renderlet.vertices.len()];
            // UNWRAP: if we can't read we want to panic.
            let mut debug = self.vertex_debug.write().unwrap();
            debug.append_array(&vertex_debug_data);
        }
        hybrid
    }

    /// Erase the [`Renderlet`] with the given [`Id`] from the stage.
    ///
    /// This removes the [`Renderlet`] from the stage.
    pub fn erase(&self, id: Id<Renderlet>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.retain(|hybrid| hybrid.id() != id);
            }
        }
    }

    /// Returns all the draw operations ([`Renderlet`]s) on the stage, paired with their [`Id`]s.
    pub fn get_draws(&self) -> Vec<Hybrid<Renderlet>> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.draws.read().unwrap();
        match draws.deref() {
            StageDrawStrategy::Direct(units) => units.clone(),
        }
    }

    /// Returns a clone of the current depth texture.
    pub fn get_depth_texture(&self) -> DepthTexture {
        DepthTexture {
            device: self.device.0.clone(),
            queue: self.queue.0.clone(),
            texture: self.depth_texture.clone(),
        }
    }

    // /// Configure [`Renderling`] to render this stage.
    // pub fn configure_graph(&self, r: &mut crate::Context, should_copy_frame_to_post: bool) {
    //     // set up the render graph
    //     use crate::{
    //         frame::{copy_frame_to_post, create_frame, present},
    //         hdr::{clear_surface_hdr_and_depth, create_hdr_render_surface},
    //         tonemapping::tonemapping,
    //     };

    //     let (hdr_surface,) = r.graph.visit(create_hdr_render_surface).unwrap().unwrap();
    //     r.graph.add_resource(hdr_surface);
    //     r.graph.add_resource(self.clone());

    //     // pre-render
    //     r.graph
    //         .add_subgraph(graph!(create_frame, clear_surface_hdr_and_depth))
    //         .add_barrier();

    //     // render
    //     if should_copy_frame_to_post {
    //         r.graph.add_subgraph(graph!(
    //             stage_render
    //                 < tonemapping
    //                 < copy_frame_to_post
    //                 < present
    //         ));
    //     } else {
    //         r.graph.add_subgraph(graph!(
    //             stage_render
    //                 < tonemapping
    //                 < present
    //         ));
    //     }
    // }

    /// Read the atlas image from the GPU.
    ///
    /// This is primarily used for debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_atlas_image(&self) -> image::RgbaImage {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.atlas
            .read()
            .unwrap()
            .atlas_img(&self.device, &self.queue)
    }

    /// Read the brdf image from the GPU.
    ///
    /// This is primarily used for debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_brdf_image(&self) -> image::Rgba32FImage {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.brdf_lut;
        let extent = texture.texture.size();
        let buffer = crate::Texture::read(
            &texture.texture,
            &self.device,
            &self.queue,
            extent.width as usize,
            extent.height as usize,
            2,
            2,
        );
        let pixels = buffer.pixels(&self.device);
        let pixels: Vec<f32> = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
            .iter()
            .copied()
            .map(|bits| half::f16::from_bits(bits).to_f32())
            .collect();
        let pixels: Vec<f32> = pixels
            .chunks_exact(2)
            .flat_map(|pixel| match pixel {
                [r, g] => [*r, *g, 0.0, 1.0],
                _ => unreachable!(),
            })
            .collect();
        let img: image::ImageBuffer<image::Rgba<f32>, Vec<f32>> =
            image::ImageBuffer::from_vec(extent.width, extent.height, pixels).unwrap();
        img
    }

    fn read_cubemap_mip0(&self, texture: &crate::Texture) -> CpuCubemap {
        let placeholder_image =
            image::Rgba32FImage::from_pixel(1, 1, image::Rgba([0.0, 0.0, 0.0, 0.0]));
        let mut out: [image::Rgba32FImage; 6] = [
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
            placeholder_image.clone(),
        ];
        let extent = texture.texture.size();
        let width = extent.width;
        let height = extent.height;
        for i in 0..6 {
            let copied_buffer = crate::Texture::read_from(
                &texture.texture,
                &self.device,
                &self.queue,
                width as usize,
                height as usize,
                4,
                2,
                0,
                Some(wgpu::Origin3d { x: 0, y: 0, z: i }),
            );
            let pixels = copied_buffer.pixels(&self.device);
            let pixels = bytemuck::cast_slice::<u8, u16>(pixels.as_slice())
                .iter()
                .map(|p| half::f16::from_bits(*p).to_f32())
                .collect::<Vec<_>>();
            let img: image::Rgba32FImage =
                image::ImageBuffer::from_vec(width, height, pixels).unwrap();
            out[i as usize] = img;
        }
        CpuCubemap {
            images: out.map(|img| img.into()),
        }
    }

    /// Read the irradiance cubemap images from the GPU.
    ///
    /// This only reads the top level of mips. This is primarily used for
    /// debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_irradiance_cubemap(&self) -> CpuCubemap {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.irradiance_cubemap;
        self.read_cubemap_mip0(texture)
    }

    /// Read the prefiltered cubemap images from the GPU.
    ///
    /// This only reads the top level of mips. This is primarily used for
    /// debugging.
    ///
    /// ## Panics
    /// Panics if the pixels read from the GPU cannot be converted into an
    /// `RgbaImage`.
    pub fn read_prefiltered_cubemap(&self) -> CpuCubemap {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let skybox = self.skybox.read().unwrap();
        let texture = &skybox.prefiltered_environment_cubemap;
        self.read_cubemap_mip0(texture)
    }

    /// Read all the data from the stage.
    ///
    /// This blocks until the GPU buffer is mappable, and then copies the data
    /// into a vector.
    ///
    /// This is primarily used for debugging.
    pub async fn read_slab(&self) -> Result<Vec<u32>, SlabManagerError> {
        self.mngr
            .read(&self.device, &self.queue, Some("stage slab read"), ..)
            .await
    }

    /// Read the vertex debug messages.
    pub fn read_vertex_debug_logs(&self) -> Vec<RenderletVertexLog> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.get_draws();
        let len = draws
            .into_iter()
            .map(|r| r.get().vertices.len())
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
                    let len = units.iter().map(|r| r.get().vertices.len()).sum::<usize>();
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

    pub fn render(&mut self, view: &wgpu::TextureView) {
        log::trace!("clearing pass");
        crate::frame::conduct_clear_pass(
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
            let slab_buffer = if let Some(new_slab_buffer) = self.mngr.upkeep(
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
                // UNWRAP: safe because we called `SlabManager::upkeep` above^, which ensures the buffer
                // exists
                self.mngr.get_buffer().unwrap()
            };
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
                                let vertex_range = if rlet.indices.is_null() {
                                    0..rlet.vertices.len() as u32
                                } else {
                                    0..rlet.indices.len() as u32
                                };
                                let id = hybrid.id();
                                let instance_range = id.inner()..id.inner() + 1;
                                log::trace!(
                            "drawing vertices {vertex_range:?} and instances {instance_range:?}"
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
    global_transform: Hybrid<Transform>,
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
            .field("global_transform", &self.global_transform)
            .field("local_transform", &self.local_transform)
            .field("children", &children)
            .field("parent", &parent)
            .finish()
    }
}

impl NestedTransform {
    pub fn new(mngr: &mut SlabManager) -> Self {
        let global_transform = mngr.new_hybrid(Transform::default());
        let nested = NestedTransform {
            local_transform: Arc::new(RwLock::new(Transform::default())),
            global_transform,
            children: Default::default(),
            parent: Default::default(),
        };
        nested
    }

    pub fn set_local_transform(&mut self, transform: Transform) {
        *self.local_transform.write().unwrap() = transform;
        let global_transform = self.get_global_transform();
        self.global_transform.set(global_transform);
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

    pub fn add_child(&mut self, node: &NestedTransform) {
        *node.parent.write().unwrap() = Some(self.clone());
        let global_transform = node.get_global_transform();
        node.global_transform.set(global_transform);
        self.children.write().unwrap().push(node.clone());
    }

    pub fn remove_child(&mut self, node: &NestedTransform) {
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
    use crabslab::{Array, GrowableSlab, Slab};
    use glam::{UVec2, Vec2, Vec3};

    use crate::{Camera, Context, Renderlet, RenderletVertexLog, Vertex};

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
        let (projection, view) = crate::default_ortho2d(100.0, 100.0);
        let camera = stage.append(&Camera::new(projection, view));
        let geometry = stage.append_array(&crate::test::right_tri_vertices());
        let tri = stage.draw(Renderlet {
            camera,
            vertices: geometry,
            ..Default::default()
        });
        stage.render(&ctx.get_current_frame().unwrap().view());

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
}

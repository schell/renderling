//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage objects for rendering.
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crate::{
    pbr::{debug::DebugMode, light::Light, PbrConfig},
    Atlas, AtlasError, AtlasImage, AtlasImageError, AtlasTexture, Camera, CpuCubemap, DepthTexture,
    Device, HdrSurface, Queue, Skybox, WgpuSlabError,
};
use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use moongraph::{NoDefault, View, ViewMut};
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
    Direct(Vec<DrawUnit>),
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
///
/// ## Note
/// Only available on the CPU. Not available in shaders.
#[derive(Clone)]
pub struct Stage {
    pub(crate) slab: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) atlas: Arc<RwLock<Atlas>>,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
    pub(crate) skybox_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) draws: Arc<RwLock<StageDrawStrategy>>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Slab for Stage {
    fn len(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().len()
    }

    fn read<T: SlabItem + Default>(&self, id: Id<T>) -> T {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().read(id)
    }

    fn write_indexed<T: SlabItem>(&mut self, t: &T, index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().write_indexed_slice(t, index)
    }
}

impl GrowableSlab for Stage {
    fn capacity(&self) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().capacity()
    }

    fn reserve_capacity(&mut self, capacity: usize) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().reserve_capacity(capacity)
    }

    fn increment_len(&mut self, n: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.write().unwrap().increment_len(n)
    }
}

impl Stage {
    /// Create a new stage.
    pub fn new(device: Device, queue: Queue) -> Self {
        let atlas = Atlas::empty(&device, &queue);
        let pbr_config = PbrConfig {
            atlas_size: atlas.size,
            ..Default::default()
        };
        let mut s = Self {
            slab: Arc::new(RwLock::new(CpuSlab::new(WgpuBuffer::new(
                device.0.clone(),
                queue.0.clone(),
                256,
            )))),
            pipeline: Default::default(),
            atlas: Arc::new(RwLock::new(atlas)),
            skybox: Arc::new(RwLock::new(Skybox::empty(device.clone(), queue.clone()))),
            skybox_bindgroup: Default::default(),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            has_bloom: Arc::new(AtomicBool::new(false)),
            buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            draws: Arc::new(RwLock::new(StageDrawStrategy::Direct(vec![]))),
            device,
            queue,
        };
        s.append(&pbr_config);
        s
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        let id: Id<DebugMode> = Id::from(PbrConfig::offset_of_debug_mode());
        self.write(id, &debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&mut self, use_lighting: bool) {
        let id = Id::<bool>::from(PbrConfig::offset_of_has_lighting());
        self.write(id, &use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the lights to use for shading.
    pub fn set_lights(&mut self, lights: Array<Light>) {
        let id = Id::<Array<Light>>::from(PbrConfig::offset_of_light_array());
        self.write(id, &lights);
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
        let size_id = Id::<glam::UVec2>::from(PbrConfig::offset_of_atlas_size());
        // UNWRAP: if we can't write to the stage legend we want to panic
        self.slab.write().unwrap().write(size_id, &atlas.size);

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
    ) -> (Arc<wgpu::RenderPipeline>, Arc<wgpu::BindGroup>) {
        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.skybox_pipeline.write().unwrap();
        let pipeline = if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(
                crate::skybox::create_skybox_render_pipeline(
                    &self.device,
                    crate::hdr::HdrSurface::TEXTURE_FORMAT,
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
            let slab = self.slab.read().unwrap();
            let bg = Arc::new(crate::skybox::create_skybox_bindgroup(
                &self.device,
                slab.as_ref().get_buffer(),
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
            let vertex_linkage = crate::linkage::stage_vertex::linkage(device);
            let fragment_linkage = crate::linkage::stage_fragment::linkage(device);
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

    pub fn get_slab_buffers_bindgroup(&self) -> Arc<wgpu::BindGroup> {
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
                    self.slab.read().unwrap().as_ref().get_buffer(),
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

    #[cfg(feature = "gltf")]
    /// Draw a GLTF rendering each frame.
    ///
    /// Returns the id of the stored `GltfRendering`.
    pub fn draw_gltf_rendering(
        &mut self,
        unit: &crate::gltf::GltfRendering,
    ) -> Id<crate::gltf::GltfRendering> {
        let id = self.append(unit);
        let draw = DrawUnit {
            id,
            vertex_count: unit.vertex_count,
            visible: true,
        };
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.push(draw);
            }
        }
        id
    }

    /// Erase the [`RenderUnit`] with the given `Id` from the stage.
    pub fn erase_unit(&self, id: Id<GltfRendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                units.retain(|unit| unit.id != id);
            }
        }
    }

    /// Returns all the draw operations on the stage.
    pub fn get_draws(&self) -> Vec<DrawUnit> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let draws = self.draws.read().unwrap();
        match draws.deref() {
            StageDrawStrategy::Direct(units) => units.clone(),
        }
    }

    /// Show the [`RenderUnit`] with the given `Id` for rendering.
    pub fn show_unit(&self, id: Id<GltfRendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for unit in units.iter_mut() {
                    if unit.id == id {
                        unit.visible = true;
                    }
                }
            }
        }
    }

    /// Hide the [`RenderUnit`] with the given `Id` from rendering.
    pub fn hide_unit(&self, id: Id<GltfRendering>) {
        let mut draws = self.draws.write().unwrap();
        match draws.deref_mut() {
            StageDrawStrategy::Direct(units) => {
                for unit in units.iter_mut() {
                    if unit.id == id {
                        unit.visible = false;
                    }
                }
            }
        }
    }

    /// Configure [`Renderling`] to render this stage.
    pub fn configure_graph(&self, r: &mut crate::Renderling, should_copy_frame_to_post: bool) {
        // set up the render graph
        use crate::{
            frame::{copy_frame_to_post, create_frame, present},
            graph::{graph, Graph},
            hdr::{clear_surface_hdr_and_depth, create_hdr_render_surface},
            tonemapping::tonemapping,
        };

        let (hdr_surface,) = r.graph.visit(create_hdr_render_surface).unwrap().unwrap();
        r.graph.add_resource(hdr_surface);
        r.graph.add_resource(self.clone());

        // pre-render
        r.graph
            .add_subgraph(graph!(create_frame, clear_surface_hdr_and_depth))
            .add_barrier();

        // render
        if should_copy_frame_to_post {
            r.graph.add_subgraph(graph!(
                stage_render
                    < tonemapping
                    < copy_frame_to_post
                    < present
            ));
        } else {
            r.graph.add_subgraph(graph!(
                stage_render
                    < tonemapping
                    < present
            ));
        }
    }

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
    pub fn read_slab(&self) -> Result<Vec<u32>, WgpuSlabError> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().as_ref().block_on_read_raw(..)
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
}

/// Render the stage.
pub fn stage_render(
    (stage, hdr_frame, depth): (
        ViewMut<Stage, NoDefault>,
        View<HdrSurface, NoDefault>,
        View<DepthTexture, NoDefault>,
    ),
) -> Result<(), StageError> {
    let label = Some("stage render");
    let pipeline = stage.get_pipeline();
    let slab_buffers_bindgroup = stage.get_slab_buffers_bindgroup();
    let textures_bindgroup = stage.get_textures_bindgroup();
    let has_skybox = stage.has_skybox.load(std::sync::atomic::Ordering::Relaxed);
    let may_skybox_pipeline_and_bindgroup = if has_skybox {
        Some(stage.get_skybox_pipeline_and_bindgroup())
    } else {
        None
    };
    //let mut may_bloom_filter = if
    // stage.has_bloom.load(std::sync::atomic::Ordering::Relaxed) {    // UNWRAP:
    // if we can't acquire the lock we want to panic.    Some(stage.bloom.
    // write().unwrap())
    //} else {
    //    None
    //};
    // UNWRAP: if we can't read we want to panic.
    let draws = stage.draws.read().unwrap();

    let mut encoder = stage
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label,
            color_attachments: &hdr_frame.color_attachments(),
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth.view,
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
                for unit in units {
                    if unit.visible {
                        render_pass
                            .draw(0..unit.vertex_count, unit.id.inner()..unit.id.inner() + 1);
                    }
                }
            } /* render_pass.multi_draw_indirect(&indirect_buffer, 0,
               * stage.number_of_indirect_draws()); */
        }

        if let Some((pipeline, bindgroup)) = may_skybox_pipeline_and_bindgroup.as_ref() {
            log::trace!("rendering skybox");
            // UNWRAP: if we can't acquire the lock we want to panic.
            let skybox = stage.skybox.read().unwrap();
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bindgroup, &[]);
            render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
        }
    }
    stage.queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

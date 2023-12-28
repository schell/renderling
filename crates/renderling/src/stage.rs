//! GPU staging area.
//!
//! The `Stage` object contains a slab buffer and a render pipeline.
//! It is used to stage objects for rendering.
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use crabslab::{Array, CpuSlab, GrowableSlab, Id, Slab, SlabItem, WgpuBuffer};
use moongraph::{View, ViewMut};
use renderling_shader::{
    debug::DebugMode,
    stage::{light::Light, RenderUnit, StageLegend},
    texture::GpuTexture,
};
use snafu::Snafu;

use crate::{
    Atlas, AtlasError, AtlasImage, DepthTexture, Device, HdrSurface, Queue, Skybox, SlabError,
};

#[cfg(feature = "gltf")]
mod gltf_support;

#[cfg(feature = "gltf")]
pub use gltf_support::*;

#[derive(Debug, Snafu)]
pub enum StageError {
    #[snafu(display("{source}"))]
    Atlas { source: AtlasError },

    #[snafu(display("{source}"))]
    Slab { source: SlabError },
}

impl From<AtlasError> for StageError {
    fn from(source: AtlasError) -> Self {
        Self::Atlas { source }
    }
}

impl From<SlabError> for StageError {
    fn from(source: SlabError) -> Self {
        Self::Slab { source }
    }
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
#[derive(Clone)]
pub struct Stage {
    pub(crate) slab: Arc<RwLock<CpuSlab<WgpuBuffer>>>,
    pub(crate) atlas: Arc<RwLock<Atlas>>,
    pub(crate) skybox: Arc<RwLock<Skybox>>,
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
        self.slab.read().unwrap().write_indexed(t, index)
    }

    fn write_indexed_slice<T: SlabItem>(&mut self, t: &[T], index: usize) -> usize {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab.read().unwrap().write_indexed_slice(t, index)
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
        let legend = StageLegend {
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
            skybox: Arc::new(RwLock::new(Skybox::empty(&device, &queue))),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            has_bloom: Arc::new(AtomicBool::new(false)),
            buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            draws: Arc::new(RwLock::new(StageDrawStrategy::Direct(vec![]))),
            device,
            queue,
        };
        s.append(&legend);
        s
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugMode) {
        let id = Id::<DebugMode>::from(StageLegend::offset_of_debug_mode());
        self.write(id, &debug_mode);
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&self, use_lighting: bool) {
        let id = Id::<bool>::from(StageLegend::offset_of_has_lighting());
        self.write(id, &use_lighting);
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the lights to use for shading.
    pub fn set_lights(&self, lights: Array<Light>) {
        let id = Id::<Array<Light>>::from(StageLegend::offset_of_light_array());
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
        &self,
        images: impl IntoIterator<Item = AtlasImage>,
    ) -> Result<Vec<GpuTexture>, StageError> {
        // UNWRAP: if we can't write the atlas we want to panic
        let mut atlas = self.atlas.write().unwrap();
        *atlas = Atlas::pack(&self.device, &self.queue, images)?;

        // The textures bindgroup will have to be remade
        let _ = self.textures_bindgroup.lock().unwrap().take();
        // The atlas size must be reset
        let size_id = Id::<glam::UVec2>::from(StageLegend::offset_of_atlas_size());
        self.write(size_id, &atlas.size);

        let textures = atlas
            .frames()
            .map(|(i, (offset_px, size_px))| GpuTexture {
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

    fn buffers_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let visibility =
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE;
        let stage_slab = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let entries = vec![stage_slab];
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stage slab buffers"),
            entries: &entries,
        })
    }

    fn textures_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        fn image2d_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        fn cubemap_entry(binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
            let img = wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: false,
                },
                count: None,
            };
            let sampler = wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            };
            (img, sampler)
        }

        let (atlas, atlas_sampler) = image2d_entry(0);
        let (irradiance, irradiance_sampler) = cubemap_entry(2);
        let (prefilter, prefilter_sampler) = cubemap_entry(4);
        let (brdf, brdf_sampler) = image2d_entry(6);
        let (environment, environment_sampler) = cubemap_entry(8);
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("stage textures"),
            entries: &[
                atlas,
                atlas_sampler,
                irradiance,
                irradiance_sampler,
                prefilter,
                prefilter_sampler,
                brdf,
                brdf_sampler,
                environment,
                environment_sampler,
            ],
        })
    }

    /// Return the skybox render pipeline, creating it if necessary.
    pub fn get_skybox_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        fn create_skybox_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
            log::trace!("creating stage's skybox render pipeline");
            let vertex_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/skybox-slabbed_vertex.spv"));
            let fragment_shader = device.create_shader_module(wgpu::include_spirv!(
                "linkage/skybox-stage_skybox_cubemap.spv"
            ));
            let stage_slab_buffers_layout = Stage::buffers_bindgroup_layout(&device);
            let textures_layout = Stage::textures_bindgroup_layout(&device);
            let label = Some("stage skybox");
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&stage_slab_buffers_layout, &textures_layout],
                push_constant_ranges: &[],
            });

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("skybox pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "skybox::slabbed_vertex",
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
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                    count: 1,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader,
                    // TODO: remove renderling_shader::skybox::fragment_cubemap after porting
                    // to GLTF
                    entry_point: "skybox::stage_skybox_cubemap",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: crate::hdr::HdrSurface::TEXTURE_FORMAT,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(wgpu::ColorTargetState {
                            format: crate::hdr::HdrSurface::TEXTURE_FORMAT,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                multiview: None,
            })
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut pipeline = self.skybox_pipeline.write().unwrap();
        if let Some(pipeline) = pipeline.as_ref() {
            pipeline.clone()
        } else {
            let p = Arc::new(create_skybox_render_pipeline(&self.device));
            *pipeline = Some(p.clone());
            p
        }
    }

    /// Return the main render pipeline, creating it if necessary.
    pub fn get_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        fn create_stage_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
            log::trace!("creating stage render pipeline");
            let label = Some("stage render pipeline");
            let vertex_shader =
                device.create_shader_module(wgpu::include_spirv!("linkage/stage-gltf_vertex.spv"));
            let fragment_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/stage-gltf_fragment.spv"));
            let stage_slab_buffers_layout = Stage::buffers_bindgroup_layout(device);
            let textures_layout = Stage::textures_bindgroup_layout(device);
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label,
                bind_group_layouts: &[&stage_slab_buffers_layout, &textures_layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: "stage::gltf_vertex",
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
                    module: &fragment_shader,
                    entry_point: "stage::gltf_fragment",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba16Float,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
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
                let slab_buffer = self.slab.as_ref();
                let label = Some("stage slab buffer");
                let stage_slab_buffers_bindgroup =
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label,
                        layout: &pipeline.get_bind_group_layout(0),
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self
                                .slab
                                .read()
                                .unwrap()
                                .as_ref()
                                .get_buffer()
                                .as_entire_binding(),
                        }],
                    });
                stage_slab_buffers_bindgroup
            });
            *bindgroup = Some(b.clone());
            b
        }
    }

    pub fn get_textures_bindgroup(&self) -> Arc<wgpu::BindGroup> {
        fn create_textures_bindgroup(
            device: &wgpu::Device,
            pipeline: &wgpu::RenderPipeline,
            atlas: &Atlas,
            skybox: &Skybox,
        ) -> wgpu::BindGroup {
            let label = Some("stage textures");
            let textures_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label,
                layout: &pipeline.get_bind_group_layout(1),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas.texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&atlas.texture.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.irradiance_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.irradiance_cubemap.sampler,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.prefiltered_environment_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.prefiltered_environment_cubemap.sampler,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: wgpu::BindingResource::TextureView(&skybox.brdf_lut.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 7,
                        resource: wgpu::BindingResource::Sampler(&skybox.brdf_lut.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 8,
                        resource: wgpu::BindingResource::TextureView(
                            &skybox.environment_cubemap.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 9,
                        resource: wgpu::BindingResource::Sampler(
                            &skybox.environment_cubemap.sampler,
                        ),
                    },
                ],
            });
            textures_bindgroup
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.textures_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(create_textures_bindgroup(
                &self.device,
                &self.get_pipeline(),
                // UNWRAP: if we can't acquire locks we want to panic
                &self.atlas.read().unwrap(),
                &self.skybox.read().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Draw the [`RenderUnit`] each frame, and immediately return its `Id`.
    pub fn draw_unit(&self, unit: &RenderUnit) -> Id<RenderUnit> {
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
    pub fn erase_unit(&self, id: Id<RenderUnit>) {
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
    pub fn show_unit(&self, id: Id<RenderUnit>) {
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
    pub fn hide_unit(&self, id: Id<RenderUnit>) {
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
            tonemapping,
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

    /// Read all the data from the stage.
    ///
    /// This blocks until the GPU buffer is mappable, and then copies the data
    /// into a vector.
    ///
    /// This is primarily used for debugging.
    pub fn read_slab(&self) -> Result<Vec<u32>, SlabError> {
        // UNWRAP: if we can't acquire the lock we want to panic.
        self.slab
            .read()
            .unwrap()
            .as_ref()
            .block_on_read_raw(0, self.len())
    }
}

/// A unit of work to be drawn.
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawUnit {
    pub id: Id<RenderUnit>,
    pub vertex_count: u32,
    pub visible: bool,
}

/// Provides a way to communicate with the stage about how you'd like your
/// objects drawn.
pub(crate) enum StageDrawStrategy {
    Direct(Vec<DrawUnit>),
}

/// Render the stage.
pub fn stage_render(
    (stage, hdr_frame, depth): (ViewMut<Stage>, View<HdrSurface>, View<DepthTexture>),
) -> Result<(), SlabError> {
    let label = Some("stage render");
    let pipeline = stage.get_pipeline();
    let slab_buffers_bindgroup = stage.get_slab_buffers_bindgroup();
    let textures_bindgroup = stage.get_textures_bindgroup();
    let may_skybox_pipeline = if stage.has_skybox.load(std::sync::atomic::Ordering::Relaxed) {
        Some(stage.get_skybox_pipeline())
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
                    store: true,
                }),
                stencil_ops: None,
            }),
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

        if let Some(pipeline) = may_skybox_pipeline.as_ref() {
            log::trace!("rendering skybox");
            // UNWRAP: if we can't acquire the lock we want to panic.
            let skybox = stage.skybox.read().unwrap();
            render_pass.set_pipeline(pipeline);
            render_pass.draw(0..36, skybox.camera.inner()..skybox.camera.inner() + 1);
        }
    }
    stage.queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

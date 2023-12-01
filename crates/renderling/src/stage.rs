//! Rendering objects in the scene graph.
//!
//! Provides a `Stage` object that can be used to render a scene graph.
use std::{
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc, Mutex, RwLock},
};

use moongraph::{View, ViewMut};
use renderling_shader::{
    array::Array,
    debug::DebugMode,
    id::Id,
    slab::Slabbed,
    stage::{GpuLight, RenderUnit, StageLegend},
};

use crate::{
    bloom::{BloomFilter, BloomResult},
    Atlas, DepthTexture, Device, HdrSurface, Queue, Skybox, SlabBuffer, SlabError,
};

#[cfg(feature = "gltf")]
mod gltf_support;
pub mod light;

#[cfg(feature = "gltf")]
pub use gltf_support::*;

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
#[derive(Clone)]
pub struct Stage {
    pub(crate) slab: SlabBuffer,
    pub(crate) atlas: Arc<RwLock<Atlas>>,
    pub(crate) skybox: Arc<Mutex<Skybox>>,
    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) skybox_pipeline: Arc<RwLock<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) has_skybox: Arc<AtomicBool>,
    pub(crate) bloom: Arc<RwLock<BloomFilter>>,
    pub(crate) has_bloom: Arc<AtomicBool>,
    pub(crate) buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) draws: Arc<RwLock<StageDrawStrategy>>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Stage {
    /// Create a new stage.
    pub fn new(device: Device, queue: Queue) -> Self {
        let s = Self {
            slab: SlabBuffer::new(&device, 256),
            pipeline: Default::default(),
            atlas: Arc::new(RwLock::new(Atlas::empty(&device, &queue))),
            skybox: Arc::new(Mutex::new(Skybox::empty(&device, &queue))),
            skybox_pipeline: Default::default(),
            has_skybox: Arc::new(AtomicBool::new(false)),
            bloom: Arc::new(RwLock::new(BloomFilter::new(&device, &queue, 1, 1))),
            has_bloom: Arc::new(AtomicBool::new(false)),
            buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            draws: Arc::new(RwLock::new(StageDrawStrategy::Direct(vec![]))),
            device,
            queue,
        };
        let _ = s.append(&StageLegend::default());
        s
    }

    /// Allocate some storage for a type on the slab, but don't write it.
    pub fn allocate<T: Slabbed>(&self) -> Id<T> {
        self.slab.allocate(&self.device, &self.queue)
    }

    /// Allocate contiguous storage for `len` elements of a type on the slab, but don't write them.
    pub fn allocate_array<T: Slabbed>(&self, len: usize) -> Array<T> {
        self.slab.allocate_array(&self.device, &self.queue, len)
    }

    /// Write an object to the slab.
    pub fn write<T: Slabbed + Default>(&self, id: Id<T>, object: &T) -> Result<(), SlabError> {
        let () = self.slab.write(&self.device, &self.queue, id, object)?;
        Ok(())
    }

    /// Write many objects to the slab.
    pub fn write_array<T: Slabbed + Default>(
        &self,
        array: Array<T>,
        objects: &[T],
    ) -> Result<(), SlabError> {
        let () = self
            .slab
            .write_array(&self.device, &self.queue, array, objects)?;
        Ok(())
    }

    /// Add an object to the slab and return its ID.
    pub fn append<T: Slabbed + Default>(&self, object: &T) -> Id<T> {
        self.slab.append(&self.device, &self.queue, object)
    }

    /// Add a slice of objects to the slab and return an [`Array`].
    pub fn append_array<T: Slabbed + Default>(&self, objects: &[T]) -> Array<T> {
        self.slab.append_array(&self.device, &self.queue, objects)
    }

    /// Set the debug mode.
    pub fn set_debug_mode(&self, debug_mode: DebugMode) {
        let id = Id::<DebugMode>::from(StageLegend::offset_of_debug_mode());
        // UNWRAP: safe because the debug mode offset is guaranteed to be valid.
        self.slab
            .write(&self.device, &self.queue, id, &debug_mode)
            .unwrap();
    }

    /// Set the debug mode.
    pub fn with_debug_mode(self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&self, use_lighting: bool) {
        let id = Id::<bool>::from(StageLegend::offset_of_has_lighting());
        // UNWRAP: safe because the has lighting offset is guaranteed to be valid.
        self.slab
            .write(&self.device, &self.queue, id, &use_lighting)
            .unwrap();
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Set the skybox.
    pub fn set_skybox(&self, skybox: Skybox) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        let mut guard = self.skybox.lock().unwrap();
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

    /// Create a new spot light and return its builder.
    pub fn new_spot_light(&self) -> light::GpuSpotLightBuilder {
        light::GpuSpotLightBuilder::new(self)
    }

    /// Create a new directional light and return its builder.
    pub fn new_directional_light(&self) -> light::GpuDirectionalLightBuilder {
        light::GpuDirectionalLightBuilder::new(self)
    }

    /// Create a new point light and return its builder.
    pub fn new_point_light(&self) -> light::GpuPointLightBuilder {
        light::GpuPointLightBuilder::new(self)
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn set_light_array(
        &self,
        lights: impl IntoIterator<Item = Id<GpuLight>>,
    ) -> Array<Id<GpuLight>> {
        let lights = lights.into_iter().collect::<Vec<_>>();
        let light_array = self.append_array(&lights);
        let id = Id::<Array<Id<GpuLight>>>::from(StageLegend::offset_of_light_array());
        // UNWRAP: safe because we just appended the array, and the light array offset is
        // guaranteed to be valid.
        self.slab
            .write(&self.device, &self.queue, id, &light_array)
            .unwrap();
        light_array
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn with_light_array(self, lights: impl IntoIterator<Item = Id<GpuLight>>) -> Self {
        self.set_light_array(lights);
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
                    entry_point: "skybox::vertex",
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
                    entry_point: "skybox::fragment_cubemap",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: crate::hdr::HdrSurface::TEXTURE_FORMAT,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
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
            let vertex_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/stage-new_stage_vertex.spv"));
            let fragment_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/stage-stage_fragment.spv"));
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
                    entry_point: "stage::new_stage_vertex",
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
                    entry_point: "stage::stage_fragment",
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
        fn create_slab_buffers_bindgroup(
            device: &wgpu::Device,
            pipeline: &wgpu::RenderPipeline,
            stage_slab: &SlabBuffer,
        ) -> wgpu::BindGroup {
            let label = Some("stage slab buffer");
            let stage_slab_buffers_bindgroup =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label,
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: stage_slab.get_buffer().as_entire_binding(),
                    }],
                });
            stage_slab_buffers_bindgroup
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.buffers_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(create_slab_buffers_bindgroup(
                &self.device,
                &self.get_pipeline(),
                &self.slab,
            ));
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
                &self.skybox.lock().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    /// Append the [`RenderUnit`] and return a [`DrawUnit`] that can be used
    /// to draw it.
    pub fn draw_unit(&self, unit: &RenderUnit) -> Id<RenderUnit> {
        let id = self.slab.append(&self.device, &self.queue, unit);
        let draw = DrawUnit {
            id,
            vertex_count: unit.vertices.len() as u32,
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

    /// Configure [`Renderling`] to render this stage.
    pub fn configure_graph(&self, r: &mut crate::Renderling, should_copy_frame_to_post: bool) {
        // set up the render graph
        use crate::{
            frame::{copy_frame_to_post, create_frame, present},
            graph::{graph, Graph},
            hdr::{clear_surface_hdr_and_depth, create_hdr_render_surface, hdr_surface_update},
            scene::tonemapping,
        };

        let (hdr_surface,) = r.graph.visit(create_hdr_render_surface).unwrap().unwrap();
        r.graph.add_resource(hdr_surface);
        r.graph.add_resource(self.clone());

        // pre-render
        r.graph
            .add_subgraph(graph!(
                create_frame,
                clear_surface_hdr_and_depth,
                hdr_surface_update
            ))
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
}

/// A unit of work to be drawn.
pub(crate) struct DrawUnit {
    pub id: Id<RenderUnit>,
    pub vertex_count: u32,
}

/// Provides a way to communicate with the stage about how you'd like your objects drawn.
pub(crate) enum StageDrawStrategy {
    Direct(Vec<DrawUnit>),
}

/// Render the stage.
pub fn stage_render(
    (stage, hdr_frame, depth): (ViewMut<Stage>, View<HdrSurface>, View<DepthTexture>),
) -> Result<(BloomResult,), SlabError> {
    let label = Some("stage render");
    let pipeline = stage.get_pipeline();
    let slab_buffers_bindgroup = stage.get_slab_buffers_bindgroup();
    let textures_bindgroup = stage.get_textures_bindgroup();
    let may_skybox_pipeline = if stage.has_skybox.load(std::sync::atomic::Ordering::Relaxed) {
        Some(stage.get_skybox_pipeline())
    } else {
        None
    };
    let mut may_bloom_filter = if stage.has_bloom.load(std::sync::atomic::Ordering::Relaxed) {
        // UNWRAP: if we can't acquire the lock we want to panic.
        Some(stage.bloom.write().unwrap())
    } else {
        None
    };
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
                    render_pass.draw(0..unit.vertex_count, unit.id.inner()..unit.id.inner() + 1);
                }
            } //render_pass.multi_draw_indirect(&indirect_buffer, 0, stage.number_of_indirect_draws());
        }

        if let Some(pipeline) = may_skybox_pipeline.as_ref() {
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &textures_bindgroup, &[]);
            render_pass.draw(0..36, 0..1);
        }
    }
    stage.queue.submit(std::iter::once(encoder.finish()));

    let bloom_result = BloomResult(
        may_bloom_filter
            .as_mut()
            .map(|bloom| bloom.run(&stage.device, &stage.queue, &hdr_frame)),
    );
    Ok((bloom_result,))
}

#[cfg(test)]
mod test {
    use glam::Vec3;

    use crate::{
        default_ortho2d,
        shader::stage::{Camera, RenderUnit, Vertex},
        slab::Slab,
        Renderling,
    };

    use super::*;

    fn right_tri_vertices() -> Vec<Vertex> {
        vec![
            Vertex::default()
                .with_position([0.0, 0.0, 0.5])
                .with_color([0.0, 1.0, 1.0, 1.0]),
            Vertex::default()
                .with_position([0.0, 100.0, 0.5])
                .with_color([1.0, 1.0, 0.0, 1.0]),
            Vertex::default()
                .with_position([100.0, 0.0, 0.5])
                .with_color([1.0, 0.0, 1.0, 1.0]),
        ]
    }

    #[test]
    fn stage_new() {
        let mut r = Renderling::headless(100, 100).with_background_color(glam::Vec4::splat(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let stage = Stage::new(device.clone(), queue.clone())
            .with_lighting(true)
            .with_bloom(true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = Camera {
            projection,
            view,
            position: Vec3::ZERO,
        };
        let camera_id = stage.append(&camera);
        let vertices = stage.append_array(&right_tri_vertices());
        println!("vertices: {vertices:?}");
        let _ = stage.draw_unit(&RenderUnit {
            camera: camera_id,
            vertices,
            ..Default::default()
        });
        let stage_slab = futures_lite::future::block_on(stage.slab.read_raw(
            &stage.device,
            &stage.queue,
            0,
            stage.slab.len(),
        ))
        .unwrap();
        assert_eq!(camera, stage_slab.read(camera_id));
        assert_eq!(right_tri_vertices(), stage_slab.read_vec(vertices));

        stage.configure_graph(&mut r, true);

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("stage/stage_cmyk_tri.png", img);
    }
}

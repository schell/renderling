//! Rendering objects in the scene graph.
//!
//! Provides a `Stage` object that can be used to render a scene graph.
use std::sync::{Arc, Mutex};

use renderling_shader::{
    array::Array,
    debug::DebugMode,
    id::Id,
    slab::Slabbed,
    stage::{DrawIndirect, GpuLight, RenderUnit, StageLegend},
};
use snafu::Snafu;

use crate::{Atlas, Device, Queue, Skybox, SlabBuffer};

pub mod light;

#[derive(Debug, Snafu)]
pub enum StageError<T: std::fmt::Debug> {
    #[snafu(display("Out of capacity. Tried to write {:?} but capacity is {capacity}"))]
    Capacity { id: Id<T>, capacity: usize },

    #[snafu(display("Async recv error: {source}"))]
    AsyncRecv { source: async_channel::RecvError },

    #[snafu(display("Async error: {source}"))]
    Async { source: wgpu::BufferAsyncError },
}

/// Represents an entire scene worth of rendering data.
///
/// A clone of a stage is a reference to the same stage.
#[derive(Clone)]
pub struct Stage {
    pub(crate) stage_slab: SlabBuffer,
    pub(crate) render_unit_slab: SlabBuffer,
    pub(crate) indirect_draws: SlabBuffer,
    pub(crate) atlas: Arc<Mutex<Atlas>>,
    pub(crate) skybox: Arc<Mutex<Skybox>>,
    pub(crate) pipeline: Arc<Mutex<Option<Arc<wgpu::RenderPipeline>>>>,
    pub(crate) slab_buffers_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) textures_bindgroup: Arc<Mutex<Option<Arc<wgpu::BindGroup>>>>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl Stage {
    /// Create a new stage.
    pub fn new(device: Device, queue: Queue, legend: StageLegend) -> Self {
        let mut s = Self {
            stage_slab: SlabBuffer::new(&device, 256),
            render_unit_slab: SlabBuffer::new(&device, 256),
            indirect_draws: SlabBuffer::new_usage(&device, 256, wgpu::BufferUsages::INDIRECT),
            pipeline: Default::default(),
            atlas: Arc::new(Mutex::new(Atlas::new(&device, &queue, 1, 1))),
            skybox: Arc::new(Mutex::new(Skybox::empty(&device, &queue))),
            slab_buffers_bindgroup: Default::default(),
            textures_bindgroup: Default::default(),
            device,
            queue,
        };
        let _ = s.append(&legend);
        s
    }

    /// Add an object to the slab and return its ID.
    pub fn append<T: Slabbed + Default + std::fmt::Debug>(&mut self, object: &T) -> Id<T> {
        self.stage_slab.append(&self.device, &self.queue, object)
    }

    /// Add a slice of objects to the slab and return an [`Array`].
    pub fn append_slice<T: Slabbed + Default + std::fmt::Debug>(
        &mut self,
        objects: &[T],
    ) -> Array<T> {
        self.stage_slab
            .append_slice(&self.device, &self.queue, objects)
    }

    ///// Add a render unit to the stage.
    /////
    ///// The render unit will be added to the stage and its ID will be returned.
    ///// The ID of the input render unit will be overwritten.
    //pub fn add_render_unit(&mut self, mut unit: RenderUnit) -> Id<RenderUnit> {
    //    unit.id = Id::from(self.render_unit_slab.len());
    //    self.indirect_draws.append(
    //        &self.device,
    //        &self.queue,
    //        &DrawIndirect {
    //            vertex_count: unit.vertices.len() as u32,
    //            instance_count: 1,
    //            base_vertex: 0,
    //            base_instance: unit.id.into(),
    //        },
    //    );
    //    self.render_unit_slab
    //        .append(&self.device, &self.queue, &unit)
    //}

    /// Set the debug mode.
    pub fn set_debug_mode(&mut self, debug_mode: DebugMode) {
        let id = Id::<DebugMode>::from(StageLegend::offset_of_debug_mode());
        // UNWRAP: safe because the debug mode offset is guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &debug_mode)
            .unwrap();
    }

    /// Set the debug mode.
    pub fn with_debug_mode(mut self, debug_mode: DebugMode) -> Self {
        self.set_debug_mode(debug_mode);
        self
    }

    /// Set whether the stage uses lighting.
    pub fn set_has_lighting(&mut self, use_lighting: bool) {
        let id = Id::<bool>::from(StageLegend::offset_of_has_lighting());
        // UNWRAP: safe because the has lighting offset is guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &use_lighting)
            .unwrap();
    }

    /// Set whether the stage uses lighting.
    pub fn with_lighting(mut self, use_lighting: bool) -> Self {
        self.set_has_lighting(use_lighting);
        self
    }

    /// Create a new spot light and return its builder.
    pub fn new_spot_light(&mut self) -> light::GpuSpotLightBuilder {
        light::GpuSpotLightBuilder::new(self)
    }

    /// Create a new directional light and return its builder.
    pub fn new_directional_light(&mut self) -> light::GpuDirectionalLightBuilder {
        light::GpuDirectionalLightBuilder::new(self)
    }

    /// Create a new point light and return its builder.
    pub fn new_point_light(&mut self) -> light::GpuPointLightBuilder {
        light::GpuPointLightBuilder::new(self)
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn set_light_array(
        &mut self,
        lights: impl IntoIterator<Item = Id<GpuLight>>,
    ) -> Array<Id<GpuLight>> {
        let lights = lights.into_iter().collect::<Vec<_>>();
        let light_array = self.append_slice(&lights);
        let id = Id::<Array<Id<GpuLight>>>::from(StageLegend::offset_of_light_array());
        // UNWRAP: safe because we just appended the array, and the light array offset is
        // guaranteed to be valid.
        self.stage_slab
            .write(&self.device, &self.queue, id, &light_array)
            .unwrap();
        light_array
    }

    /// Set the light array.
    ///
    /// This should be an iterator over the ids of all the lights on the stage.
    pub fn with_light_array(mut self, lights: impl IntoIterator<Item = Id<GpuLight>>) -> Self {
        self.set_light_array(lights);
        self
    }

    /// Return the render pipeline, creating it if necessary.
    pub fn get_pipeline(&self) -> Arc<wgpu::RenderPipeline> {
        fn buffers_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            let visibility = wgpu::ShaderStages::VERTEX
                | wgpu::ShaderStages::FRAGMENT
                | wgpu::ShaderStages::COMPUTE;
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
            let unit_slab = wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            };
            let indirect_draw_slab = wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            };
            let entries = vec![stage_slab, unit_slab, indirect_draw_slab];
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("stage slab buffers"),
                entries: &entries,
            })
        }

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

        fn textures_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            let (atlas, atlas_sampler) = image2d_entry(0);
            let (irradiance, irradiance_sampler) = cubemap_entry(2);
            let (prefilter, prefilter_sampler) = cubemap_entry(4);
            let (brdf, brdf_sampler) = image2d_entry(6);
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
                ],
            })
        }

        fn create_stage_render_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
            log::trace!("creating stage render pipeline");
            let label = Some("stage render pipeline");
            let vertex_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/stage-new_stage_vertex.spv"));
            let fragment_shader = device
                .create_shader_module(wgpu::include_spirv!("linkage/stage-stage_fragment.spv"));
            let stage_slab_buffers_layout = buffers_bindgroup_layout(device);
            let textures_layout = textures_bindgroup_layout(device);
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
                            format: wgpu::TextureFormat::Rgba8UnormSrgb,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                        //Some(wgpu::ColorTargetState {
                        //    format: wgpu::TextureFormat::Rgba16Float,
                        //    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        //    write_mask: wgpu::ColorWrites::ALL,
                        //}),
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
            render_unit_slab: &SlabBuffer,
            indirect_draws: &SlabBuffer,
        ) -> wgpu::BindGroup {
            let label = Some("stage slab buffers");
            let stage_slab_buffers_bindgroup =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label,
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: stage_slab.get_buffer().as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: render_unit_slab.get_buffer().as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: indirect_draws.get_buffer().as_entire_binding(),
                        },
                    ],
                });
            stage_slab_buffers_bindgroup
        }

        // UNWRAP: safe because we're only ever called from the render thread.
        let mut bindgroup = self.slab_buffers_bindgroup.lock().unwrap();
        if let Some(bindgroup) = bindgroup.as_ref() {
            bindgroup.clone()
        } else {
            let b = Arc::new(create_slab_buffers_bindgroup(
                &self.device,
                &self.get_pipeline(),
                &self.stage_slab,
                &self.render_unit_slab,
                &self.indirect_draws,
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
                // UNWRAP: we can't acquire locks we want to panic
                &self.atlas.lock().unwrap(),
                &self.skybox.lock().unwrap(),
            ));
            *bindgroup = Some(b.clone());
            b
        }
    }

    pub fn number_of_indirect_draws(&self) -> u32 {
        (self.indirect_draws.len() / DrawIndirect::slab_size()) as u32
    }

    pub fn number_of_render_units(&self) -> u32 {
        (self.render_unit_slab.len() / RenderUnit::slab_size()) as u32
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;
    use moongraph::{graph, View, ViewMut};
    use renderling_shader::{
        slab::Slab,
        stage::{Camera, RenderUnit, Vertex},
    };
    use wgpu::util::DeviceExt;

    use crate::{default_ortho2d, frame::FrameTextureView, DepthTexture, HdrSurface, Renderling};

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

    #[cfg(feature = "nene")]
    #[test]
    fn slab_shader_sanity() {
        let r = Renderling::headless(100, 100).unwrap();
        let (device, queue) = r.get_device_and_queue_owned();
        let slab = SlabBuffer::new(&device, 256);
        let vertices = slab.append_slice(&device, &queue, &right_tri_vertices());
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = slab.append(
            &device,
            &queue,
            &Camera {
                projection,
                view,
                ..Default::default()
            },
        );
        let unit = slab.append(
            &device,
            &queue,
            &RenderUnit {
                camera,
                vertices,
                ..Default::default()
            },
        );

        //// Create a bindgroup for the slab so our shader can read out the types.
        //let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //    label: Some("slab shader sanity"),
        //    entries: &[wgpu::BindGroupLayoutEntry {
        //        binding: 0,
        //        visibility: wgpu::ShaderStages::VERTEX,
        //        ty: wgpu::BindingType::Buffer {
        //            ty: wgpu::BufferBindingType::Storage { read_only: true },
        //            has_dynamic_offset: false,
        //            min_binding_size: None,
        //        },
        //        count: None,
        //    }],
        //});
        //let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //    label: Some("slab shader sanity"),
        //    bind_group_layouts: &[&bindgroup_layout],
        //    push_constant_ranges: &[],
        //});
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("slab shader sanity"),
            layout: None, //Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device
                    .create_shader_module(wgpu::include_spirv!("linkage/stage-simple_vertex.spv")),
                entry_point: "stage::simple_vertex",
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
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/stage-simple_fragment.spv"
                )),
                entry_point: "stage::simple_fragment",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        //let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //    label: Some("slab shader sanity"),
        //    layout: &bindgroup_layout,
        //    entries: &[wgpu::BindGroupEntry {
        //        binding: 0,
        //        resource: slab.get_buffer().as_entire_binding(),
        //    }],
        //});
        let depth = crate::texture::Texture::create_depth_texture(&device, 100, 100);
        let frame = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("slab shader sanity"),
            size: wgpu::Extent3d {
                width: 100,
                height: 100,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let (frame_view, _) = crate::frame::default_frame_texture_view(&frame);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("slab shader sanity"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("slab shader sanity"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                })],
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
            //render_pass.set_bind_group(0, &bindgroup, &[]);
            render_pass.draw(0..3, 0..1);
        }
        queue.submit(std::iter::once(encoder.finish()));

        let buffer = crate::Texture::read(&frame, &device, &queue, 100, 100, 4, 1);
        let img = buffer.into_rgba(&device).unwrap();
        img_diff::save("stage/slab_shader_sanity.png", img);
    }

    fn stage_render(
        (stage, frame_view, depth): (ViewMut<Stage>, View<FrameTextureView>, View<DepthTexture>),
    ) -> Result<(), StageError<RenderUnit>> {
        let label = Some("stage render");
        let pipeline = stage.get_pipeline();
        let slab_buffers_bindgroup = stage.get_slab_buffers_bindgroup();
        let textures_bindgroup = stage.get_textures_bindgroup();
        let _indirect_buffer = stage.indirect_draws.get_buffer();
        let mut encoder = stage
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
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
            //render_pass.multi_draw_indirect(&indirect_buffer, 0, stage.number_of_indirect_draws());
            render_pass.draw(0..3, 0..1);
        }
        stage.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    #[cfg(feature = "none")]
    #[test]
    fn stage_new() {
        let mut r = Renderling::headless(100, 100)
            .unwrap()
            .with_background_color(glam::Vec4::splat(1.0));
        let (device, queue) = r.get_device_and_queue_owned();
        let mut stage = Stage::new(device, queue, StageLegend::default()).with_lighting(true);
        let (projection, view) = default_ortho2d(100.0, 100.0);
        let camera = Camera {
            projection,
            view,
            position: Vec3::ZERO,
        };
        let camera_id = stage.append(&camera);
        let vertices = stage.append_slice(&right_tri_vertices());
        println!("vertices: {vertices:?}");
        let mut unit = RenderUnit {
            camera: camera_id,
            vertices,
            ..Default::default()
        };
        let unit_id = stage.add_render_unit(unit);
        unit.id = unit_id;
        assert_eq!(Id::new(0), unit_id);
        assert_eq!(1, stage.number_of_render_units());
        assert_eq!(1, stage.number_of_indirect_draws());

        let stage_slab = futures_lite::future::block_on(stage.stage_slab.read_raw::<u32>(
            &stage.device,
            &stage.queue,
            0,
            stage.stage_slab.len(),
        ))
        .unwrap();
        assert_eq!(camera, stage_slab.read(camera_id));
        assert_eq!(right_tri_vertices(), stage_slab.read_vec(vertices));
        let render_unit_slab =
            futures_lite::future::block_on(stage.render_unit_slab.read_raw::<u32>(
                &stage.device,
                &stage.queue,
                0,
                stage.render_unit_slab.len(),
            ))
            .unwrap();
        assert_eq!(unit, render_unit_slab.read(unit_id));
        let indirect_slab = futures_lite::future::block_on(stage.indirect_draws.read_raw::<u32>(
            &stage.device,
            &stage.queue,
            0,
            stage.indirect_draws.len(),
        ))
        .unwrap();
        assert_eq!(
            DrawIndirect {
                vertex_count: 3,
                instance_count: 1,
                base_vertex: 0,
                base_instance: 0,
            },
            indirect_slab.read(Id::new(0))
        );

        {
            // set up the render graph
            use crate::{
                frame::{clear_frame_and_depth, create_frame, present},
                graph::{graph, Graph},
            };
            r.graph.add_resource(stage.clone());

            let (device, queue) = r.get_device_and_queue_owned();

            // pre-render passes
            r.graph
                .add_subgraph(graph!(create_frame < clear_frame_and_depth))
                .add_barrier();
            // render passes
            r.graph.add_subgraph(graph!(stage_render)).add_barrier();
            // post-render passes
            let copy_frame_to_post = crate::frame::PostRenderBufferCreate::create;
            r.graph.add_subgraph(graph!(copy_frame_to_post < present));
        }

        let img = r.render_image().unwrap();
        img_diff::save("stage/stage_new.png", img);
    }
}

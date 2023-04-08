//! The CPU side of [`renderling_shader::scene`] module.
use std::{any::Any, marker::PhantomData};

use async_channel::{Receiver, Sender};
use glam::Mat4;
use moongraph::{IsGraphNode, Read, Write};
use renderling_shader::scene::{DrawIndirect, GpuCamera, GpuEntity, GpuMeshlet, GpuVertex};
use snafu::prelude::*;
use wgpu::util::DeviceExt;

use crate::{
    linkage, node::FrameTextureView, DepthTexture, Device, Queue, RenderTarget, Renderling,
};

#[derive(Debug, Snafu)]
pub enum SceneError {
    #[snafu(display(
        "Array has run out of capacity. Capacity is {capacity} but the operation requires \
         {required}."
    ))]
    NoCapacity { capacity: usize, required: usize },

    #[snafu(display("Out of bounds, index is {index} but length is {length}."))]
    OutOfBounds { index: usize, length: usize },

    #[snafu(display("Invalid indirect draw count"))]
    InvalidIndirectCount
}

fn scene_render_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC
}

fn scene_indirect_usage() -> wgpu::BufferUsages {
    wgpu::BufferUsages::STORAGE
        | wgpu::BufferUsages::COPY_DST
        | wgpu::BufferUsages::COPY_SRC
        | wgpu::BufferUsages::INDIRECT
}

/// Read a vector from the GPU corresponding to the given range.
///
/// This creates an output buffer, creates an encoder, submits the queue and
/// then maps the output buffer and polls the device.
pub fn read_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &wgpu::Buffer,
    start: usize,
    length: usize,
) -> Result<Vec<T>, SceneError> {
    log::trace!("reading {length} {} starting at index {start}", std::any::type_name::<T>());
    let output_buffer_size = (length * std::mem::size_of::<T>()) as u64;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("GpuArray output buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(
        buffer,
        (start * std::mem::size_of::<T>()) as u64,
        &output_buffer,
        0,
        output_buffer_size,
    );
    queue.submit(std::iter::once(encoder.finish()));

    let buffer_slice = output_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let items = bytemuck::cast_slice::<u8, T>(&buffer_slice.get_mapped_range()).to_vec();
    output_buffer.unmap();
    Ok(items)
}

fn gpu_storage_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
    device: &wgpu::Device,
    label: Option<&str>,
    contents: &[T],
    capacity: usize,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let mut contents = contents.to_vec();
    contents.resize_with(capacity, T::zeroed);
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label,
        usage,
        contents: bytemuck::cast_slice(contents.as_slice()),
    });
    buffer
}

pub struct GpuBuffer {
    // the gpu-side buffer
    pub buffer: wgpu::Buffer,
    // the number of elements stored in the buffer
    len: usize,
    // the total number of elements that can be stored in the buffer
    capacity: usize,
}

/// An array of `T` elements living on the GPU, backed by a storage buffer.
pub struct GpuArray<T: bytemuck::Pod + bytemuck::Zeroable> {
    pub buffer: GpuBuffer,
    updates: (Sender<(usize, Vec<T>)>, Receiver<(usize, Vec<T>)>),
    _phantom: PhantomData<T>,
}

impl<T: Any + Clone + Copy + bytemuck::Pod + bytemuck::Zeroable> GpuArray<T> {
    /// Create a new buffer of [`GpuMeshVertex`] on the GPU.
    pub fn new(
        device: &wgpu::Device,
        contents: &[T],
        capacity: usize,
        usage: wgpu::BufferUsages,
    ) -> Self {
        GpuArray {
            buffer: GpuBuffer {
                buffer: gpu_storage_buffer(
                    device,
                    Some("GpuArray::new"),
                    &contents,
                    capacity,
                    usage,
                ),
                len: 0,
                capacity,
            },
            updates: async_channel::unbounded(),
            _phantom: PhantomData,
        }
    }

    /// Push items onto the end of the array.
    ///
    /// Returns the index of the first item and the number of new items.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn extend(
        &mut self,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), SceneError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = self.buffer.len + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                capacity: self.buffer.capacity,
                required
            }
        );
        let start = self.buffer.len;
        self.updates.0.try_send((start, items)).unwrap();
        self.buffer.len += items_len;
        Ok((start, items_len))
    }

    /// Push an item onto the end of the array.
    ///
    /// Returns the index of the item and `1`.
    ///
    /// Errs if the array has no capacity for the item.
    pub fn push(&mut self, item: T) -> Result<(usize, usize), SceneError> {
        self.extend(vec![item])
    }

    /// Overwrite a portion of items in the array.
    ///
    /// Returns the starting index and the length.
    ///
    /// Errs if the array has no capacity for the items.
    pub fn overwrite(
        &mut self,
        start_index: usize,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(usize, usize), SceneError> {
        let items = items.into_iter().collect::<Vec<_>>();
        let items_len = items.len();
        let required = start_index + items_len;
        snafu::ensure!(
            required <= self.buffer.capacity,
            NoCapacitySnafu {
                capacity: self.buffer.capacity,
                required
            }
        );
        self.updates.0.try_send((start_index, items)).unwrap();
        self.buffer.len = self.buffer.len.max(start_index + items_len);
        Ok((start_index, items_len))
    }

    /// Return the length of the array.
    pub fn len(&self) -> usize {
        self.buffer.len
    }

    /// Update the buffer on the GPU side.
    ///
    /// This array won't be fully updated on the GPU side until the queue has
    /// been submitted.
    pub fn update(&self, queue: &wgpu::Queue) {
        while let Ok((starting_index, items)) = self.updates.1.try_recv() {
            log::trace!(
                "{} updating {} items",
                std::any::type_name::<Self>(),
                items.len()
            );
            queue.write_buffer(
                &self.buffer.buffer,
                (starting_index * std::mem::size_of::<T>()) as u64,
                bytemuck::cast_slice(items.as_slice()),
            );
        }
    }

    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buffer
    }

    /// Read a vector from the GPU corresponding to the given range.
    ///
    /// This creates an output buffer, creates an encoder, submits the queue and
    /// then maps the output buffer and polls the device.
    pub fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        start: usize,
        length: usize,
    ) -> Result<Vec<T>, SceneError> {
        read_buffer(device, queue, self.get_buffer(), start, length)
    }
}

pub struct Scene {
    next_entity: u32,
    pub vertices: GpuArray<GpuVertex>,
    pub meshlets: GpuArray<GpuMeshlet>,
    pub transforms: GpuArray<Mat4>,
    pub entities: GpuArray<GpuEntity>,
    pub camera: wgpu::Buffer,
    pub indirect_draws: GpuArray<DrawIndirect>,
    camera_update: Option<GpuCamera>,
    indirect_count_buffer: wgpu::Buffer,
    cull_bindgroup: wgpu::BindGroup,
    render_bindgroup: wgpu::BindGroup,
}

impl Scene {
    /// Graph helper to create a new scene on the GPU.
    pub fn new(
        device: &wgpu::Device,
        max_vertices: usize,
        max_meshlets: usize,
        max_transforms: usize,
        max_entities: usize,
    ) -> Self {
        let vertices = GpuArray::<GpuVertex>::new(&device, &[], max_vertices, scene_render_usage());
        let meshlets = GpuArray::<GpuMeshlet>::new(&device, &[], max_meshlets, scene_render_usage());
        let transforms = GpuArray::<Mat4>::new(&device, &[], max_transforms, scene_render_usage());
        let entities = GpuArray::<GpuEntity>::new(&device, &[], max_entities, scene_render_usage());
        let indirect_draws =
            GpuArray::<DrawIndirect>::new(&device, &[], max_entities, scene_indirect_usage());
        let indirect_count = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene::new indirect_count"),
            contents: bytemuck::cast_slice(&[0u32]),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });
        let camera = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene::new camera"),
            contents: bytemuck::cast_slice(&[GpuCamera::default()]),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let cull_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene render bindgroup"),
            layout: &scene_draw_indirect_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: indirect_draws.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: indirect_count.as_entire_binding(),
                },
            ],
        });

        let render_bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Scene::new render_bindgroup"),
            layout: &scene_bindgroup_layout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: meshlets.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: transforms.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: entities.get_buffer().as_entire_binding(),
                },
            ],
        });
        Self {
            next_entity: 0,
            render_bindgroup,
            vertices,
            meshlets,
            transforms,
            entities,
            camera_update: None,
            camera,
            indirect_draws,
            indirect_count_buffer: indirect_count,
            cull_bindgroup,
        }
    }

    /// Update the scene.
    ///
    /// This uploads changed data to the GPU and submits the queue.
    pub fn update(&mut self, queue: &wgpu::Queue) {
        let Self {
            next_entity: _,
            camera: _,
            render_bindgroup: _,
            indirect_draws: _,
            indirect_count_buffer: _,
            cull_bindgroup: _,
            vertices,
            meshlets,
            transforms,
            entities,
            camera_update,
        } = self;
        vertices.update(queue);
        meshlets.update(queue);
        transforms.update(queue);
        entities.update(queue);
        if let Some(camera) = camera_update.take() {
            queue.write_buffer(&self.camera, 0, bytemuck::cast_slice(&[camera]));
        }
        queue.submit(std::iter::empty());
    }

    /// Reads the indirect count from the count buffer.
    pub fn read_indirect_count(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<u32, SceneError> {
        match read_buffer(device, queue, &self.indirect_count_buffer, 0, 1)?.as_slice() {
            &[count] => Ok(count),
            _ => Err(SceneError::InvalidIndirectCount),
        }
    }

    /// Set the camera.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn set_camera(&mut self, camera: GpuCamera) {
        self.camera_update = Some(camera);
    }

    /// Load a new mesh.
    ///
    /// Returns the index of the meshlet in the GPU mesh buffer.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn new_meshlet(
        &mut self,
        vertices: impl IntoIterator<Item = GpuVertex>,
    ) -> Result<usize, SceneError> {
        let (start_index, vertex_count) = self.vertices.extend(vertices)?;
        let (start_index, _) = self.meshlets.push(GpuMeshlet {
            first_vertex: start_index as u32,
            vertex_count: vertex_count as u32,
        })?;
        Ok(start_index)
    }

    /// Load a new transform.
    ///
    /// Returns the index of the transform in the GPU transform buffer.
    ///
    /// The data is not uploaded to the cpu until [`Scene::update`] has been
    /// called.
    pub fn new_transform(&mut self, transform: Mat4) -> Result<usize, SceneError> {
        Ok(self.transforms.push(transform)?.0)
    }

    pub fn new_entity(&mut self) -> EntityBuilder<'_> {
        EntityBuilder {
            scene: self,
            meshlet: None,
            transform: None,
        }
    }
}

pub struct EntityBuilder<'a> {
    scene: &'a mut Scene,
    meshlet: Option<Vec<GpuVertex>>,
    transform: Option<Mat4>,
}

impl<'a> EntityBuilder<'a> {
    pub fn with_meshlet(mut self, vertices: impl IntoIterator<Item = GpuVertex>) -> Self {
        self.meshlet = Some(vertices.into_iter().collect());
        self
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn build(self) -> Result<GpuEntity, SceneError> {
        let EntityBuilder {
            scene,
            meshlet,
            transform,
        } = self;
        let mut entity = GpuEntity {
            id: scene.next_entity,
            ..Default::default()
        };
        scene.next_entity += 1;
        if let Some(vertices) = meshlet {
            let mesh_id = scene.new_meshlet(vertices)?;
            entity.mesh = mesh_id as u32;
        }
        if let Some(transform) = transform {
            let transform_id = scene.new_transform(transform)?;
            entity.transform = transform_id as u32;
        }

        let (id, _) = scene.entities.push(entity.clone())?;
        debug_assert_eq!(entity.id, id as u32);

        Ok(entity)
    }
}

pub fn scene_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entries = (0..5)
        .map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect::<Vec<_>>();
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene render"),
        entries: &entries,
    })
}

pub fn scene_draw_indirect_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("scene compute cull indirect"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

pub fn create_scene_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let label = Some("scene render pipeline");
    let shader_crate = linkage::shader_crate(device);
    let scene_buffers_layout = scene_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&scene_buffers_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader_crate,
            entry_point: "main_vertex_scene",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: Some(wgpu::Face::Back),
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
            module: &shader_crate,
            entry_point: "main_fragment_scene",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    pipeline
}

pub fn create_scene_compute_cull_pipeline(device: &wgpu::Device) -> wgpu::ComputePipeline {
    let label = Some("scene compute cull pipeline");
    let shader_crate = linkage::shader_crate(device);
    let scene_buffers_layout = scene_bindgroup_layout(device);
    let indirect_buffers_layout = scene_draw_indirect_bindgroup_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label,
        bind_group_layouts: &[&scene_buffers_layout, &indirect_buffers_layout],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label,
        layout: Some(&layout),
        module: &shader_crate,
        entry_point: "compute_cull_entities",
    });
    pipeline
}

pub struct SceneRenderPipeline(wgpu::RenderPipeline);

pub struct SceneComputeCullPipeline(wgpu::ComputePipeline);

pub fn scene_update((queue, mut scene): (Read<Queue>, Write<Scene>)) -> Result<(), SceneError> {
    scene.update(&queue);
    Ok(())
}

pub fn scene_cull(
    (device, queue, scene, pipeline): (
        Read<Device>,
        Read<Queue>,
        Read<Scene>,
        Read<SceneComputeCullPipeline>,
    ),
) -> Result<(), SceneError> {
    let label = Some("scene cull");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label });
    compute_pass.set_pipeline(&pipeline.0);
    compute_pass.set_bind_group(0, &scene.render_bindgroup, &[]);
    compute_pass.set_bind_group(1, &scene.cull_bindgroup, &[]);
    let num_entities = scene.next_entity;
    let groups = num_entities / 32 + 1;
    compute_pass.dispatch_workgroups(groups, 1, 1);
    drop(compute_pass);
    queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

fn scene_render(
    (device, queue, scene, pipeline, frame, depth): (
        Read<Device>,
        Read<Queue>,
        Read<Scene>,
        Read<SceneRenderPipeline>,
        Read<FrameTextureView>,
        Read<DepthTexture>,
    ),
) -> Result<(), SceneError> {
    let count = scene.read_indirect_count(&device, &queue)?;
    let label = Some("scene render");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &frame,
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
    render_pass.set_pipeline(&pipeline.0);
    render_pass.set_bind_group(0, &scene.render_bindgroup, &[]);
    render_pass.multi_draw_indirect(scene.indirect_draws.get_buffer(), 0, count);
    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

pub fn setup_scene_render_graph(scene: Scene, r: &mut Renderling) {
    r.add_resource(scene);

    let pipeline = SceneRenderPipeline(
        r.graph
            .visit(|(device, target): (Read<Device>, Read<RenderTarget>)| {
                create_scene_render_pipeline(&device, target.format())
            })
            .unwrap(),
    );
    r.graph.add_resource(pipeline);

    r.graph.add_node(
        crate::node::create_frame
            .into_node()
            .with_name("create_frame"),
    );
    r.graph.add_node(
        crate::node::clear_frame_and_depth
            .into_node()
            .with_name("clear_frame_and_depth"),
    );

    r.graph
        .add_node(scene_update.into_node().with_name("scene_update"));

    let pipeline = SceneComputeCullPipeline(
        r.graph
            .visit(|device: Read<Device>| create_scene_compute_cull_pipeline(&device))
            .unwrap(),
    );
    r.graph.add_resource(pipeline);

    r.graph.add_node(
        scene_cull
            .into_node()
            .with_name("scene_cull")
            .run_after("scene_update"),
    );
    r.graph.add_barrier();

    r.graph
        .add_node(scene_render.into_node().with_name("scene_render"));
    r.graph
        .add_node(crate::node::present.into_node().with_name("present"));
}

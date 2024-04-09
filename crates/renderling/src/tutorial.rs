//! A tutorial module for the renderling crate.
use crabslab::{Array, Id, Slab, SlabItem};
use glam::{Mat4, Vec4};
use spirv_std::spirv;

use crate::{stage::Vertex, Renderlet};

#[cfg(feature = "tutorial_passthru_fragment")]
/// Simple fragment shader that writes the input color to the output color.
#[spirv(fragment)]
pub fn tutorial_passthru_fragment(in_color: Vec4, output: &mut Vec4) {
    *output = in_color;
}

fn implicit_isosceles_triangle(vertex_index: u32) -> Vec4 {
    let x = (1 - vertex_index as i32) as f32 * 0.5;
    let y = ((vertex_index & 1) as f32 * 2.0 - 1.0) * 0.5;
    Vec4::new(x, y, 0.0, 1.0)
}

#[cfg(feature = "tutorial_implicit_isosceles_vertex")]
/// Simple vertex shader with an implicit isosceles triangle.
#[spirv(vertex)]
pub fn tutorial_implicit_isosceles_vertex(
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    //#[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],
    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let pos = implicit_isosceles_triangle(vertex_index);
    *out_color = Vec4::new(1.0, 0.0, 0.0, 1.0);
    *clip_pos = pos;
}

#[cfg(feature = "tutorial_slabbed_vertices_no_instance")]
/// This shader uses the vertex index as a slab [`Id`]. The [`Id`] is used to
/// read the vertex from the slab. The vertex's position and color are written
/// to the output.
#[spirv(vertex)]
pub fn tutorial_slabbed_vertices_no_instance(
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let vertex_id = Id::<Vertex>::from(vertex_index as usize * Vertex::SLAB_SIZE);
    let vertex = slab.read(vertex_id);
    *clip_pos = vertex.position.extend(1.0);
    *out_color = vertex.color;
}

#[cfg(feature = "tutorial_slabbed_vertices")]
/// This shader uses the `instance_index` as a slab [`Id`].
/// The `instance_index` is the [`Id`] of an [`Array`] of [`Vertex`]s. The
/// `vertex_index` is the index of a [`Vertex`] within the [`Array`].
#[spirv(vertex)]
pub fn tutorial_slabbed_vertices(
    // Id of the array of vertices we are rendering
    #[spirv(instance_index)] array_id: Id<Array<Vertex>>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let array = slab.read(array_id);
    let vertex_id = array.at(vertex_index as usize);
    let vertex = slab.read(vertex_id);
    *clip_pos = vertex.position.extend(1.0);
    *out_color = vertex.color;
}

#[cfg(feature = "tutorial_slabbed_renderlet")]
/// This shader uses the `instance_index` as a slab [`Id`].
/// The `instance_index` is the [`Id`] of a [`RenderUnit`].
/// The [`RenderUnit`] contains an [`Array`] of [`Vertex`]s
/// as its mesh, the [`Id`]s of a [`Material`] and [`Camera`],
/// and TRS transforms.
/// The `vertex_index` is the index of a [`Vertex`] within the
/// [`RenderUnit`]'s `vertices` [`Array`].
#[spirv(vertex)]
pub fn tutorial_slabbed_renderlet(
    // Id of the renderlet
    #[spirv(instance_index)] renderlet_id: Id<Renderlet>,
    // Which vertex within the render unit are we rendering
    #[spirv(vertex_index)] vertex_index: u32,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] slab: &[u32],

    out_color: &mut Vec4,
    #[spirv(position)] clip_pos: &mut Vec4,
) {
    let renderlet = slab.read_unchecked(renderlet_id);
    let vertex_id = renderlet.geometry.at(vertex_index as usize);
    let vertex = slab.read(vertex_id);
    *out_color = vertex.color;

    let transform = slab.read(renderlet.transform);
    let model = Mat4::from_scale_rotation_translation(
        transform.scale,
        transform.rotation,
        transform.translation,
    );
    let camera = slab.read(renderlet.camera);
    *clip_pos = camera.projection * camera.view * model * vertex.position.extend(1.0);
}

#[cfg(test)]
mod test {
    use crabslab::*;
    use glam::{Vec3, Vec4, Vec4Swizzles};

    use crate::{
        frame::FrameTextureView,
        graph::{graph, Graph, GraphError, NoDefault, View},
        Camera, DepthTexture, Device, Queue, Renderlet, Renderling, Transform, Vertex,
    };

    #[test]
    fn tutorial_implicit_isosceles_triangle() {
        let mut r = Renderling::headless(100, 100);
        let (device, _queue) = r.get_device_and_queue_owned();
        let label = Some("implicit isosceles triangle");
        let vertex = crate::linkage::tutorial_implicit_isosceles_vertex::linkage(&device);
        let fragment = crate::linkage::tutorial_passthru_fragment::linkage(&device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: vertex.entry_point,
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
                module: &fragment.module,
                entry_point: fragment.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        fn render(
            (device, queue, pipeline, frame, depth): (
                View<Device, NoDefault>,
                View<Queue, NoDefault>,
                View<wgpu::RenderPipeline, NoDefault>,
                View<FrameTextureView, NoDefault>,
                View<DepthTexture, NoDefault>,
            ),
        ) -> Result<(), GraphError> {
            let label = Some("implicit isosceles triangle");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
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
                render_pass.draw(0..3, 0..1);
            }
            queue.submit(std::iter::once(encoder.finish()));
            Ok(())
        }

        use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
        r.graph.add_resource(pipeline);
        r.graph.add_subgraph(graph!(
            create_frame
                < clear_frame_and_depth
                < render
                < copy_frame_to_post
                < present
        ));

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("tutorial/implicit_isosceles_triangle.png", img);
    }

    #[test]
    fn slabbed_isosceles_triangle_no_instance() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let mut slab = WgpuBuffer::new(&device, &queue, 256);
        let initial_vertices = [
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let vertices = slab.append_array(&initial_vertices);
        assert_eq!(3, vertices.len());

        // Create a bindgroup for the slab so our shader can read out the types.
        let label = Some("slabbed isosceles triangle");
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

        let vertex = crate::linkage::tutorial_slabbed_vertices_no_instance::linkage(&device);
        let fragment = crate::linkage::tutorial_passthru_fragment::linkage(&device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: vertex.entry_point,
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
                module: &fragment.module,
                entry_point: &fragment.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: slab.get_buffer().as_entire_binding(),
            }],
        });

        struct App {
            pipeline: wgpu::RenderPipeline,
            bindgroup: wgpu::BindGroup,
            vertices: Array<Vertex>,
        }

        let app = App {
            pipeline,
            bindgroup,
            vertices,
        };
        r.graph.add_resource(app);

        fn render(
            (device, queue, app, frame, depth): (
                View<Device, NoDefault>,
                View<Queue, NoDefault>,
                View<App, NoDefault>,
                View<FrameTextureView, NoDefault>,
                View<DepthTexture, NoDefault>,
            ),
        ) -> Result<(), GraphError> {
            let label = Some("slabbed isosceles triangle");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
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
                render_pass.set_pipeline(&app.pipeline);
                render_pass.set_bind_group(0, &app.bindgroup, &[]);
                render_pass.draw(0..app.vertices.len() as u32, 0..1);
            }
            queue.submit(std::iter::once(encoder.finish()));
            Ok(())
        }

        use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
        r.graph.add_subgraph(graph!(
            create_frame
                < clear_frame_and_depth
                < render
                < copy_frame_to_post
                < present
        ));

        // assert that we're reading the data correctly
        let data = slab.block_on_read_raw(..).unwrap();
        let mut vertices = vec![];
        for i in 0..3 {
            let mut out_color = Vec4::ONE;
            let mut clip_pos = Vec4::ZERO;
            super::tutorial_slabbed_vertices_no_instance(i, &data, &mut out_color, &mut clip_pos);
            pretty_assertions::assert_eq!(super::implicit_isosceles_triangle(i), clip_pos);
            vertices.push(
                Vertex::default()
                    .with_position(clip_pos.xyz())
                    .with_color(out_color),
            );
        }

        pretty_assertions::assert_eq!(&initial_vertices, vertices.as_slice());

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_isosceles_triangle_no_instance.png", img);
    }

    #[test]
    fn tutorial_slabbed_isosceles_triangle() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let mut slab = WgpuBuffer::new(&device, &queue, 16);
        let geometry = vec![
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 1.0, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let vertices = slab.append_array(&geometry);
        let vertices_id = slab.append(&vertices);

        // Create a bindgroup for the slab so our shader can read out the types.
        let label = Some("slabbed isosceles triangle");
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });
        let vertex = crate::linkage::tutorial_slabbed_vertices::linkage(&device);
        let fragment = crate::linkage::tutorial_passthru_fragment::linkage(&device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: vertex.entry_point,
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
                module: &fragment.module,
                entry_point: fragment.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: slab.get_buffer().as_entire_binding(),
            }],
        });

        struct App {
            pipeline: wgpu::RenderPipeline,
            bindgroup: wgpu::BindGroup,
            vertices_id: Id<Array<Vertex>>,
            vertices: Array<Vertex>,
        }

        let app = App {
            pipeline,
            bindgroup,
            vertices_id,
            vertices,
        };
        r.graph.add_resource(app);

        fn render(
            (device, queue, app, frame, depth): (
                View<Device, NoDefault>,
                View<Queue, NoDefault>,
                View<App, NoDefault>,
                View<FrameTextureView, NoDefault>,
                View<DepthTexture, NoDefault>,
            ),
        ) -> Result<(), GraphError> {
            let label = Some("slabbed isosceles triangle");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
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
                render_pass.set_pipeline(&app.pipeline);
                render_pass.set_bind_group(0, &app.bindgroup, &[]);
                render_pass.draw(
                    0..app.vertices.len() as u32,
                    app.vertices_id.inner()..app.vertices_id.inner() + 1,
                );
            }
            queue.submit(std::iter::once(encoder.finish()));
            Ok(())
        }

        use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
        r.graph.add_subgraph(graph!(
            create_frame
                < clear_frame_and_depth
                < render
                < copy_frame_to_post
                < present
        ));

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_isosceles_triangle.png", img);
    }

    #[test]
    fn tutorial_slabbed_renderlet() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let mut slab = WgpuBuffer::new(&device, &queue, 16);
        let geometry = slab.append_array(&[
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 1.0, 0.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ]);
        let (projection, view) = crate::default_ortho2d(100.0, 100.0);
        let camera = slab.append(&Camera::new(projection, view));
        let transform = slab.append(&Transform {
            translation: Vec3::new(50.0, 50.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 1.0),
            ..Default::default()
        });
        let renderlet = Renderlet {
            camera,
            transform,
            geometry,
            ..Default::default()
        };
        let unit_id = slab.append(&renderlet);

        // Create a bindgroup for the slab so our shader can read out the types.
        let label = Some("slabbed renderlet");
        let bindgroup_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&bindgroup_layout],
            push_constant_ranges: &[],
        });

        let vertex = crate::linkage::tutorial_slabbed_renderlet::linkage(&device);
        let fragment = crate::linkage::tutorial_passthru_fragment::linkage(&device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex.module,
                entry_point: vertex.entry_point,
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
                module: &fragment.module,
                entry_point: fragment.entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &bindgroup_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: slab.get_buffer().as_entire_binding(),
            }],
        });

        struct App {
            pipeline: wgpu::RenderPipeline,
            bindgroup: wgpu::BindGroup,
            unit_id: Id<Renderlet>,
            unit_vertex_count: u32,
        }

        let app = App {
            pipeline,
            bindgroup,
            unit_id,
            unit_vertex_count: renderlet.geometry.len() as u32,
        };
        r.graph.add_resource(app);

        fn render(
            (device, queue, app, frame, depth): (
                View<Device, NoDefault>,
                View<Queue, NoDefault>,
                View<App, NoDefault>,
                View<FrameTextureView, NoDefault>,
                View<DepthTexture, NoDefault>,
            ),
        ) -> Result<(), GraphError> {
            let label = Some("slabbed renderlet");
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
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
                render_pass.set_pipeline(&app.pipeline);
                render_pass.set_bind_group(0, &app.bindgroup, &[]);
                render_pass.draw(
                    0..app.unit_vertex_count,
                    app.unit_id.inner()..app.unit_id.inner() + 1,
                );
            }
            queue.submit(std::iter::once(encoder.finish()));
            Ok(())
        }

        use crate::frame::{clear_frame_and_depth, copy_frame_to_post, create_frame, present};
        r.graph.add_subgraph(graph!(
            create_frame
                < clear_frame_and_depth
                < render
                < copy_frame_to_post
                < present
        ));

        let img = r.render_linear_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_renderlet.png", img);
    }
}

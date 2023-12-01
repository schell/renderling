//! A tutorial module for the renderling crate.

#[cfg(test)]
mod test {
    use glam::{Vec3, Vec4};

    use crate::{
        frame::FrameTextureView,
        graph::{graph, Graph, GraphError, View},
        shader::{
            array::Array,
            id::Id,
            stage::{Camera, RenderUnit, Vertex},
        },
        DepthTexture, Device, Queue, Renderling,
    };

    #[test]
    fn implicit_isosceles_triangle() {
        let mut r = Renderling::headless(100, 100);
        let (device, _queue) = r.get_device_and_queue_owned();
        let label = Some("implicit isosceles triangle");
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: None,
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/tutorial-implicit_isosceles_vertex.spv"
                )),
                entry_point: "tutorial::implicit_isosceles_vertex",
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
                    "linkage/tutorial-passthru_fragment.spv"
                )),
                entry_point: "tutorial::passthru_fragment",
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
                View<Device>,
                View<Queue>,
                View<wgpu::RenderPipeline>,
                View<FrameTextureView>,
                View<DepthTexture>,
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
        let slab = crate::slab::SlabBuffer::new(&device, 16);
        let vertices = slab.append_array(
            &device,
            &queue,
            &[
                Vertex {
                    position: Vec4::new(0.5, -0.5, 0.0, 1.0),
                    color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(0.0, 0.5, 0.0, 1.0),
                    color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                    ..Default::default()
                },
                Vertex {
                    position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
                    color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                    ..Default::default()
                },
            ],
        );
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/tutorial-slabbed_vertices_no_instance.spv"
                )),
                entry_point: "tutorial::slabbed_vertices_no_instance",
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
                    "linkage/tutorial-passthru_fragment.spv"
                )),
                entry_point: "tutorial::passthru_fragment",
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
                View<Device>,
                View<Queue>,
                View<App>,
                View<FrameTextureView>,
                View<DepthTexture>,
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

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_isosceles_triangle_no_instance.png", img);
    }

    #[test]
    fn slabbed_isosceles_triangle() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let slab = crate::slab::SlabBuffer::new(&device, 16);
        let geometry = vec![
            Vertex {
                position: Vec4::new(0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 1.0, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let vertices = slab.append_array(&device, &queue, &geometry);
        let vertices_id = slab.append(&device, &queue, &vertices);

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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/tutorial-slabbed_vertices.spv"
                )),
                entry_point: "tutorial::slabbed_vertices",
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
                    "linkage/tutorial-passthru_fragment.spv"
                )),
                entry_point: "tutorial::passthru_fragment",
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
                View<Device>,
                View<Queue>,
                View<App>,
                View<FrameTextureView>,
                View<DepthTexture>,
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

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_isosceles_triangle.png", img);
    }

    #[test]
    fn slabbed_render_unit() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let slab = crate::slab::SlabBuffer::new(&device, 16);
        let geometry = vec![
            Vertex {
                position: Vec4::new(0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 1.0, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let vertices = slab.append_array(&device, &queue, &geometry);
        let unit = RenderUnit {
            vertices,
            ..Default::default()
        };
        let unit_id = slab.append(&device, &queue, &unit);

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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/tutorial-slabbed_render_unit.spv"
                )),
                entry_point: "tutorial::slabbed_render_unit",
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
                    "linkage/tutorial-passthru_fragment.spv"
                )),
                entry_point: "tutorial::passthru_fragment",
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
            unit_id: Id<RenderUnit>,
            unit: RenderUnit,
        }

        let app = App {
            pipeline,
            bindgroup,
            unit_id,
            unit,
        };
        r.graph.add_resource(app);

        fn render(
            (device, queue, app, frame, depth): (
                View<Device>,
                View<Queue>,
                View<App>,
                View<FrameTextureView>,
                View<DepthTexture>,
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
                render_pass.set_pipeline(&app.pipeline);
                render_pass.set_bind_group(0, &app.bindgroup, &[]);
                render_pass.draw(
                    0..app.unit.vertices.len() as u32,
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

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_render_unit.png", img);
    }

    #[test]
    fn slabbed_render_unit_camera() {
        let mut r = Renderling::headless(100, 100);
        let (device, queue) = r.get_device_and_queue_owned();

        // Create our geometry on the slab.
        // Don't worry too much about capacity, it can grow.
        let slab = crate::slab::SlabBuffer::new(&device, 16);
        let geometry = vec![
            Vertex {
                position: Vec4::new(0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-0.5, -0.5, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 1.0, 0.0, 1.0),
                color: Vec4::new(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(-1.0, 0.0, 0.0, 1.0),
                color: Vec4::new(0.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            Vertex {
                position: Vec4::new(0.0, 1.0, 0.0, 1.0),
                color: Vec4::new(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ];
        let vertices = slab.append_array(&device, &queue, &geometry);
        let (projection, view) = crate::default_ortho2d(100.0, 100.0);
        let camera_id = slab.append(
            &device,
            &queue,
            &Camera {
                projection,
                view,
                ..Default::default()
            },
        );
        let unit = RenderUnit {
            vertices,
            camera: camera_id,
            position: Vec3::new(50.0, 50.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 1.0),
            ..Default::default()
        };
        let unit_id = slab.append(&device, &queue, &unit);

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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &device.create_shader_module(wgpu::include_spirv!(
                    "linkage/tutorial-slabbed_render_unit.spv"
                )),
                entry_point: "tutorial::slabbed_render_unit",
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
                    "linkage/tutorial-passthru_fragment.spv"
                )),
                entry_point: "tutorial::passthru_fragment",
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
            unit_id: Id<RenderUnit>,
            unit: RenderUnit,
        }

        let app = App {
            pipeline,
            bindgroup,
            unit_id,
            unit,
        };
        r.graph.add_resource(app);

        fn render(
            (device, queue, app, frame, depth): (
                View<Device>,
                View<Queue>,
                View<App>,
                View<FrameTextureView>,
                View<DepthTexture>,
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
                render_pass.set_pipeline(&app.pipeline);
                render_pass.set_bind_group(0, &app.bindgroup, &[]);
                render_pass.draw(
                    0..app.unit.vertices.len() as u32,
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

        let img = r.render_image().unwrap();
        img_diff::assert_img_eq("tutorial/slabbed_render_unit_camera.png", img);
    }
}

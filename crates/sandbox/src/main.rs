use renderling::{
    frame::{clear_frame_and_depth, create_frame, present, FrameTextureView},
    graph::{graph, Graph},
    math::{Vec3, Vec4},
    DepthTexture, Device, GraphError, Queue, Renderling, View,
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 100,
        height: 100,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer");
    let window = window_builder.build(&event_loop).unwrap();

    let mut r = Renderling::try_from_window(&window)
        .unwrap()
        .with_background_color(Vec3::splat(0.0).extend(1.0));
    //let (projection, view) = renderling::default_ortho2d(100.0, 100.0);

    let (device, _queue) = r.get_device_and_queue_owned();
    //let slab = SlabBuffer::new(&device, 256);
    //let vertices = slab.append_slice(&device, &queue, &right_tri_vertices());
    //let (projection, view) = default_ortho2d(100.0, 100.0);
    //let camera = slab.append(
    //    &device,
    //    &queue,
    //    &Camera {
    //        projection,
    //        view,
    //        ..Default::default()
    //    },
    //);
    //let unit = slab.append(
    //    &device,
    //    &queue,
    //    &RenderUnit {
    //        camera,
    //        vertices,
    //        ..Default::default()
    //    },
    //);

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
            module: &device.create_shader_module(wgpu::include_spirv!(
                "../../renderling/src/linkage/stage-simple_vertex.spv"
            )),
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
                "../../renderling/src/linkage/stage-simple_fragment.spv"
            )),
            entry_point: "stage::simple_fragment",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
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

    fn render(
        (device, queue, pipeline, frame, depth): (
            View<Device>,
            View<Queue>,
            View<wgpu::RenderPipeline>,
            View<FrameTextureView>,
            View<DepthTexture>,
        ),
    ) -> Result<(), GraphError> {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("slab shader sanity"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("slab shader sanity"),
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
            //render_pass.set_bind_group(0, &bindgroup, &[]);
            render_pass.draw(0..3, 0..1);
        }
        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    r.graph.add_resource(pipeline);
    r.graph
        .add_subgraph(graph!(create_frame < clear_frame_and_depth < render < present));

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;

        match &event {
            winit::event::Event::WindowEvent { event, .. } => match &event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                winit::event::WindowEvent::Resized(size) => {
                    r.resize(size.width, size.height);
                }
                _ => {}
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::RedrawEventsCleared => {
                r.get_device().poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                r.render().unwrap();
            }
            _ => {}
        }
    })
}

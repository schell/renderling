use renderling::{
    math::{Vec3, Vec4},
    GpuVertex, Renderling,
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
    let (projection, view) = renderling::default_ortho2d(100.0, 100.0);
    let mut builder = r.new_scene().with_camera(projection, view);
    let size = 1.0;
    let cyan_tri = builder
        .new_entity()
        .with_meshlet(vec![
            GpuVertex {
                position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(size, 0.0, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(size, size, 0.0, 0.0),
                color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                ..Default::default()
            },
        ])
        .with_position(Vec3::new(25.0, 25.0, 0.0))
        .with_scale(Vec3::new(25.0, 25.0, 1.0))
        .build();
    let _yellow_tri = builder
        .new_entity()
        .with_meshlet(vec![
            GpuVertex {
                position: Vec4::new(0.0, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(size, 0.0, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
            GpuVertex {
                position: Vec4::new(size, size, 0.0, 0.0),
                color: Vec4::new(1.0, 1.0, 0.0, 1.0),
                ..Default::default()
            },
        ])
        .with_position(Vec3::new(25.0, 25.0, 0.1))
        .with_parent(&cyan_tri)
        .build();
    let scene = builder.build().unwrap();
    r.setup_render_graph(Some(scene), None, [], false);

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

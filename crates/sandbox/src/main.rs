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

//! Main entry point for the gltf viewer.

use renderling::Renderling;

// mod demo;
mod gltf;

fn run() -> Result<(), anyhow::Error> {
    env_logger::Builder::default()
        .filter_module("example", log::LevelFilter::Trace)
        .filter_module("renderling", log::LevelFilter::Trace)
        //.filter_module("naga", log::LevelFilter::Warn)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .init();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer");
    let window = window_builder.build(&event_loop)?;

    // Set up a new renderling
    let mut r = Renderling::try_from_window(&window)
        .unwrap()
        .with_background_color(renderling::math::Vec3::splat(0x33 as f32 / 255.0).extend(1.0));
    let model = std::env::args().skip(1).next();
    let mut run_current_frame: Box<dyn FnMut(&mut Renderling, Option<&winit::event::WindowEvent>)> =
        Box::new(gltf::demo(&mut r, model));
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
                _ => (run_current_frame)(&mut r, Some(event)),
            },
            winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            winit::event::Event::RedrawEventsCleared => {
                r.get_device().poll(wgpu::Maintain::Wait);
            }
            winit::event::Event::RedrawRequested(_) => {
                (run_current_frame)(&mut r, None);
            }
            _ => {}
        }
    })
}

fn main() {
    run().unwrap();
}

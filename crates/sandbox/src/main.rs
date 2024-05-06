//! This is a sandbox.
//!
//! This program will change on a whim and does not contain anything all that
//! useful.
use renderling::{math::UVec2, Context};
use std::sync::Arc;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_size = winit::dpi::LogicalSize {
        width: 100,
        height: 100,
    };
    let window_builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer");
    let window = Arc::new(window_builder.build(&event_loop).unwrap());
    let mut ctx = Context::from_window(window);
    {
        let _frame = ctx.get_next_frame().unwrap();
        let _frame2 = ctx.get_next_frame().unwrap();
    }
    let mut stage = ctx.new_stage();
    event_loop
        .run(move |event, target| {
            target.set_control_flow(winit::event_loop::ControlFlow::Poll);

            match &event {
                winit::event::Event::WindowEvent { event, .. } => match &event {
                    winit::event::WindowEvent::CloseRequested
                    | winit::event::WindowEvent::KeyboardInput {
                        event:
                            winit::event::KeyEvent {
                                logical_key:
                                    winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                                ..
                            },
                        ..
                    } => target.exit(),

                    winit::event::WindowEvent::Resized(size) => {
                        let size = UVec2::new(size.width, size.height);
                        ctx.set_size(size);
                        stage.set_size(size)
                    }

                    winit::event::WindowEvent::RedrawRequested => {
                        ctx.get_device().poll(wgpu::Maintain::Wait);
                    }
                    _ => {}
                },
                winit::event::Event::AboutToWait => {
                    let frame = ctx.get_next_frame().unwrap();
                    stage.render(&frame.view());
                    frame.present();
                }
                _ => {}
            }
        })
        .unwrap()
}

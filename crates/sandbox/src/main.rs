//! This is a sandbox.
//!
//! This program will change on a whim and does not contain anything all that
//! useful.
use renderling::{math::UVec2, stage::Stage, Context};
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    window::{Window, WindowAttributes},
};

#[allow(dead_code)]
struct Example {
    ctx: Context,
    window: Arc<Window>,
    stage: Stage,
}

impl Example {
    fn event(&mut self, event: winit::event::WindowEvent) -> bool {
        match &event {
            winit::event::WindowEvent::CloseRequested
            | winit::event::WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                        ..
                    },
                ..
            } => return true,

            winit::event::WindowEvent::Resized(size) => {
                let size = UVec2::new(size.width, size.height);
                self.ctx.set_size(size);
                self.stage.set_size(size)
            }

            winit::event::WindowEvent::RedrawRequested => {
                self.ctx.get_device().poll(wgpu::Maintain::Wait);
            }
            _ => {}
        }

        false
    }
}

#[derive(Default)]
struct App {
    example: Option<Example>,
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = WindowAttributes::default()
            .with_inner_size(LogicalSize {
                width: 100,
                height: 100,
            })
            .with_title("renderling gltf viewer");
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let ctx = Context::from_window(None, window.clone());
        let stage = ctx.new_stage();
        self.example = Some(Example { ctx, window, stage });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(example) = self.example.as_mut() {
            if example.event(event) {
                event_loop.exit();
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(example) = self.example.as_mut() {
            let frame = example.ctx.get_next_frame().unwrap();
            example.stage.render(&frame.view());
            frame.present();
        }
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

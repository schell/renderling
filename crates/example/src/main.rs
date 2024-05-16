//! Main entry point for the gltf viewer.
use std::sync::Arc;

use clap::Parser;
use example::App;
use renderling::{math::UVec2, Context};
use winit::{application::ApplicationHandler, event::WindowEvent, window::WindowAttributes};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Optional gltf model to load at startup
    #[arg(short, long)]
    model: Option<String>,

    /// Optional HDR image to use as skybox at startup
    #[arg(short, long)]
    skybox: Option<String>,
}

struct InnerApp {
    ctx: Context,
    app: App,
}

impl InnerApp {
    fn tick(&mut self) {
        self.app.tick_loads();
        self.app.update_camera_view();
        self.app.animate();
    }

    fn event(&mut self, event: WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, up) => up,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };

                self.app.zoom(delta);
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.app.pan(position);
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = matches!(state, winit::event::ElementState::Pressed);
                let is_left_button = matches!(button, winit::event::MouseButton::Left);
                self.app.mouse_button(is_pressed, is_left_button);
            }
            winit::event::WindowEvent::DroppedFile(path) => {
                log::trace!("got dropped file event: {}", path.display());
                let path = format!("{}", path.display());
                self.app.load(&path);
            }

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
                self.app.set_size(size);
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.ctx.get_device().poll(wgpu::Maintain::Wait);
            }
            _ => {}
        }
        false
    }
}

struct OuterApp {
    cli: Cli,
    inner: Option<InnerApp>,
}

impl ApplicationHandler for OuterApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_size = winit::dpi::LogicalSize {
            width: 800,
            height: 600,
        };
        let attributes = WindowAttributes::default()
            .with_inner_size(window_size)
            .with_title("renderling gltf viewer");
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        // Set up a new renderling context
        let ctx = Context::try_from_window(window.clone()).unwrap();
        let mut app = App::new(&ctx);
        if let Some(file) = self.cli.model.as_ref() {
            app.load(file.as_ref());
        }
        if let Some(file) = self.cli.skybox.as_ref() {
            app.load(file.as_ref());
        }
        self.inner = Some(InnerApp { ctx, app });
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(inner) = self.inner.as_mut() {
            inner.tick();
            let frame = inner.ctx.get_next_frame().unwrap();
            inner.app.stage.render(&frame.view());
            frame.present();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(inner) = self.inner.as_mut() {
            if inner.event(event) {
                event_loop.exit();
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::default()
        .filter_module("example", log::LevelFilter::Trace)
        .filter_module("renderling", log::LevelFilter::Debug)
        .filter_module("renderling::stage::cpu", log::LevelFilter::Debug)
        .filter_module("renderling::slab", log::LevelFilter::Debug)
        .filter_module(
            "renderling::stage::gltf_support::anime",
            log::LevelFilter::Debug,
        )
        //.filter_module("naga", log::LevelFilter::Warn)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut outer_app = OuterApp { cli, inner: None };
    event_loop.run_app(&mut outer_app).unwrap();
}

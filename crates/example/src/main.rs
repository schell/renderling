//! Main entry point for the gltf viewer.
use std::sync::Arc;

use clap::Parser;
use example::{camera::CameraControl, App};
use renderling::{
    math::{UVec2, Vec2},
    Context,
};
use winit::{application::ApplicationHandler, event::WindowEvent, window::WindowAttributes};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Optional gltf model to load at startup
    #[arg(short, long)]
    model: Option<String>,

    /// Optional HDR image to use as skybox at startup
    #[arg(short, long)]
    skybox: Option<String>,

    /// Camera control scheme
    #[arg(short, long, default_value = "turntable")]
    camera_control: CameraControl,
    // /// Optional number of repeat instances of the same model
    // #[arg(short, long)]
    // repeat_n: Option<u32>,
}

struct InnerApp {
    ctx: Context,
    app: App,
}

impl InnerApp {
    fn tick(&mut self) {
        self.app.tick();
    }

    fn event(&mut self, event: WindowEvent) -> bool {
        match event {
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
            e => self.app.camera_controller.window_event(e),
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
        let ctx = Context::try_from_window(None, window.clone()).unwrap();
        let mut app = App::new(&ctx, self.cli.camera_control);
        if let Some(file) = self.cli.model.as_ref() {
            log::info!("loading model '{file}'");
            app.load(file.as_ref());
        }
        if let Some(file) = self.cli.skybox.as_ref() {
            log::info!("loading skybox '{file}'");
            app.load(file.as_ref());
        }
        self.inner = Some(InnerApp { ctx, app });
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(inner) = self.inner.as_mut() {
            inner.tick();
            inner.app.render(&inner.ctx);
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

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(inner) = self.inner.as_mut() {
            if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                inner
                    .app
                    .camera_controller
                    .mouse_motion(Vec2::new(delta.0 as f32, delta.1 as f32))
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    env_logger::builder().init();
    log::info!("starting up with options: {cli:#?}");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut outer_app = OuterApp { cli, inner: None };
    event_loop.run_app(&mut outer_app).unwrap();
}

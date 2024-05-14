//! Main entry point for the gltf viewer.
use std::sync::Arc;

use clap::Parser;
use example::App;
use renderling::{math::UVec2, Context};

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
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
            .with_title("renderling gltf viewer")
            .build(&event_loop)
            .unwrap(),
    );

    // Set up a new renderling
    let mut ctx = Context::try_from_window(window.clone()).unwrap();
    let mut app = App::new(&ctx);
    if let Some(file) = cli.model {
        app.load(file.as_ref());
    }
    if let Some(file) = cli.skybox {
        app.load(file.as_ref());
    }
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
                        app.set_size(size);
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        ctx.get_device().poll(wgpu::Maintain::Wait);
                    }

                    _ => tick(&mut app, Some(event)),
                },
                winit::event::Event::AboutToWait => {
                    tick(&mut app, None);
                    let frame = ctx.get_next_frame().unwrap();
                    app.stage.render(&frame.view());
                    frame.present();
                }
                _ => {}
            }
        })
        .unwrap()
}

pub fn tick(app: &mut App, ev: Option<&winit::event::WindowEvent>) {
    if let Some(ev) = ev {
        match ev {
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, up) => *up,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };

                app.zoom(delta);
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                app.pan(*position);
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = matches!(state, winit::event::ElementState::Pressed);
                let is_left_button = matches!(button, winit::event::MouseButton::Left);
                app.mouse_button(is_pressed, is_left_button);
            }
            winit::event::WindowEvent::DroppedFile(path) => {
                log::trace!("got dropped file event: {}", path.display());
                let path = format!("{}", path.display());
                app.load(&path);
            }
            _ => {}
        }
    }

    // if let Some(ev) = event_state.event_from_winit(ev) {
    //     let scene = r.graph.get_resource_mut::<Scene>().unwrap().unwrap();
    //     let channel = scene.get_debug_channel();
    //     let mut set_debug_channel = |mode| {
    //         log::debug!("setting debug mode to {mode:?}");
    //         if channel != mode {
    //             scene.set_debug_channel(mode);
    //         } else {
    //             scene.set_debug_channel(DebugChannel::None);
    //         }
    //     };

    //     match app.ui.event(ev) {
    //         None => {}
    //         Some(ev) => match ev {
    //             UiEvent::ToggleDebugChannel(channel) =>
    // set_debug_channel(channel),         },
    //     }
    // }

    app.tick_loads();
    app.update_camera_view();
    app.animate();
}

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use renderling::Renderling;
use winit::event_loop::EventLoopWindowTarget;

fn main() {
    //std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // fern::Dispatch::new()
    //     .level(log::LevelFilter::Trace)
    //     .level_for("renderling", log::LevelFilter::Trace)
    //     .level_for("wgpu", log::LevelFilter::Warn)
    //     .level_for("naga", log::LevelFilter::Warn)
    //     .chain(std::io::stdout())
    //     .apply()
    //     .unwrap();

    env_logger::Builder::default()
        .filter_module("sdfshop", log::LevelFilter::Trace)
        .filter_module("renderling", log::LevelFilter::Trace)
        //.filter_module("naga", log::LevelFilter::Warn)
        .filter_module("wgpu", log::LevelFilter::Warn)
        .init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    let builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling sdfshop");
    // #[cfg(target_arch = "wasm32")]
    // {
    //     use wasm_bindgen::JsCast;
    //     use winit::platform::web::WindowBuilderExtWebSys;
    //     let window = web_sys::window().unwrap();
    //     let document = window.document().unwrap();
    //     let body = document.body().unwrap();
    //     let canvas = document
    //         .create_element("canvas")
    //         .unwrap()
    //         .dyn_into::<web_sys::HtmlCanvasElement>()
    //         .unwrap();
    //     body.append_child(&canvas).unwrap();
    //     builder = builder.with_canvas(Some(canvas));
    // }
    let window = builder.build(&event_loop).unwrap();
    let window = Arc::new(window);
    log::info!("creating renderling");
    let mut r = Renderling::try_from_window(window.clone()).unwrap();
    // {
    //     use crabslab::{GrowableSlab, Id};
    //     use renderling::Vertex;

    //     let mut stage = r.new_stage();
    //     stage.configure_graph(&mut r, false);
    //     let (projection, view) = renderling::default_ortho2d(100.0, 100.0);
    //     let camera = stage.append(&renderling::Camera {
    //         projection,
    //         view,
    //         ..Default::default()
    //     });

    //     fn right_tri_vertices() -> Vec<Vertex> {
    //         vec![
    //             Vertex::default()
    //                 .with_position([0.0, 0.0, 0.5])
    //                 .with_color([0.0, 1.0, 1.0, 1.0]),
    //             Vertex::default()
    //                 .with_position([0.0, 100.0, 0.5])
    //                 .with_color([1.0, 1.0, 0.0, 1.0]),
    //             Vertex::default()
    //                 .with_position([100.0, 0.0, 0.5])
    //                 .with_color([1.0, 0.0, 1.0, 1.0]),
    //         ]
    //     }

    //     let mesh = stage
    //         .new_mesh()
    //         .with_primitive(right_tri_vertices(), [], Id::NONE)
    //         .build();
    //     let mesh = stage.append(&mesh);
    //     let node = stage.append(&renderling::gltf::GltfNode {
    //         mesh,
    //         ..Default::default()
    //     });
    //     let node_path = stage.append_array(&[node]);
    //     let _tri = stage.draw_gltf_rendering(&renderling::gltf::GltfRendering {
    //         camera,
    //         node_path,
    //         vertex_count: 3,
    //         ..Default::default()
    //     });
    // }
    let mut raymarcher =
        renderling::sdf::raymarch::renderer::RaymarchingRenderer::from_renderling(&mut r, 256);
    sdfshop::create_raymarch(&mut raymarcher, &mut r);
    raymarcher.configure_graph(&mut r, false);
    // r.graph.save_graph_dot("raymarch_graph.dot");
    log::info!("...created renderling w/ raymarcher");

    // #[cfg(target_arch = "wasm32")]
    // let event_loop_fn = {
    //     use winit::platform::web::EventLoopExtWebSys;
    //     winit::event_loop::EventLoop::spawn
    // };
    //#[cfg(not(target_arch = "wasm32"))]
    let event_loop_fn = winit::event_loop::EventLoop::run;

    let mut frame_times = [0.0f32; 100];
    let mut frame_times_index = 0;
    let mut last_frame = Instant::now();
    let mut last_frame_print = Instant::now();

    let _result = (event_loop_fn)(
        event_loop,
        move |event: winit::event::Event<()>, target: &EventLoopWindowTarget<()>| {
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
                    winit::event::WindowEvent::RedrawRequested => {
                        // redraw
                        r.render().unwrap();
                    }

                    _ => {
                        // do the other stuff
                    }
                },
                winit::event::Event::AboutToWait => {
                    let now = Instant::now();
                    let elapsed = (now - last_frame).as_secs_f32();
                    last_frame = now;
                    frame_times_index = (frame_times_index + 1) % frame_times.len();
                    frame_times[frame_times_index] = elapsed;
                    if (now - last_frame_print) > Duration::from_secs(1) {
                        last_frame_print = now;
                        let total_seconds = frame_times.iter().sum::<f32>();
                        let total_frames = frame_times.len() as f32;
                        let fps = total_frames / total_seconds;
                        log::info!("fps: {fps}");
                    }

                    window.request_redraw();

                    r.get_device().poll(wgpu::Maintain::Wait);
                }
                _ => {}
            }
        },
    );
    #[cfg(not(target_arch = "wasm32"))]
    _result.unwrap();
}

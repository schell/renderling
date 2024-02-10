//! A web application for exploring and authoring SDFs.
use crabslab::{GrowableSlab, Id};
use glam::{Mat4, Vec2, Vec3};
use renderling::{
    sdf::{
        raymarch::{renderer::RaymarchingRenderer, *},
        Scene, Sdf,
    },
    AtlasImage, Camera, Renderling, Skybox,
};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoopWindowTarget;

pub mod wgsl;

fn create_raymarch(r: &mut RaymarchingRenderer) -> Id<Raymarch> {
    let (width, height) = r.renderling.get_screen_size();
    let width = width as f32;
    let height = height as f32;
    let projection = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, width / height, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 3.0, 5.0), Vec3::ZERO, Vec3::Y);
    let camera = Camera::new(projection, view);
    let camera_id = r.slab.append(&camera);
    let (device, queue) = r.renderling.get_device_and_queue_owned();
    // let hdr = AtlasImage::from_hdr_path("../../img/hdr/helipad.hdr")
    //     .unwrap_or_else(|e| panic!("could not load hdr: {e}\n{:?}", std::env::current_dir()));
    // let skybox = Skybox::new(device, queue, hdr, camera_id);
    // r.set_skybox(skybox);

    let mut sdfs = vec![];

    let sdf = Sdf {
        distance: r.slab.append_array(&sdf_sphere(input_position(), 1.0)),
        albedo: r.slab.append_array(&constant(Vec3::new(1.0, 1.0, 0.0))),
    };
    sdfs.push(sdf);

    let sdf = Sdf {
        distance: r
            .slab
            .append_array(&sdf_plane(input_position(), Vec3::Y, 0.0)),
        albedo: r.slab.append_array(&constant(Vec3::new(0.0, 1.0, 1.0))),
    };
    sdfs.push(sdf);

    let sdfs = r.slab.append_array(&sdfs);

    let scene = r.slab.append(&Scene {
        camera: camera_id,
        sdfs,
        ..Default::default()
    });
    let raymarch = r.slab.append(&Raymarch {
        scene,
        screen_resolution: Vec2::new(width, height),
        ..Default::default()
    });
    raymarch
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        //.level_for("renderling", log::LevelFilter::Trace)
        .level_for("wgpu", log::LevelFilter::Warn)
        .level_for("naga", log::LevelFilter::Warn)
        .chain(fern::Output::call(console_log::log))
        .apply()
        .unwrap();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling sdfshop");
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        body.append_child(&canvas).unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder.build(&event_loop).unwrap();
    let window = Arc::new(window);
    log::info!("creating renderling");
    let r = Renderling::from_window_async(window.clone()).await;
    let mut r = renderling::sdf::raymarch::renderer::RaymarchingRenderer::from_renderling(r, 256);
    let raymarch = create_raymarch(&mut r);

    log::info!("...created renderling");

    #[cfg(target_arch = "wasm32")]
    let event_loop_fn = {
        use winit::platform::web::EventLoopExtWebSys;
        winit::event_loop::EventLoop::spawn
    };
    #[cfg(not(target_arch = "wasm32"))]
    let event_loop_fn = winit::event_loop::EventLoop::run;

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
                        log::info!("frame");
                        // redraw
                        r.render(raymarch);
                        window.request_redraw();
                    }
                    _ => {
                        // do the other stuff
                    }
                },
                _ => {}
            }
        },
    );
    #[cfg(not(target_arch = "wasm32"))]
    _result.unwrap();
}

use example::gltf;
use renderling::Renderling;
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window_size = winit::dpi::LogicalSize {
        width: 800,
        height: 600,
    };
    let window = winit::window::WindowBuilder::new()
        .with_inner_size::<winit::dpi::LogicalSize<u32>>(window_size)
        .with_title("renderling gltf viewer")
        .build(&event_loop)
        .unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        let dom_window = web_sys::window().unwrap();
        let dom_doc = dom_window.document().unwrap();
        let dom_body = dom_doc.body().unwrap();
        let dom_canvas = web_sys::Element::from(window.canvas());
        dom_body.append_child(&dom_canvas).unwrap();
    }
    let mut r = Renderling::from_window_async(&window).await;
    let mut run_current_frame: Box<dyn FnMut(&mut Renderling, Option<&winit::event::WindowEvent>)> =
        Box::new(
            gltf::demo(
                &mut r,
                Some("gltf/DamagedHelmet.glb"),
                Some("img/hdr/resting_place.hdr"),
            )
            .await,
        );

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

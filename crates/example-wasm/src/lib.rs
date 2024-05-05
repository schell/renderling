use futures_lite::StreamExt;
use renderling::Context;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod event;

fn surface_from_canvas(_canvas: HtmlCanvasElement) -> Option<wgpu::SurfaceTarget<'static>> {
    #[cfg(target_arch = "wasm32")]
    {
        Some(wgpu::SurfaceTarget::Canvas(_canvas))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .level_for("wgpu", log::LevelFilter::Error)
        .level_for("naga", log::LevelFilter::Error)
        .level_for("renderling", log::LevelFilter::Debug)
        .chain(fern::Output::call(console_log::log))
        .apply()
        .unwrap();

    log::info!("Starting example-wasm");

    let dom_window = web_sys::window().unwrap();
    let ww = dom_window.inner_width().unwrap().as_f64().unwrap() as u32;
    let wh = dom_window.inner_height().unwrap().as_f64().unwrap() as u32;
    let dom_doc = dom_window.document().unwrap();

    let viewport_canvas = dom_doc
        .query_selector("main canvas")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    viewport_canvas.set_width(ww);
    viewport_canvas.set_height(wh);

    let surface = surface_from_canvas(viewport_canvas.clone()).unwrap();
    let ctx = Context::try_from_raw_window(ww, wh, surface).await.unwrap();
    let mut app = example::App::new(&ctx);

    let window_resize = event::event_stream("resize", &dom_window);
    let mut all_events = window_resize;

    while let Some(event) = all_events.next().await {
        log::trace!("event: {event:#?}");
        let frame = ctx.get_current_frame().unwrap();
        app.stage.render(&frame.view());
        frame.present();
    }
}

use futures_lite::StreamExt;
use glam::{Vec2, Vec3, Vec4};
use renderling::{prelude::*, ui::prelude::*};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod event;
mod req_animation_frame;

const HDR_IMAGE_BYTES: &[u8] = include_bytes!("../../../img/hdr/helipad.hdr");
const GLTF_FOX_BYTES: &[u8] = include_bytes!("../../../gltf/Fox.glb");

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

pub struct App {
    ctx: Context,
    ui: Ui,
    path: UiPath,
    stage: Stage,
    doc: GltfDocument,
    camera: Hybrid<Camera>,
    text: UiText,
}

impl App {
    fn tick(&self) {
        let frame = self.ctx.get_next_frame().unwrap();
        self.ui.render(&frame.view());
        self.stage.render(&frame.view());
        frame.present();
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("wgpu", log::LevelFilter::Warn)
        .level_for("naga", log::LevelFilter::Trace)
        .level_for("renderling::draw", log::LevelFilter::Trace)
        .chain(fern::Output::call(console_log::log))
        .apply()
        .unwrap();

    log::info!("Starting example-wasm");

    let dom_window = web_sys::window().unwrap();
    let dom_doc = dom_window.document().unwrap();

    let canvas = dom_doc
        .query_selector("main canvas")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(800);
    canvas.set_height(600);

    let surface = surface_from_canvas(canvas.clone()).unwrap();
    let ctx = Context::try_new_with_surface(800, 600, None, surface)
        .await
        .unwrap();

    let ui = ctx.new_ui();
    let path = ui
        .new_path()
        .with_circle(Vec2::splat(100.0), 20.0)
        .with_fill_color(Vec4::new(1.0, 1.0, 0.0, 1.0))
        .fill();
    let _ = ui
        .load_font("Recursive Mn Lnr St Med Nerd Font Complete.ttf")
        .await
        .expect_throw("Could not load font");
    let text = ui
        .new_text()
        .with_color(
            // white
            Vec4::ONE,
        )
        .with_section(Section::default().add_text(Text::new("WASM example").with_scale(24.0)))
        .build();

    let stage = ctx
        .new_stage()
        .with_background_color(
            // black
            // Vec3::ZERO.extend(1.0),
            Vec4::new(1.0, 0.0, 0.0, 1.0),
        )
        .with_lighting(false);

    let skybox = stage.new_skybox_from_bytes(HDR_IMAGE_BYTES).unwrap();
    stage.set_skybox(skybox);

    let fox = stage.load_gltf_document_from_bytes(GLTF_FOX_BYTES).unwrap();
    log::info!("fox aabb: {:?}", fox.bounding_volume());

    let camera = stage.new_camera(Camera::default_perspective(800.0, 600.0));

    let app = App {
        ctx,
        ui,
        path,
        stage,
        doc: fox,
        camera,
        text,
    };
    app.tick();

    loop {
        let _ = req_animation_frame::next_animation_frame().await;
        app.tick();
    }
}

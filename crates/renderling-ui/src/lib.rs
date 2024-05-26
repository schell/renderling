//! This is an integration of the `renderling` renderer, the `lyon`
//! path tesselator and the `glyph_brush` text rendering libraries.
use std::collections::HashMap;

use renderling::{
    camera::Camera,
    math::{UVec2, Vec4},
    slab::Hybrid,
    stage::Stage,
    Context,
};

mod path;
pub use path::*;

mod text;
pub use text::*;

pub struct Ui {
    camera: Hybrid<Camera>,
    stage: Stage,
    fonts: HashMap<usize, FontArc>,
}

impl Ui {
    pub fn new(ctx: &Context) -> Self {
        let UVec2 { x, y } = ctx.get_size();
        let mut stage = ctx
            .new_stage()
            .with_background_color(Vec4::ONE)
            .with_lighting(false)
            .with_bloom(false);
        let camera = stage.new_value(Camera::default_ortho2d(x as f32, y as f32));
        Ui {
            camera,
            stage,
            fonts: Default::default(),
        }
    }

    pub fn new_path(&self) -> UiPathBuilder {
        UiPathBuilder::new(&self.stage, self.camera.id())
    }

    pub fn new_text(&self) -> UiTextBuilder {
        UiTextBuilder::new(&self.stage, self.camera.id())
    }

    pub fn render(&mut self, view: &wgpu::TextureView) {
        self.stage.render(view);
    }
}

#[cfg(test)]
mod test {
    use renderling::math::Vec2;

    use super::*;

    #[ctor::ctor]
    fn init_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Warn)
            .filter_module("renderling", log::LevelFilter::Info)
            .filter_module("crabslab", log::LevelFilter::Debug)
            .try_init();
    }

    #[test]
    fn can_build_path_sanity() {
        let ctx = Context::headless(100, 100);
        let mut ui = Ui::new(&ctx);
        let _path = ui
            .new_path()
            .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
            .build();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::save("renderling-ui/path_sanity.png", img);
    }
}

//! `renderling-ui` is a "GPU driven" 2d renderer with a focus on simplicity and ease of use.
//!
//! This library is meant to be used with its parent [`renderling`].
//!
//! # Getting Started
//! First we create a context, then we create a [`Ui`], which we can use to "stage" our
//! paths, text, etc:
//!
//! ```rust
//! use renderling::{Context, math::Vec2};
//! use renderling_ui::Ui;
//!
//! let ctx = Context::headless(100, 100);
//! let mut ui = Ui::new(&ctx);

//! let _path = ui
//!     .new_path()
//!     .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
//!     .build();
//!
//! let frame = ctx.get_next_frame().unwrap();
//! ui.render(&frame.view());
//! frame.present();
//! ```
//!
//! Happy hacking!
use std::sync::{Arc, RwLock};

use crabslab::Id;
use renderling::{
    camera::Camera,
    math::{Quat, UVec2, Vec2, Vec3Swizzles, Vec4},
    slab::{Hybrid, UpdatesSlab},
    stage::{NestedTransform, Renderlet, Stage},
    transform::Transform,
    Context,
};
use rustc_hash::FxHashMap;

mod path;
pub use path::*;

mod text;
pub use text::*;

/// A two dimensional transformation.
///
/// Clones of `UiTransform` all point to the same data.
#[derive(Clone)]
pub struct UiTransform {
    transform: NestedTransform,
    renderlet_ids: Arc<Vec<Id<Renderlet>>>,
}

impl UiTransform {
    pub(crate) fn id(&self) -> Id<Transform> {
        self.transform.global_transform_id()
    }

    pub fn set_translation(&self, t: Vec2) {
        self.transform.modify(|a| {
            a.translation.x = t.x;
            a.translation.y = t.y;
        });
    }

    pub fn get_translation(&self) -> Vec2 {
        let t = self.transform.get();
        t.translation.xy()
    }

    pub fn set_rotation(&self, radians: f32) {
        let rotation = Quat::from_rotation_z(radians);
        self.transform.modify(|t| {
            t.rotation *= rotation;
        });
    }

    pub fn get_rotation(&self) -> f32 {
        self.transform
            .get()
            .rotation
            .to_euler(renderling::math::EulerRot::XYZ)
            .2
    }

    pub fn set_z(&self, z: f32) {
        self.transform.modify(|t| {
            t.translation.z = z;
        });
    }

    pub fn get_z(&self) -> f32 {
        self.transform.get().translation.z
    }
}

/// A 2d user interface renderer.
///
/// Clones of `Ui` all point to the same data.
#[derive(Clone)]
pub struct Ui {
    camera: Hybrid<Camera>,
    stage: Stage,
    fonts: Arc<RwLock<Vec<FontArc>>>,
    // We keep a list of transforms that we use to "manually" order renderlets.
    //
    // This is required because interface elements have transparency.
    transforms: Arc<RwLock<FxHashMap<usize, UiTransform>>>,
}

impl Ui {
    pub fn new(ctx: &Context) -> Self {
        let UVec2 { x, y } = ctx.get_size();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::ONE)
            .with_lighting(false)
            .with_bloom(false);
        let camera = stage.new_value(Camera::default_ortho2d(x as f32, y as f32));
        Ui {
            camera,
            stage,
            fonts: Default::default(),
            transforms: Default::default(),
        }
    }

    fn new_transform(&self, renderlet_ids: Vec<Id<Renderlet>>) -> UiTransform {
        let transform = self.stage.new_nested_transform();
        let transform = UiTransform {
            transform,
            renderlet_ids: Arc::new(renderlet_ids),
        };
        self.transforms
            .write()
            .unwrap()
            .insert(transform.transform.id(), transform.clone());
        transform
    }

    pub fn new_path(&self) -> UiPathBuilder {
        UiPathBuilder::new(self)
    }

    pub fn new_text(&self) -> UiTextBuilder {
        UiTextBuilder::new(self)
    }

    pub fn add_font(&mut self, font: FontArc) -> usize {
        // UNWRAP: panic on purpose
        let mut fonts = self.fonts.write().unwrap();
        let id = fonts.len();
        fonts.push(font);
        id
    }

    pub fn get_fonts(&self) -> Vec<FontArc> {
        // UNWRAP: panic on purpose
        self.fonts.read().unwrap().clone()
    }

    fn reorder_renderlets(&self) {
        // UNWRAP: panic on purpose
        let guard = self.transforms.read().unwrap();
        let mut transforms = guard.values().collect::<Vec<_>>();
        transforms.sort_by(|a, b| {
            let ta = a.transform.get_global_transform();
            let tb = b.transform.get_global_transform();
            ta.translation.z.total_cmp(&tb.translation.z)
        });
        self.stage.reorder_renderlets(
            transforms
                .iter()
                .flat_map(|t| t.renderlet_ids.as_ref().clone()),
        );
    }

    pub fn render(&mut self, view: &wgpu::TextureView) {
        let mut should_reorder = false;
        // UNWRAP: panic on purpose
        let mut transforms = self.transforms.write().unwrap();
        for update_id in self.stage.get_updated_source_ids().into_iter() {
            if let Some(ui_transform) = transforms.get(&update_id) {
                if Arc::strong_count(&ui_transform.renderlet_ids) == 1 {
                    let _ = transforms.remove(&update_id);
                } else {
                    should_reorder = true;
                }
            }
        }
        drop(transforms);
        if should_reorder {
            log::trace!("a ui transform changed, sorting the renderlets");
            self.reorder_renderlets();
        }
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
            .filter_module("renderling_ui", log::LevelFilter::Trace)
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
        img_diff::save("ui/path/sanity.png", img);
    }
}

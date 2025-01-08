//! `renderling-ui` is a "GPU driven" 2d renderer with a focus on simplicity and
//! ease of use.
//!
//! This library is meant to be used with its parent [`renderling`].
//!
//! # Getting Started
//! First we create a context, then we create a [`Ui`], which we can use to
//! "stage" our paths, text, etc:
//!
//! ```rust
//! use renderling::{math::Vec2, Context};
//! use renderling_ui::Ui;
//!
//! let ctx = Context::headless(100, 100);
//! let mut ui = Ui::new(&ctx);
//!
//! let _path = ui
//!     .new_path()
//!     .with_stroke_color([1.0, 1.0, 0.0, 1.0])
//!     .with_rectangle(Vec2::splat(10.0), Vec2::splat(60.0))
//!     .stroke();
//!
//! let frame = ctx.get_next_frame().unwrap();
//! ui.render(&frame.view());
//! frame.present();
//! ```
//!
//! Happy hacking!
use std::sync::{Arc, RwLock};

use craballoc::prelude::Hybrid;
use crabslab::Id;
use glyph_brush::ab_glyph;
use renderling::{
    atlas::AtlasTexture,
    camera::Camera,
    math::{Quat, UVec2, Vec2, Vec3Swizzles, Vec4},
    stage::{NestedTransform, Renderlet, Stage},
    transform::Transform,
    Context,
};
use rustc_hash::FxHashMap;
use snafu::{prelude::*, ResultExt};

pub use glyph_brush::FontId;

mod path;
pub use path::*;

mod text;
pub use text::*;

#[derive(Debug, Snafu)]
pub enum UiError {
    #[snafu(display("{source}"))]
    Loading {
        source: loading_bytes::LoadingBytesError,
    },

    #[snafu(display("{source}"))]
    InvalidFont { source: ab_glyph::InvalidFont },

    #[snafu(display("{source}"))]
    Image { source: image::ImageError },

    #[snafu(display("{source}"))]
    Stage {
        source: renderling::stage::StageError,
    },
}

/// An image identifier.
///
/// This locates the image within a [`Ui`].
///
/// `ImageId` can be created with [`Ui::load_image`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ImageId(usize);

/// A two dimensional transformation.
///
/// Clones of `UiTransform` all point to the same data.
#[derive(Clone, Debug)]
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

#[derive(Clone)]
#[repr(transparent)]
struct UiImage(Hybrid<AtlasTexture>);

/// A 2d user interface renderer.
///
/// Clones of `Ui` all point to the same data.
#[derive(Clone)]
pub struct Ui {
    camera: Hybrid<Camera>,
    stage: Stage,
    images: Arc<RwLock<Vec<UiImage>>>,
    fonts: Arc<RwLock<Vec<FontArc>>>,
    // We keep a list of transforms that we use to "manually" order renderlets.
    //
    // This is required because interface elements have transparency.
    //
    // The `usize` key here is the update source notifier index, which is needed
    // to re-order after any transform performs an update.
    transforms: Arc<RwLock<FxHashMap<usize, UiTransform>>>,
    default_stroke_options: Arc<RwLock<StrokeOptions>>,
    default_fill_options: Arc<RwLock<FillOptions>>,
}

impl Ui {
    pub fn new(ctx: &Context) -> Self {
        let UVec2 { x, y } = ctx.get_size();
        let stage = ctx
            .new_stage()
            .with_background_color(Vec4::ONE)
            .with_lighting(false)
            .with_bloom(false)
            .with_msaa_sample_count(4);
        let camera = stage.new_value(Camera::default_ortho2d(x as f32, y as f32));
        Ui {
            camera,
            stage,
            images: Default::default(),
            fonts: Default::default(),
            transforms: Default::default(),
            default_stroke_options: Default::default(),
            default_fill_options: Default::default(),
        }
    }

    pub fn set_clear_color_attachments(&self, should_clear: bool) {
        self.stage.set_clear_color_attachments(should_clear);
    }

    pub fn with_clear_color_attachments(self, should_clear: bool) -> Self {
        self.set_clear_color_attachments(should_clear);
        self
    }

    pub fn set_clear_depth_attachments(&self, should_clear: bool) {
        self.stage.set_clear_depth_attachments(should_clear);
    }

    pub fn with_clear_depth_attachments(self, should_clear: bool) -> Self {
        self.set_clear_depth_attachments(should_clear);
        self
    }

    pub fn set_background_color(&self, color: impl Into<Vec4>) -> &Self {
        self.stage.set_background_color(color);
        self
    }

    pub fn with_background_color(self, color: impl Into<Vec4>) -> Self {
        self.set_background_color(color);
        self
    }

    pub fn set_antialiasing(&self, antialiasing_is_on: bool) -> &Self {
        let sample_count = if antialiasing_is_on { 4 } else { 1 };
        self.stage.set_msaa_sample_count(sample_count);
        self
    }

    pub fn with_antialiasing(self, antialiasing_is_on: bool) -> Self {
        self.set_antialiasing(antialiasing_is_on);
        self
    }

    pub fn set_default_stroke_options(&self, options: StrokeOptions) -> &Self {
        *self.default_stroke_options.write().unwrap() = options;
        self
    }

    pub fn with_default_stroke_options(self, options: StrokeOptions) -> Self {
        self.set_default_stroke_options(options);
        self
    }

    pub fn set_default_fill_options(&self, options: FillOptions) -> &Self {
        *self.default_fill_options.write().unwrap() = options;
        self
    }

    pub fn with_default_fill_options(self, options: FillOptions) -> Self {
        self.set_default_fill_options(options);
        self
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
            .insert(transform.transform.get_notifier_index(), transform.clone());
        transform
    }

    pub fn new_path(&self) -> UiPathBuilder {
        UiPathBuilder::new(self)
    }

    pub fn new_text(&self) -> UiTextBuilder {
        UiTextBuilder::new(self)
    }

    pub async fn load_font(&self, path: impl AsRef<str>) -> Result<FontId, UiError> {
        let path_s = path.as_ref();
        let bytes = loading_bytes::load(path_s).await.context(LoadingSnafu)?;
        let font = FontArc::try_from_vec(bytes).context(InvalidFontSnafu)?;
        Ok(self.add_font(font))
    }

    pub fn add_font(&self, font: FontArc) -> FontId {
        // UNWRAP: panic on purpose
        let mut fonts = self.fonts.write().unwrap();
        let id = fonts.len();
        fonts.push(font);
        FontId(id)
    }

    pub fn get_fonts(&self) -> Vec<FontArc> {
        // UNWRAP: panic on purpose
        self.fonts.read().unwrap().clone()
    }

    pub fn get_camera(&self) -> &Hybrid<Camera> {
        &self.camera
    }

    pub async fn load_image(&self, path: impl AsRef<str>) -> Result<ImageId, UiError> {
        let path_s = path.as_ref();
        let bytes = loading_bytes::load(path_s).await.context(LoadingSnafu)?;
        let img = image::load_from_memory_with_format(
            bytes.as_slice(),
            image::ImageFormat::from_path(path_s).context(ImageSnafu)?,
        )
        .context(ImageSnafu)?;
        let entry = self
            .stage
            .add_images(Some(img))
            .context(StageSnafu)?
            .pop()
            .unwrap();
        entry.modify(|t| {
            t.modes.s = renderling::atlas::TextureAddressMode::Repeat;
            t.modes.t = renderling::atlas::TextureAddressMode::Repeat;
        });
        let mut guard = self.images.write().unwrap();
        let id = guard.len();
        guard.push(UiImage(entry));
        Ok(ImageId(id))
    }

    pub(crate) fn get_image(&self, index: usize) -> Option<UiImage> {
        self.images.read().unwrap().get(index).cloned()
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

    pub fn render(&self, view: &wgpu::TextureView) {
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
        self.stage.tick();
        self.stage.render(view);
    }
}

#[cfg(test)]
mod test {
    use renderling::{color::rgb_hex_color, math::Vec4};

    #[ctor::ctor]
    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    pub struct Colors<const N: usize>(std::iter::Cycle<std::array::IntoIter<Vec4, N>>);

    pub fn cute_beach_palette() -> [Vec4; 4] {
        [
            rgb_hex_color(0x6DC5D1),
            rgb_hex_color(0xFDE49E),
            rgb_hex_color(0xFEB941),
            rgb_hex_color(0xDD761C),
        ]
    }

    impl<const N: usize> Colors<N> {
        pub fn from_array(colors: [Vec4; N]) -> Self {
            Colors(colors.into_iter().cycle())
        }

        pub fn next_color(&mut self) -> Vec4 {
            self.0.next().unwrap()
        }
    }
}

//! CPU part of ui.

use core::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

use crate::{
    atlas::{shader::AtlasTextureDescriptor, AtlasTexture, TextureAddressMode, TextureModes},
    camera::Camera,
    stage::Stage,
    transform::NestedTransform,
    Context,
};
use crabslab::Id;
use glam::{Quat, UVec2, Vec2, Vec3Swizzles, Vec4};
use glyph_brush::ab_glyph;
use rustc_hash::FxHashMap;
use snafu::{prelude::*, ResultExt};

pub use glyph_brush::FontId;

mod path;
pub use path::*;

mod text;
pub use text::*;

pub mod prelude {
    //! A prelude for user interface development, meant to be glob-imported.

    #[cfg(cpu)]
    pub extern crate craballoc;
    pub extern crate glam;

    #[cfg(cpu)]
    pub use craballoc::prelude::*;
    pub use crabslab::{Array, Id};

    #[cfg(cpu)]
    pub use crate::context::*;

    pub use super::*;
}

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
    Stage { source: crate::stage::StageError },
}

/// An image identifier.
///
/// This locates the image within a [`Ui`].
///
/// `ImageId` can be created with [`Ui::load_image`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ImageId(Id<AtlasTextureDescriptor>);

/// A two dimensional transformation.
///
/// Clones of `UiTransform` all point to the same data.
#[derive(Clone, Debug)]
pub struct UiTransform {
    should_reorder: Arc<AtomicBool>,
    transform: NestedTransform,
}

impl UiTransform {
    fn mark_should_reorder(&self) {
        self.should_reorder
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_translation(&self, t: Vec2) {
        self.mark_should_reorder();
        self.transform.modify_local_translation(|a| {
            a.x = t.x;
            a.y = t.y;
        });
    }

    pub fn get_translation(&self) -> Vec2 {
        self.transform.local_translation().xy()
    }

    pub fn set_rotation(&self, radians: f32) {
        self.mark_should_reorder();
        let rotation = Quat::from_rotation_z(radians);
        // TODO: check to see if *= rotation makes sense here
        self.transform.modify_local_rotation(|t| {
            *t *= rotation;
        });
    }

    pub fn get_rotation(&self) -> f32 {
        self.transform
            .local_rotation()
            .to_euler(glam::EulerRot::XYZ)
            .2
    }

    pub fn set_z(&self, z: f32) {
        self.mark_should_reorder();
        self.transform.modify_local_translation(|t| {
            t.z = z;
        });
    }

    pub fn get_z(&self) -> f32 {
        self.transform.local_translation().z
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct UiImage(AtlasTexture);

/// A 2d user interface renderer.
///
/// Clones of `Ui` all point to the same data.
#[derive(Clone)]
pub struct Ui {
    camera: Camera,
    stage: Stage,
    should_reorder: Arc<AtomicBool>,
    images: Arc<RwLock<FxHashMap<Id<AtlasTextureDescriptor>, UiImage>>>,
    fonts: Arc<RwLock<Vec<FontArc>>>,
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
            .with_msaa_sample_count(4)
            .with_frustum_culling(false);
        let (proj, view) = crate::camera::default_ortho2d(x as f32, y as f32);
        let camera = stage.new_camera().with_projection_and_view(proj, view);
        Ui {
            camera,
            stage,
            should_reorder: AtomicBool::new(true).into(),
            images: Default::default(),
            fonts: Default::default(),
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

    fn new_transform(&self) -> UiTransform {
        self.mark_should_reorder();
        let transform = self.stage.new_nested_transform();
        UiTransform {
            transform,
            should_reorder: self.should_reorder.clone(),
        }
    }

    fn mark_should_reorder(&self) {
        self.should_reorder
            .store(true, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn path_builder(&self) -> UiPathBuilder {
        self.mark_should_reorder();
        UiPathBuilder::new(self)
    }

    /// Remove the `path` from the [`Ui`].
    ///
    /// The given `path` must have been created with this [`Ui`], otherwise this function is
    /// a noop.
    pub fn remove_path(&self, path: &UiPath) {
        self.stage.remove_primitive(&path.primitive);
    }

    pub fn text_builder(&self) -> UiTextBuilder {
        self.mark_should_reorder();
        UiTextBuilder::new(self)
    }

    /// Remove the text from the [`Ui`].
    ///
    /// The given `text` must have been created with this [`Ui`], otherwise this function is
    /// a noop.
    pub fn remove_text(&self, text: &UiText) {
        self.stage.remove_primitive(&text.renderlet);
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

    pub fn get_camera(&self) -> &Camera {
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
        entry.set_modes(TextureModes {
            s: TextureAddressMode::Repeat,
            t: TextureAddressMode::Repeat,
        });
        let mut guard = self.images.write().unwrap();
        let id = entry.id();
        guard.insert(id, UiImage(entry));
        Ok(ImageId(id))
    }

    /// Remove an image previously loaded with [`Ui::load_image`].
    pub fn remove_image(&self, image_id: &ImageId) -> Option<UiImage> {
        self.images.write().unwrap().remove(&image_id.0)
    }

    fn reorder_renderlets(&self) {
        self.stage.sort_renderlets(|a, b| {
            let za = a
                .transform()
                .as_ref()
                .map(|t| t.translation().z)
                .unwrap_or_default();
            let zb = b
                .transform()
                .as_ref()
                .map(|t| t.translation().z)
                .unwrap_or_default();
            za.total_cmp(&zb)
        });
    }

    pub fn render(&self, view: &wgpu::TextureView) {
        if self
            .should_reorder
            .swap(false, std::sync::atomic::Ordering::Relaxed)
        {
            self.reorder_renderlets();
        }
        self.stage.render(view);
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{color::rgb_hex_color, prelude::glam::Vec4};

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

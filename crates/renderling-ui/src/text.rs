//! Text rendering capabilities for `Renderling`.
//!
//! This module is only enabled with the `text` cargo feature.
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ab_glyph::Rect;
use crabslab::Id;
use glyph_brush::*;

pub use ab_glyph::FontArc;
pub use glyph_brush::{Color, FontId, GlyphCruncher, OwnedSection, OwnedText, Section, Text};

use image::{GenericImage, ImageBuffer, Luma};
use renderling::{
    camera::Camera,
    stage::{Stage, Vertex},
};

pub struct UiText {}

pub struct UiTextBuilder {
    stage: Stage,
    camera_id: Id<Camera>,
    fonts: HashMap<usize, FontArc>,
}

impl UiTextBuilder {
    pub fn new(ui: &super::Ui) -> Self {
        Self {
            stage: ui.stage.clone(),
            camera_id: ui.camera.id(),
            fonts: ui.fonts.clone(),
        }
    }
}

/// A text cache maintained mostly by ab_glyph.
pub struct Cache {
    img: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
    dirty: bool,
}

impl Cache {
    pub fn new(width: u32, height: u32) -> Cache {
        Cache {
            img: image::ImageBuffer::from_pixel(width, height, image::Luma([0])),
            dirty: false,
        }
    }

    pub fn update(&mut self, offset: [u16; 2], size: [u16; 2], data: &[u8]) {
        let width = size[0] as u32;
        let height = size[1] as u32;
        let x = offset[0] as u32;
        let y = offset[1] as u32;

        // UNWRAP: panic on purpose
        let source =
            image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_vec(width, height, data.to_vec())
                .unwrap();
        self.img.copy_from(&source, x, y).unwrap();
        self.dirty = true;
    }
}

/// A cache of glyphs.
///
/// Text is required to come from a cache. Creation is easy and only requires
/// a vector of the fonts to be used in sections.
///
/// ``` ignore
/// let font = fs::load_font("my_font.ttf").await?;
/// let mut cache = GlyphCache::new(vec![font]);
/// cache.brush.queue(
///     Section::default()
///         .add_text(
///             Text::new("Here is some text.\n")
///                 .with_scale(64.0)
///                 .with_color(Color::rgb(0x00, 0x00, 0x00)),
///         )
///         .add_text(
///             Text::new("Here is text in a new color\n")
///                 .with_scale(64.0)
///                 .with_color(Color::rgb(255, 255, 0)),
///         )
///         .add_text(
///             Text::new("(and variable size)\n")
///                 .with_scale(32.0)
///                 .with_color(Color::rgb(255, 0, 255))
///         )
///         .add_text(
///             Text::new("...and variable transparency\n...and word wrap")
///                 .with_scale(64.0)
///                 .with_color(Color::rgba(0x33, 0x33, 0x33, 127)),
///         )
/// );
/// ```
///
/// Once you have a `GlyphCache` that has text [`queue`](GlyphBrush::queue)d you
/// can use [`TextData`] to create a builder, which can be used to position and
/// scale the text entity and add other components.
pub struct GlyphCache {
    /// Image on the CPU or GPU used as our texture cache
    cache: Option<Cache>,
    brush: GlyphBrush<Vec<Vertex>>,
}

impl Deref for GlyphCache {
    type Target = GlyphBrush<Vec<Vertex>>;

    fn deref(&self) -> &Self::Target {
        &self.brush
    }
}

impl DerefMut for GlyphCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.brush
    }
}

#[inline]
fn to_vertex(
    glyph_brush::GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: glyph_brush::GlyphVertex,
) -> Vec<Vertex> {
    let gl_bounds = bounds;

    let mut gl_rect = Rect {
        min: ab_glyph::point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
        max: ab_glyph::point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
    };

    // handle overlapping bounds, modify uv_rect to preserve texture aspect
    if gl_rect.max.x > gl_bounds.max.x {
        let old_width = gl_rect.width();
        gl_rect.max.x = gl_bounds.max.x;
        tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.min.x < gl_bounds.min.x {
        let old_width = gl_rect.width();
        gl_rect.min.x = gl_bounds.min.x;
        tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.max.y > gl_bounds.max.y {
        let old_height = gl_rect.height();
        gl_rect.max.y = gl_bounds.max.y;
        tex_coords.max.y = tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
    }
    if gl_rect.min.y < gl_bounds.min.y {
        let old_height = gl_rect.height();
        gl_rect.min.y = gl_bounds.min.y;
        tex_coords.min.y = tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
    }
    let tl = Vertex::default()
        .with_position([gl_rect.min.x, gl_rect.min.y, 0.0])
        .with_uv0([tex_coords.min.x, tex_coords.min.y])
        .with_color(extra.color);
    let tr = Vertex::default()
        .with_position([gl_rect.max.x, gl_rect.min.y, 0.0])
        .with_uv0([tex_coords.max.x, tex_coords.min.y])
        .with_color(extra.color);
    let br = Vertex::default()
        .with_position([gl_rect.max.x, gl_rect.max.y, 0.0])
        .with_uv0([tex_coords.max.x, tex_coords.max.y])
        .with_color(extra.color);
    let bl = Vertex::default()
        .with_position([gl_rect.min.x, gl_rect.max.y, 0.0])
        .with_uv0([tex_coords.min.x, tex_coords.max.y])
        .with_color(extra.color);

    // Draw as two tris
    let data = vec![tl, br, tr, tl, bl, br];
    data
}

impl GlyphCache {
    pub fn new(fonts: Vec<FontArc>) -> Self {
        let brush = GlyphBrushBuilder::using_fonts(fonts).build();
        GlyphCache { cache: None, brush }
    }

    pub fn bounds<'a, S>(&mut self, section: S) -> Option<ab_glyph::Rect>
    where
        S: Into<Cow<'a, Section<'a, Extra>>>,
    {
        self.brush.glyph_bounds(section)
    }

    /// Process any brushes, updating textures, etc.
    ///
    /// Returns a new mesh if the mesh needs to be updated.
    /// Returns a new texture if the texture needs to be updated.
    ///
    /// The texture and mesh are meant to be used to build or update a `Renderlet`
    /// to display.
    pub fn get_updated(&mut self) -> (Option<Vec<Vertex>>, Option<ImageBuffer<Luma<u8>, Vec<u8>>>) {
        let mut may_mesh: Option<Vec<Vertex>> = None;
        let mut cache = self.cache.take().unwrap_or_else(|| {
            let (width, height) = self.brush.texture_dimensions();
            Cache::new(width, height)
        });

        let mut brush_action;
        loop {
            brush_action = self.brush.process_queued(
                |rect, tex_data| {
                    let offset = [rect.min[0] as u16, rect.min[1] as u16];
                    let size = [rect.width() as u16, rect.height() as u16];
                    cache.update(offset, size, tex_data)
                },
                to_vertex,
            );

            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let max_image_dimension = 2048;

                    let (new_width, new_height) = if (suggested.0 > max_image_dimension
                        || suggested.1 > max_image_dimension)
                        && (self.brush.texture_dimensions().0 < max_image_dimension
                            || self.brush.texture_dimensions().1 < max_image_dimension)
                    {
                        (max_image_dimension, max_image_dimension)
                    } else {
                        suggested
                    };

                    log::warn!(
                        "Increasing glyph texture size {old:?} -> {new:?}. Consider building with \
                         `.initial_cache_size({new:?})` to avoid resizing",
                        old = self.brush.texture_dimensions(),
                        new = (new_width, new_height),
                    );

                    cache = Cache::new(new_width, new_height);
                    self.brush.resize_texture(new_width, new_height);
                }
            }
        }

        match brush_action.unwrap() {
            BrushAction::Draw(all_vertices) => {
                if !all_vertices.is_empty() {
                    may_mesh = Some(
                        all_vertices
                            .into_iter()
                            .flat_map(|vs| vs.into_iter())
                            .collect(),
                    );
                }
            }
            BrushAction::ReDraw => {}
        }
        let may_texture = if cache.dirty {
            Some(cache.img.clone())
        } else {
            None
        };
        self.cache = Some(cache);

        (may_mesh, may_texture)
    }
}

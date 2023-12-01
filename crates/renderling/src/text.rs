//! Text rendering capabilities for `Renderling`.
//!
//! This module is only enabled with the `text` cargo feature.
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use ::ab_glyph::Rect;
use glyph_brush::*;

pub use ::ab_glyph::FontArc;
pub use glyph_brush::{Color, FontId, GlyphCruncher, OwnedSection, OwnedText, Section, Text};

use crate::{Texture, UiVertex};

/// A text cache maintained mostly by ab_glyph.
pub struct Cache {
    pub(crate) texture: Texture,
}

impl Cache {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Cache {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("systems::text::cache::Cache"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        });

        let texture = Texture::from_wgpu_tex(
            device,
            texture,
            Some(wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }),
            None,
        );

        Cache { texture }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, offset: [u16; 2], size: [u16; 2], data: &[u8]) {
        let width = size[0] as usize;
        let height = size[1] as usize;

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: u32::from(offset[0]),
                    y: u32::from(offset[1]),
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width as u32),
                rows_per_image: Some(height as u32),
            },
            wgpu::Extent3d {
                width: size[0] as u32,
                height: size[1] as u32,
                depth_or_array_layers: 1,
            },
        );
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
    pub(crate) cache: Option<Cache>,
    pub brush: GlyphBrush<Vec<UiVertex>>,
}

impl Deref for GlyphCache {
    type Target = GlyphBrush<Vec<UiVertex>>;

    fn deref(&self) -> &Self::Target {
        &self.brush
    }
}

impl DerefMut for GlyphCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.brush
    }
}

impl GlyphCache {}

#[inline]
fn to_vertex(
    glyph_brush::GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: glyph_brush::GlyphVertex,
) -> Vec<UiVertex> {
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
    let tl = UiVertex::default()
        .with_position([gl_rect.min.x, gl_rect.min.y])
        .with_uv([tex_coords.min.x, tex_coords.min.y])
        .with_color(extra.color);
    let tr = UiVertex::default()
        .with_position([gl_rect.max.x, gl_rect.min.y])
        .with_uv([tex_coords.max.x, tex_coords.min.y])
        .with_color(extra.color);
    let br = UiVertex::default()
        .with_position([gl_rect.max.x, gl_rect.max.y])
        .with_uv([tex_coords.max.x, tex_coords.max.y])
        .with_color(extra.color);
    let bl = UiVertex::default()
        .with_position([gl_rect.min.x, gl_rect.max.y])
        .with_uv([tex_coords.min.x, tex_coords.max.y])
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
    /// The texture and mesh are meant to be used to build or update an object
    /// to display.
    pub fn get_updated(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (Option<Vec<UiVertex>>, Option<Texture>) {
        let mut may_mesh: Option<Vec<UiVertex>> = None;
        let mut may_texture: Option<Texture> = None;
        let mut cache = self.cache.take().unwrap_or_else(|| {
            let (width, height) = self.brush.texture_dimensions();
            let cache = Cache::new(device, width, height);
            may_texture = Some(cache.texture.clone());
            cache
        });

        let mut brush_action;
        loop {
            brush_action = self.brush.process_queued(
                |rect, tex_data| {
                    let offset = [rect.min[0] as u16, rect.min[1] as u16];
                    let size = [rect.width() as u16, rect.height() as u16];
                    cache.update(queue, offset, size, tex_data)
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

                    cache = Cache::new(device, new_width, new_height);
                    self.brush.resize_texture(new_width, new_height);
                    may_texture = Some(cache.texture.clone());
                }
            }
        }
        self.cache = Some(cache);

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

        (may_mesh, may_texture)
    }
}

//! Text rendering capabilities for `Renderling`.
//!
//! This module is only enabled with the `text` cargo feature.

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use ab_glyph::Rect;
use glyph_brush::*;

pub use ab_glyph::FontArc;
pub use glyph_brush::{Section, Text};

use image::{DynamicImage, GenericImage, ImageBuffer, Luma, Rgba};
use renderling::{
    atlas::AtlasTexture,
    math::{Vec2, Vec4},
    pbr::Material,
    slab::{GpuArray, Hybrid},
    stage::{Renderlet, Vertex},
};

use crate::{Ui, UiTransform};

// TODO: make UiText able to be updated without fully destroying it

pub struct UiText {
    pub cache: GlyphCache,
    pub vertices: GpuArray<Vertex>,
    pub transform: UiTransform,
    pub texture: Hybrid<AtlasTexture>,
    pub material: Hybrid<Material>,
    pub renderlet: Hybrid<Renderlet>,
    pub bounds: (Vec2, Vec2),
}

pub struct UiTextBuilder {
    ui: Ui,
    material: Material,
    bounds: (Vec2, Vec2),
    brush: GlyphBrush<Vec<Vertex>>,
}

impl UiTextBuilder {
    pub fn new(ui: &Ui) -> Self {
        Self {
            ui: ui.clone(),
            material: Material::default(),
            brush: GlyphBrushBuilder::using_fonts(ui.get_fonts()).build(),
            bounds: (Vec2::ZERO, Vec2::ZERO),
        }
    }

    pub fn set_color(&mut self, color: impl Into<Vec4>) -> &mut Self {
        self.material.albedo_factor = color.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_section<'a>(
        &mut self,
        section: impl Into<Cow<'a, Section<'a, Extra>>>,
    ) -> &mut Self {
        self.brush = self.brush.to_builder().build();
        let section: Cow<'a, Section<'a, Extra>> = section.into();
        if let Some(bounds) = self.brush.glyph_bounds(section.clone()) {
            let min = Vec2::new(bounds.min.x, bounds.min.y);
            let max = Vec2::new(bounds.max.x, bounds.max.y);
            self.bounds = (min, max);
        }
        self.brush.queue(section);
        self
    }

    pub fn with_section<'a>(mut self, section: impl Into<Cow<'a, Section<'a, Extra>>>) -> Self {
        self.set_section(section);
        self
    }

    pub fn build(self) -> UiText {
        let UiTextBuilder {
            ui,
            mut material,
            bounds,
            brush,
        } = self;
        let mut cache = GlyphCache { cache: None, brush };

        let (maybe_mesh, maybe_img) = cache.get_updated();
        let mesh = maybe_mesh.unwrap_or_default();
        let luma_img = maybe_img.unwrap_or_default();
        let img = DynamicImage::from(ImageBuffer::from_fn(
            luma_img.width(),
            luma_img.height(),
            |x, y| {
                let luma = luma_img.get_pixel(x, y);
                Rgba([255, 255, 255, luma.0[0]])
            },
        ));

        // UNWRAP: panic on purpose
        let entry = ui.stage.add_images(Some(img)).unwrap().pop().unwrap();
        material.albedo_texture_id = entry.id();

        let vertices = ui.stage.new_array(mesh);
        let material = ui.stage.new_value(material);
        let renderlet = ui.stage.new_value(Renderlet {
            vertices_array: vertices.array(),
            camera_id: ui.camera.id(),
            material_id: material.id(),
            ..Default::default()
        });
        ui.stage.add_renderlet(&renderlet);

        let transform = ui.new_transform(vec![renderlet.id()]);
        renderlet.modify(|r| r.transform_id = transform.id());

        UiText {
            cache,
            bounds,
            vertices: vertices.into_gpu_only(),
            transform,
            texture: entry,
            material,
            renderlet,
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
        min: ab_glyph::point(pixel_coords.min.x, pixel_coords.min.y),
        max: ab_glyph::point(pixel_coords.max.x, pixel_coords.max.y),
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
    /// Process any brushes, updating textures, etc.
    ///
    /// Returns a new mesh if the mesh needs to be updated.
    /// Returns a new texture if the texture needs to be updated.
    ///
    /// The texture and mesh are meant to be used to build or update a
    /// `Renderlet` to display.
    #[allow(clippy::type_complexity)]
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

#[cfg(test)]
mod test {
    use glyph_brush::Section;
    use renderling::Context;

    use crate::Ui;

    use super::*;

    #[test]
    fn can_display_uitext() {
        log::info!("{:#?}", std::env::current_dir());
        let bytes =
            std::fs::read("../../fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf").unwrap();
        let font = FontArc::try_from_vec(bytes).unwrap();

        let ctx = Context::headless(455, 145);
        let ui = Ui::new(&ctx);
        let _font_id = ui.add_font(font);
        let _text = ui
            .new_text()
            .with_section(
                Section::default()
                    .add_text(
                        Text::new("Here is some text.\n")
                            .with_scale(32.0)
                            .with_color([0.0, 0.0, 0.0, 1.0]),
                    )
                    .add_text(
                        Text::new("Here is text in a new color\n")
                            .with_scale(32.0)
                            .with_color([1.0, 1.0, 0.0, 1.0]),
                    )
                    .add_text(
                        Text::new("(and variable size)\n")
                            .with_scale(16.0)
                            .with_color([1.0, 0.0, 1.0, 1.0]),
                    )
                    .add_text(
                        Text::new("...and variable transparency\n...and word wrap")
                            .with_scale(32.0)
                            .with_color([0.2, 0.2, 0.2, 0.5]),
                    ),
            )
            .build();

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("ui/text/can_display.png", img);
    }

    #[test]
    /// Tests that if we overlay text (which has transparency) on top of other
    /// objects, it renders the transparency correctly.
    fn text_overlayed() {
        log::info!("{:#?}", std::env::current_dir());

        let ctx = Context::headless(500, 253);
        let ui = Ui::new(&ctx).with_antialiasing(false);
        let font_id = futures_lite::future::block_on(
            ui.load_font("../../fonts/Recursive Mn Lnr St Med Nerd Font Complete.ttf"),
        )
        .unwrap();
        let text1 = "Voluptas magnam sint et incidunt. Aliquam praesentium voluptas ut nemo \
                     laboriosam. Dicta qui et dicta.";
        let text2 = "Inventore impedit quo ratione ullam blanditiis soluta aliquid. Enim \
                     molestiae eaque ab commodi et.\nQuidem ex tempore ipsam. Incidunt suscipit \
                     aut commodi cum atque voluptate est.";
        let text = ui
            .new_text()
            .with_section(
                Section::default().add_text(
                    Text::new(text1)
                        .with_scale(24.0)
                        .with_color([0.0, 0.0, 0.0, 1.0])
                        .with_font_id(font_id),
                ),
            )
            .with_section(
                Section::default()
                    .add_text(
                        Text::new(text2)
                            .with_scale(24.0)
                            .with_color([0.0, 0.0, 0.0, 1.0]),
                    )
                    .with_bounds((400.0, f32::INFINITY)),
            )
            .build();

        let (fill, stroke) = ui
            .new_path()
            .with_fill_color([1.0, 1.0, 0.0, 1.0])
            .with_stroke_color([1.0, 0.0, 1.0, 1.0])
            .with_rectangle(text.bounds.0, text.bounds.1)
            .fill_and_stroke();

        for path in [&fill, &stroke] {
            // move the path to (50, 50)
            path.transform.set_translation(Vec2::new(51.0, 53.0));
            // move it to the back
            path.transform.set_z(-0.1);
        }

        let frame = ctx.get_next_frame().unwrap();
        ui.render(&frame.view());
        let img = frame.read_image().unwrap();
        img_diff::assert_img_eq("ui/text/overlay.png", img);
        let depth_img = ui.stage.get_depth_texture().read_image().unwrap();
        img_diff::assert_img_eq("ui/text/overlay_depth.png", depth_img);
    }
}

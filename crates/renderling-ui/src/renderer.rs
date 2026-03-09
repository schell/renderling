//! Core `UiRenderer` implementation.
//!
//! This module contains the GPU pipeline setup, element management,
//! and rendering logic for the 2D/UI renderer.
//!
//! ## Architecture
//!
//! The renderer uses a [`SlabAllocator`] from `craballoc` to manage GPU
//! memory. Each UI element is backed by a [`Hybrid<UiDrawCallDescriptor>`]
//! which keeps a CPU copy in sync with a GPU slab allocation. Calling
//! [`SlabAllocator::commit`] flushes all pending changes to the GPU buffer.
//!
//! Element wrapper types ([`UiRect`], [`UiCircle`],
//! [`UiEllipse`]) follow the same pattern as
//! [`renderling::camera::Camera`] — each wraps a `Hybrid` and provides
//! typed setter methods that queue GPU updates automatically.

use craballoc::{
    prelude::*,
    slab::{SlabAllocator, SlabBuffer},
    value::Hybrid,
};
use crabslab::Id;
use glam::{Mat4, UVec2, Vec2, Vec4};
use renderling::{
    atlas::{Atlas, AtlasImage, AtlasTexture},
    context::Context,
    ui_slab::{GradientDescriptor, UiDrawCallDescriptor, UiElementType, UiViewport},
};

// ---------------------------------------------------------------------------
// Element wrapper types (follow the Camera pattern from camera/cpu.rs)
// ---------------------------------------------------------------------------

/// A live handle to a rectangle element in the renderer.
///
/// Modifications via the `set_*` methods are reflected on the GPU after
/// the next call to [`UiRenderer::render`].
///
/// Clones of this type all point to the same underlying GPU data.
///
/// **Dropping this handle does NOT remove the element** — call
/// [`UiRenderer::remove_rect`] explicitly.
#[derive(Clone, Debug)]
pub struct UiRect {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiRect {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the top-left position in screen pixels.
    pub fn set_position(&self, position: Vec2) -> &Self {
        self.inner.modify(|d| d.position = position);
        self
    }

    /// Set the top-left position in screen pixels (builder).
    pub fn with_position(self, position: Vec2) -> Self {
        self.set_position(position);
        self
    }

    /// Set the size in screen pixels.
    pub fn set_size(&self, size: Vec2) -> &Self {
        self.inner.modify(|d| d.size = size);
        self
    }

    /// Set the size in screen pixels (builder).
    pub fn with_size(self, size: Vec2) -> Self {
        self.set_size(size);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set per-corner radii (top-left, top-right, bottom-right,
    /// bottom-left).
    pub fn set_corner_radii(&self, radii: Vec4) -> &Self {
        self.inner.modify(|d| d.corner_radii = radii);
        self
    }

    /// Set per-corner radii (builder).
    pub fn with_corner_radii(self, radii: Vec4) -> Self {
        self.set_corner_radii(radii);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity (0.0 = transparent, 1.0 = opaque).
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting. Lower values are drawn first.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

/// A live handle to a circle element in the renderer.
///
/// See [`UiRect`] for general usage notes.
#[derive(Clone, Debug)]
pub struct UiCircle {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiCircle {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the center position in screen pixels.
    pub fn set_center(&self, center: Vec2) -> &Self {
        self.inner.modify(|d| {
            let radius = d.size.x / 2.0;
            d.position = center - Vec2::splat(radius);
        });
        self
    }

    /// Set the center position in screen pixels (builder).
    pub fn with_center(self, center: Vec2) -> Self {
        self.set_center(center);
        self
    }

    /// Set the radius in screen pixels.
    pub fn set_radius(&self, radius: f32) -> &Self {
        self.inner.modify(|d| {
            let center = d.position + d.size / 2.0;
            d.size = Vec2::splat(radius * 2.0);
            d.position = center - Vec2::splat(radius);
        });
        self
    }

    /// Set the radius in screen pixels (builder).
    pub fn with_radius(self, radius: f32) -> Self {
        self.set_radius(radius);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity.
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

/// A live handle to an ellipse element in the renderer.
///
/// See [`UiRect`] for general usage notes.
#[derive(Clone, Debug)]
pub struct UiEllipse {
    inner: Hybrid<UiDrawCallDescriptor>,
}

impl UiEllipse {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the center position in screen pixels.
    pub fn set_center(&self, center: Vec2) -> &Self {
        self.inner.modify(|d| {
            let radii = d.size / 2.0;
            d.position = center - radii;
        });
        self
    }

    /// Set the center position in screen pixels (builder).
    pub fn with_center(self, center: Vec2) -> Self {
        self.set_center(center);
        self
    }

    /// Set the radii (horizontal, vertical) in screen pixels.
    pub fn set_radii(&self, radii: Vec2) -> &Self {
        self.inner.modify(|d| {
            let center = d.position + d.size / 2.0;
            d.size = radii * 2.0;
            d.position = center - radii;
        });
        self
    }

    /// Set the radii (builder).
    pub fn with_radii(self, radii: Vec2) -> Self {
        self.set_radii(radii);
        self
    }

    /// Set the fill color (RGBA).
    pub fn set_fill_color(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set the fill color (builder).
    pub fn with_fill_color(self, color: Vec4) -> Self {
        self.set_fill_color(color);
        self
    }

    /// Set the border width and color.
    pub fn set_border(&self, width: f32, color: Vec4) -> &Self {
        self.inner.modify(|d| {
            d.border_width = width;
            d.border_color = color;
        });
        self
    }

    /// Set the border width and color (builder).
    pub fn with_border(self, width: f32, color: Vec4) -> Self {
        self.set_border(width, color);
        self
    }

    /// Set the gradient fill. Pass `None` to remove the gradient.
    pub fn set_gradient(&self, gradient: Option<GradientDescriptor>) -> &Self {
        self.inner
            .modify(|d| d.gradient = gradient.unwrap_or_default());
        self
    }

    /// Set the gradient fill (builder).
    pub fn with_gradient(self, gradient: Option<GradientDescriptor>) -> Self {
        self.set_gradient(gradient);
        self
    }

    /// Set the opacity.
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

/// A live handle to an image element in the renderer.
///
/// See [`UiRect`] for general usage notes.
#[derive(Clone)]
pub struct UiImage {
    inner: Hybrid<UiDrawCallDescriptor>,
    /// Kept alive to prevent the atlas from garbage-collecting the texture.
    #[allow(dead_code)]
    atlas_texture: AtlasTexture,
}

impl std::fmt::Debug for UiImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiImage")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

impl UiImage {
    /// Returns the slab [`Id`] of the underlying descriptor.
    pub fn id(&self) -> Id<UiDrawCallDescriptor> {
        self.inner.id()
    }

    /// Returns a copy of the underlying descriptor.
    pub fn descriptor(&self) -> UiDrawCallDescriptor {
        self.inner.get()
    }

    /// Set the top-left position in screen pixels.
    pub fn set_position(&self, position: Vec2) -> &Self {
        self.inner.modify(|d| d.position = position);
        self
    }

    /// Set the top-left position in screen pixels (builder).
    pub fn with_position(self, position: Vec2) -> Self {
        self.set_position(position);
        self
    }

    /// Set the size in screen pixels.
    pub fn set_size(&self, size: Vec2) -> &Self {
        self.inner.modify(|d| d.size = size);
        self
    }

    /// Set the size in screen pixels (builder).
    pub fn with_size(self, size: Vec2) -> Self {
        self.set_size(size);
        self
    }

    /// Set a tint color (multiplied with the texture color).
    /// Use `Vec4::ONE` for no tint.
    pub fn set_tint(&self, color: Vec4) -> &Self {
        self.inner.modify(|d| d.fill_color = color);
        self
    }

    /// Set a tint color (builder).
    pub fn with_tint(self, color: Vec4) -> Self {
        self.set_tint(color);
        self
    }

    /// Set the opacity (0.0 = transparent, 1.0 = opaque).
    pub fn set_opacity(&self, opacity: f32) -> &Self {
        self.inner.modify(|d| d.opacity = opacity);
        self
    }

    /// Set the opacity (builder).
    pub fn with_opacity(self, opacity: f32) -> Self {
        self.set_opacity(opacity);
        self
    }

    /// Set the z-depth for sorting.
    pub fn set_z(&self, z: f32) -> &Self {
        self.inner.modify(|d| d.z = z);
        self
    }

    /// Set the z-depth for sorting (builder).
    pub fn with_z(self, z: f32) -> Self {
        self.set_z(z);
        self
    }
}

// ---------------------------------------------------------------------------
// Text types (behind "text" feature)
// ---------------------------------------------------------------------------

#[cfg(feature = "text")]
mod text {
    use super::*;
    use glyph_brush::ab_glyph;

    /// Re-export common glyph_brush types for convenience.
    pub use ab_glyph::FontArc;
    use glyph_brush::GlyphCruncher as _;
    pub use glyph_brush::{FontId, Section, Text};

    /// A CPU-side glyph rasterization cache.
    ///
    /// Wraps a `GlyphBrush` and maintains a single-channel (Luma8) image
    /// that accumulates rasterized glyph bitmaps.
    pub(crate) struct GlyphCache {
        brush: glyph_brush::GlyphBrush<GlyphQuad>,
        cache_img: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
        /// Cached dimensions (updated whenever cache_img is replaced).
        cache_w: f32,
        cache_h: f32,
        dirty: bool,
    }

    /// Intermediate representation of one glyph quad produced by the brush.
    #[derive(Clone, Debug)]
    pub(crate) struct GlyphQuad {
        /// Top-left position in screen pixels.
        pub position: Vec2,
        /// Size in screen pixels.
        pub size: Vec2,
        /// UV rect within the glyph cache image (in pixels).
        pub tex_offset_px: UVec2,
        /// UV rect size within the glyph cache image (in pixels).
        pub tex_size_px: UVec2,
        /// Text color from the section.
        pub color: Vec4,
    }

    impl GlyphCache {
        /// Create a new glyph cache with the given fonts.
        pub fn new(fonts: Vec<FontArc>) -> Self {
            let brush = glyph_brush::GlyphBrushBuilder::using_fonts(fonts).build();
            let (w, h) = brush.texture_dimensions();
            Self {
                brush,
                cache_img: image::ImageBuffer::from_pixel(w, h, image::Luma([0])),
                cache_w: w as f32,
                cache_h: h as f32,
                dirty: false,
            }
        }

        /// Rebuild the brush with the current font set (after adding fonts).
        pub fn rebuild_with_fonts(&mut self, fonts: Vec<FontArc>) {
            self.brush = self.brush.to_builder().replace_fonts(|_| fonts).build();
            let (w, h) = self.brush.texture_dimensions();
            self.cache_img = image::ImageBuffer::from_pixel(w, h, image::Luma([0]));
            self.cache_w = w as f32;
            self.cache_h = h as f32;
            self.dirty = false;
        }

        /// Queue a section for layout and rasterization.
        pub fn queue<'a>(&mut self, section: impl Into<std::borrow::Cow<'a, Section<'a>>>) {
            self.brush.queue(section);
        }

        /// Compute the bounding rectangle for a section.
        pub fn glyph_bounds<'a>(
            &mut self,
            section: impl Into<std::borrow::Cow<'a, Section<'a>>>,
        ) -> Option<ab_glyph::Rect> {
            self.brush.glyph_bounds(section)
        }

        /// Process queued sections, rasterizing glyphs and producing quad
        /// data. Returns `Some(quads)` if new vertices need to be drawn,
        /// or `None` if the previous frame's data can be reused.
        ///
        /// Also marks whether the cache image is dirty (needs re-upload).
        pub fn process(&mut self) -> Option<Vec<GlyphQuad>> {
            let cache_img = &mut self.cache_img;
            let dirty = &mut self.dirty;

            let mut result;
            loop {
                // Capture dimensions each iteration (they change on resize).
                let cw = cache_img.width() as f32;
                let ch = cache_img.height() as f32;
                result = self.brush.process_queued(
                    // Callback: write rasterized glyph data into cache image.
                    |rect, tex_data| {
                        let src = image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::from_vec(
                            rect.width(),
                            rect.height(),
                            tex_data.to_vec(),
                        )
                        .expect("glyph rasterization buffer size mismatch");
                        image::imageops::replace(
                            cache_img,
                            &src,
                            rect.min[0] as i64,
                            rect.min[1] as i64,
                        );
                        *dirty = true;
                    },
                    // Callback: convert GlyphVertex -> GlyphQuad.
                    |gv| {
                        let mut tex_coords = gv.tex_coords;
                        let pixel_coords = gv.pixel_coords;
                        let bounds = gv.bounds;

                        // Clip glyph rect to section bounds.
                        let mut gl_rect = ab_glyph::Rect {
                            min: ab_glyph::point(pixel_coords.min.x, pixel_coords.min.y),
                            max: ab_glyph::point(pixel_coords.max.x, pixel_coords.max.y),
                        };

                        if gl_rect.max.x > bounds.max.x {
                            let old_width = gl_rect.width();
                            gl_rect.max.x = bounds.max.x;
                            tex_coords.max.x =
                                tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
                        }
                        if gl_rect.min.x < bounds.min.x {
                            let old_width = gl_rect.width();
                            gl_rect.min.x = bounds.min.x;
                            tex_coords.min.x =
                                tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
                        }
                        if gl_rect.max.y > bounds.max.y {
                            let old_height = gl_rect.height();
                            gl_rect.max.y = bounds.max.y;
                            tex_coords.max.y = tex_coords.min.y
                                + tex_coords.height() * gl_rect.height() / old_height;
                        }
                        if gl_rect.min.y < bounds.min.y {
                            let old_height = gl_rect.height();
                            gl_rect.min.y = bounds.min.y;
                            tex_coords.min.y = tex_coords.max.y
                                - tex_coords.height() * gl_rect.height() / old_height;
                        }

                        // tex_coords are in normalized 0..1 space of the
                        // glyph cache image. Convert to pixel coordinates.
                        let tex_offset_px = UVec2::new(
                            (tex_coords.min.x * cw) as u32,
                            (tex_coords.min.y * ch) as u32,
                        );
                        let tex_size_px = UVec2::new(
                            ((tex_coords.max.x - tex_coords.min.x) * cw) as u32,
                            ((tex_coords.max.y - tex_coords.min.y) * ch) as u32,
                        );

                        GlyphQuad {
                            position: Vec2::new(gl_rect.min.x, gl_rect.min.y),
                            size: Vec2::new(gl_rect.width(), gl_rect.height()),
                            tex_offset_px,
                            tex_size_px,
                            color: Vec4::new(
                                gv.extra.color[0],
                                gv.extra.color[1],
                                gv.extra.color[2],
                                gv.extra.color[3],
                            ),
                        }
                    },
                );

                match &result {
                    Err(glyph_brush::BrushError::TextureTooSmall { suggested, .. }) => {
                        let (new_w, new_h) = *suggested;
                        let max_dim = 2048;
                        let (new_w, new_h) = if (new_w > max_dim || new_h > max_dim)
                            && (cache_img.width() < max_dim || cache_img.height() < max_dim)
                        {
                            (max_dim, max_dim)
                        } else {
                            (new_w, new_h)
                        };
                        *cache_img = image::ImageBuffer::from_pixel(new_w, new_h, image::Luma([0]));
                        self.brush.resize_texture(new_w, new_h);
                        *dirty = true;
                    }
                    Ok(_) => break,
                }
            }

            match result.unwrap() {
                glyph_brush::BrushAction::Draw(quads) => Some(quads),
                glyph_brush::BrushAction::ReDraw => None,
            }
        }

        /// Returns the cache image if it has been modified since the last
        /// call to `take_image()`, converting from Luma8 to RGBA8 (white +
        /// alpha).
        pub fn take_image(&mut self) -> Option<image::RgbaImage> {
            if !self.dirty {
                return None;
            }
            self.dirty = false;
            let (w, h) = (self.cache_img.width(), self.cache_img.height());
            let rgba = image::RgbaImage::from_fn(w, h, |x, y| {
                let luma = self.cache_img.get_pixel(x, y).0[0];
                image::Rgba([255, 255, 255, luma])
            });
            Some(rgba)
        }
    }

    /// A live handle to a text element in the renderer.
    ///
    /// This represents a block of text rendered as a set of glyph quads.
    /// Each glyph is a separate draw call internally, but they are all
    /// managed as a single logical element.
    ///
    /// **Dropping this handle does NOT remove the text** — call
    /// [`UiRenderer::remove_text`] explicitly.
    #[derive(Clone, Debug)]
    pub struct UiText {
        /// The descriptors for each glyph quad (one per visible glyph).
        pub(crate) glyph_descriptors: Vec<Hybrid<UiDrawCallDescriptor>>,
        /// Per-glyph atlas texture descriptors (kept alive for slab lifetime).
        #[allow(dead_code)]
        pub(crate) glyph_atlas_descriptors:
            Vec<Hybrid<renderling::atlas::shader::AtlasTextureDescriptor>>,
        /// Bounding box of the text (min, max) in screen pixels.
        pub(crate) bounds: (Vec2, Vec2),
        /// Unique identifier for this text block.
        #[allow(dead_code)]
        pub(crate) text_id: u64,
    }

    impl UiText {
        /// Returns the bounding box of the laid-out text (min, max) in
        /// screen pixels.
        pub fn bounds(&self) -> (Vec2, Vec2) {
            self.bounds
        }

        /// Set the z-depth for all glyphs in this text block.
        pub fn set_z(&self, z: f32) -> &Self {
            for desc in &self.glyph_descriptors {
                desc.modify(|d| d.z = z);
            }
            self
        }

        /// Set the z-depth for all glyphs (builder).
        pub fn with_z(self, z: f32) -> Self {
            self.set_z(z);
            self
        }

        /// Set the opacity for all glyphs in this text block.
        pub fn set_opacity(&self, opacity: f32) -> &Self {
            for desc in &self.glyph_descriptors {
                desc.modify(|d| d.opacity = opacity);
            }
            self
        }

        /// Set the opacity for all glyphs (builder).
        pub fn with_opacity(self, opacity: f32) -> Self {
            self.set_opacity(opacity);
            self
        }
    }
}

#[cfg(feature = "text")]
use text::GlyphCache;
#[cfg(feature = "text")]
pub use text::{FontArc, FontId, Section, Text, UiText};

// ---------------------------------------------------------------------------
// Internal draw call entry
// ---------------------------------------------------------------------------

/// Internal representation of a draw call for the renderer.
struct DrawCall {
    /// The hybrid descriptor (shared with the element wrapper).
    descriptor: Hybrid<UiDrawCallDescriptor>,
    /// Number of vertices (6 for quads, variable for paths).
    vertex_count: u32,
}

// ---------------------------------------------------------------------------
// UiRenderer
// ---------------------------------------------------------------------------

/// The 2D/UI renderer.
///
/// This renderer maintains its own lightweight GPU pipeline separate from
/// renderling's 3D PBR pipeline. It renders directly to a provided
/// `TextureView` with no intermediate HDR buffer, bloom, or tonemapping.
///
/// GPU memory is managed via a [`SlabAllocator`]. Each element is a
/// [`Hybrid<UiDrawCallDescriptor>`] — modifications via the element
/// wrapper types are automatically synced to the GPU on the next
/// [`render`](Self::render) call.
pub struct UiRenderer {
    slab: SlabAllocator<WgpuRuntime>,
    viewport: Hybrid<UiViewport>,
    atlas: Atlas,
    pipeline: wgpu::RenderPipeline,
    bindgroup_layout: wgpu::BindGroupLayout,
    /// Cached slab buffer from the last commit.
    slab_buffer: Option<SlabBuffer<wgpu::Buffer>>,
    /// Cached bind group (recreated when slab buffer changes).
    bindgroup: Option<wgpu::BindGroup>,
    /// ID of the atlas texture at the time the bind group was created.
    /// Used to detect when the atlas is recreated and the bind group
    /// needs rebuilding.
    bindgroup_atlas_texture_id: Option<usize>,
    /// All active draw calls, sorted by z before rendering.
    draw_calls: Vec<DrawCall>,
    /// Viewport size.
    viewport_size: UVec2,
    /// Background clear color.
    background_color: Option<Vec4>,
    /// MSAA sample count.
    msaa_sample_count: u32,
    /// The texture format of the render target.
    format: wgpu::TextureFormat,
    /// MSAA resolve texture (if msaa_sample_count > 1).
    msaa_texture: Option<wgpu::TextureView>,

    // --- Text support (behind "text" feature) ---
    #[cfg(feature = "text")]
    fonts: Vec<glyph_brush::ab_glyph::FontArc>,
    #[cfg(feature = "text")]
    glyph_cache: GlyphCache,
    /// Atlas texture entry for the glyph cache image. Replaced when the
    /// cache image is re-uploaded.
    #[cfg(feature = "text")]
    glyph_cache_atlas_texture: Option<AtlasTexture>,
    /// Monotonic counter for assigning unique text block IDs.
    #[cfg(feature = "text")]
    next_text_id: u64,
}

impl UiRenderer {
    const LABEL: Option<&'static str> = Some("renderling-ui");

    /// Default atlas texture size.
    const DEFAULT_ATLAS_SIZE: wgpu::Extent3d = wgpu::Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 2,
    };

    /// Create a new `UiRenderer` from a renderling `Context`.
    pub fn new(ctx: &Context) -> Self {
        let device = ctx.get_device();
        let size = ctx.get_size();
        let format = ctx.get_render_target().format();

        let slab = SlabAllocator::new(ctx.runtime(), "ui-slab", wgpu::BufferUsages::empty());

        // IMPORTANT: The viewport must be the first slab allocation so it
        // lands at offset 0. The vertex/fragment shaders read UiViewport
        // via `Id::new(0)`.
        let viewport = slab.new_value(UiViewport {
            projection: Self::ortho2d(size.x as f32, size.y as f32),
            size,
            atlas_size: UVec2::new(
                Self::DEFAULT_ATLAS_SIZE.width,
                Self::DEFAULT_ATLAS_SIZE.height,
            ),
        });

        let atlas = Atlas::new(
            &slab,
            Self::DEFAULT_ATLAS_SIZE,
            None,
            Some("ui-atlas"),
            None,
        );

        let bindgroup_layout = Self::create_bindgroup_layout(device);
        let pipeline = Self::create_pipeline(device, &bindgroup_layout, format, 1);

        Self {
            slab,
            viewport,
            atlas,
            pipeline,
            bindgroup_layout,
            slab_buffer: None,
            bindgroup: None,
            bindgroup_atlas_texture_id: None,
            draw_calls: Vec::new(),
            viewport_size: size,
            background_color: None,
            msaa_sample_count: 1,
            format,
            msaa_texture: None,
            #[cfg(feature = "text")]
            fonts: Vec::new(),
            #[cfg(feature = "text")]
            glyph_cache: GlyphCache::new(Vec::new()),
            #[cfg(feature = "text")]
            glyph_cache_atlas_texture: None,
            #[cfg(feature = "text")]
            next_text_id: 0,
        }
    }

    /// Set the background clear color. `None` means don't clear
    /// (load existing content).
    pub fn set_background_color(&mut self, color: Option<Vec4>) -> &mut Self {
        self.background_color = color;
        self
    }

    /// Builder-style background color setter.
    pub fn with_background_color(mut self, color: Vec4) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the viewport size (typically matches the render target size).
    pub fn set_size(&mut self, size: UVec2) {
        if self.viewport_size != size {
            self.viewport_size = size;
            self.viewport.modify(|v| {
                v.projection = Self::ortho2d(size.x as f32, size.y as f32);
                v.size = size;
            });

            // Recreate MSAA texture if needed.
            if self.msaa_sample_count > 1 {
                self.msaa_texture = Some(Self::create_msaa_texture(
                    self.slab.device(),
                    self.format,
                    size,
                    self.msaa_sample_count,
                ));
            }
        }
    }

    /// Add a rectangle element and return a live handle.
    ///
    /// The element starts with sensible defaults (100x100 white rect
    /// at the origin). Use the `with_*` builder methods or `set_*`
    /// methods to configure it.
    ///
    /// ```ignore
    /// let rect = ui.add_rect()
    ///     .with_position(Vec2::new(10.0, 10.0))
    ///     .with_size(Vec2::new(200.0, 100.0))
    ///     .with_fill_color(Vec4::new(0.2, 0.4, 0.8, 1.0));
    /// ```
    pub fn add_rect(&mut self) -> UiRect {
        let desc = self.default_descriptor(UiElementType::Rectangle);
        let hybrid = self.slab.new_value(desc);
        let element = UiRect {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Add a circle element and return a live handle.
    ///
    /// The element starts centered at (0, 0) with radius 50 and
    /// white fill. Use `with_center`, `with_radius`, etc. to
    /// configure.
    pub fn add_circle(&mut self) -> UiCircle {
        let desc = self.default_descriptor(UiElementType::Circle);
        let hybrid = self.slab.new_value(desc);
        let element = UiCircle {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Add an ellipse element and return a live handle.
    ///
    /// The element starts centered at (0, 0) with size 100x100 and
    /// white fill. Use `with_center`, `with_radii`, etc. to
    /// configure.
    pub fn add_ellipse(&mut self) -> UiEllipse {
        let desc = self.default_descriptor(UiElementType::Ellipse);
        let hybrid = self.slab.new_value(desc);
        let element = UiEllipse {
            inner: hybrid.clone(),
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Add an image element and return a live handle.
    ///
    /// The image is loaded into the atlas from an [`AtlasImage`]
    /// (CPU-side pixel data). The element is sized to match the
    /// image dimensions by default.
    ///
    /// ```ignore
    /// let img = image::open("icon.png").unwrap();
    /// let _icon = ui.add_image(img.into())
    ///     .with_position(Vec2::new(10.0, 10.0));
    /// ```
    pub fn add_image(&mut self, image: impl Into<AtlasImage>) -> UiImage {
        let image = image.into();
        let image_size = image.size;
        let atlas_texture = self
            .atlas
            .add_image(&image)
            .expect("failed to add image to atlas");

        // Update the viewport with the (possibly new) atlas size.
        let atlas_extent = self.atlas.get_size();
        self.viewport.modify(|v| {
            v.atlas_size = UVec2::new(atlas_extent.width, atlas_extent.height);
        });

        let mut desc = self.default_descriptor(UiElementType::Image);
        desc.size = Vec2::new(image_size.x as f32, image_size.y as f32);
        desc.atlas_texture_id = atlas_texture.id().inner();
        desc.fill_color = Vec4::ONE; // no tint

        let hybrid = self.slab.new_value(desc);
        let element = UiImage {
            inner: hybrid.clone(),
            atlas_texture,
        };
        self.draw_calls.push(DrawCall {
            descriptor: hybrid,
            vertex_count: 6,
        });
        element
    }

    /// Register a font and return its [`FontId`].
    ///
    /// Fonts must be registered before they can be used in
    /// [`Section`]/[`Text`] for [`add_text`](Self::add_text).
    ///
    /// ```ignore
    /// let bytes = std::fs::read("fonts/MyFont.ttf").unwrap();
    /// let font = FontArc::try_from_vec(bytes).unwrap();
    /// let font_id = ui.add_font(font);
    /// ```
    #[cfg(feature = "text")]
    pub fn add_font(&mut self, font: FontArc) -> FontId {
        let id = self.fonts.len();
        self.fonts.push(font);
        self.glyph_cache.rebuild_with_fonts(self.fonts.clone());
        FontId(id)
    }

    /// Add a text element from a glyph_brush [`Section`].
    ///
    /// This rasterizes the glyphs, uploads the cache image to the atlas,
    /// and creates one draw call per visible glyph.
    ///
    /// ```ignore
    /// use glyph_brush::{Section, Text};
    /// let font_id = ui.add_font(my_font);
    /// let _text = ui.add_text(
    ///     Section::default()
    ///         .add_text(
    ///             Text::new("Hello, UI!")
    ///                 .with_scale(32.0)
    ///                 .with_color([0.0, 0.0, 0.0, 1.0])
    ///         )
    ///         .with_screen_position((10.0, 10.0))
    /// );
    /// ```
    #[cfg(feature = "text")]
    pub fn add_text<'a>(
        &mut self,
        section: impl Into<std::borrow::Cow<'a, Section<'a>>>,
    ) -> UiText {
        use renderling::atlas::shader::AtlasTextureDescriptor;

        let section = section.into();

        // Compute text bounds.
        let bounds = self
            .glyph_cache
            .glyph_bounds(section.clone())
            .map(|r| (Vec2::new(r.min.x, r.min.y), Vec2::new(r.max.x, r.max.y)))
            .unwrap_or((Vec2::ZERO, Vec2::ZERO));

        // Queue and process.
        self.glyph_cache.queue(section);
        let quads = self.glyph_cache.process().unwrap_or_default();

        // Upload the glyph cache image to the atlas (if dirty).
        if let Some(rgba_img) = self.glyph_cache.take_image() {
            // Drop old atlas entry (if any) so the atlas can reclaim space.
            self.glyph_cache_atlas_texture = None;

            let atlas_img = AtlasImage::from(image::DynamicImage::ImageRgba8(rgba_img));
            let atlas_tex = self
                .atlas
                .add_image(&atlas_img)
                .expect("failed to upload glyph cache to atlas");

            // Update the viewport with the (possibly new) atlas size.
            let atlas_extent = self.atlas.get_size();
            self.viewport.modify(|v| {
                v.atlas_size = UVec2::new(atlas_extent.width, atlas_extent.height);
            });

            self.glyph_cache_atlas_texture = Some(atlas_tex);
        }

        // Get the atlas placement of the glyph cache image.
        let cache_atlas_desc = self
            .glyph_cache_atlas_texture
            .as_ref()
            .expect("glyph cache not uploaded")
            .descriptor();

        let text_id = self.next_text_id;
        self.next_text_id += 1;

        let mut glyph_descriptors = Vec::with_capacity(quads.len());
        let mut glyph_atlas_descriptors = Vec::with_capacity(quads.len());

        for quad in &quads {
            // Create an AtlasTextureDescriptor for this specific glyph's
            // sub-region within the glyph cache, which itself is a sub-
            // region of the atlas.
            let glyph_atlas_desc = AtlasTextureDescriptor {
                offset_px: cache_atlas_desc.offset_px + quad.tex_offset_px,
                size_px: quad.tex_size_px,
                layer_index: cache_atlas_desc.layer_index,
                frame_index: 0,
                ..Default::default()
            };
            let glyph_atlas_hybrid = self.slab.new_value(glyph_atlas_desc);

            let mut desc = self.default_descriptor(UiElementType::TextGlyph);
            desc.position = quad.position;
            desc.size = quad.size;
            desc.fill_color = quad.color;
            desc.atlas_texture_id = glyph_atlas_hybrid.id().inner();

            let hybrid = self.slab.new_value(desc);
            self.draw_calls.push(DrawCall {
                descriptor: hybrid.clone(),
                vertex_count: 6,
            });

            glyph_descriptors.push(hybrid);
            glyph_atlas_descriptors.push(glyph_atlas_hybrid);
        }

        UiText {
            glyph_descriptors,
            glyph_atlas_descriptors,
            bounds,
            text_id,
        }
    }

    /// Remove a rectangle element by its handle.
    pub fn remove_rect(&mut self, element: &UiRect) {
        self.remove_by_id(element.id());
    }

    /// Remove a circle element by its handle.
    pub fn remove_circle(&mut self, element: &UiCircle) {
        self.remove_by_id(element.id());
    }

    /// Remove an ellipse element by its handle.
    pub fn remove_ellipse(&mut self, element: &UiEllipse) {
        self.remove_by_id(element.id());
    }

    /// Remove an image element by its handle.
    pub fn remove_image(&mut self, element: &UiImage) {
        self.remove_by_id(element.id());
    }

    /// Remove a text element by its handle.
    #[cfg(feature = "text")]
    pub fn remove_text(&mut self, element: &UiText) {
        for desc in &element.glyph_descriptors {
            self.remove_by_id(desc.id());
        }
    }

    /// Remove all elements.
    pub fn clear(&mut self) {
        self.draw_calls.clear();
        // Dropping the Hybrid values reclaims slab memory on next
        // commit.
    }

    /// Render all UI elements to the given texture view.
    pub fn render(&mut self, view: &wgpu::TextureView) {
        if self.draw_calls.is_empty() {
            return;
        }

        // Sort draw calls by z (painter's algorithm).
        // We read z from the CPU-side Hybrid each frame.
        let mut sorted_indices: Vec<usize> = (0..self.draw_calls.len()).collect();
        sorted_indices.sort_by(|a, b| {
            let z_a = self.draw_calls[*a].descriptor.get().z;
            let z_b = self.draw_calls[*b].descriptor.get().z;
            z_a.partial_cmp(&z_b).unwrap_or(core::cmp::Ordering::Equal)
        });

        // Run atlas upkeep (garbage-collect dropped textures).
        let atlas_texture_recreated = self.atlas.upkeep(self.slab.runtime());
        if atlas_texture_recreated {
            // Update viewport with new atlas size.
            let extent = self.atlas.get_size();
            self.viewport.modify(|v| {
                v.atlas_size = UVec2::new(extent.width, extent.height);
            });
        }

        // Commit slab changes to the GPU.
        let buffer = self.slab.commit();

        // Check if bind group needs recreation: slab buffer changed,
        // atlas texture changed, or first render.
        let atlas_tex = self.atlas.get_texture();
        let atlas_tex_id = atlas_tex.id();
        let atlas_changed = self.bindgroup_atlas_texture_id != Some(atlas_tex_id);
        let should_recreate_bindgroup =
            buffer.is_new_this_commit() || atlas_changed || self.bindgroup.is_none();

        if should_recreate_bindgroup {
            self.bindgroup = Some(self.create_bindgroup(&buffer, &atlas_tex));
            self.bindgroup_atlas_texture_id = Some(atlas_tex_id);
        }
        drop(atlas_tex);
        self.slab_buffer = Some(buffer);

        let device = self.slab.device();
        let queue = self.slab.queue();

        // Create command encoder.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Self::LABEL });

        let load_op = if let Some(bg) = self.background_color {
            wgpu::LoadOp::Clear(wgpu::Color {
                r: bg.x as f64,
                g: bg.y as f64,
                b: bg.z as f64,
                a: bg.w as f64,
            })
        } else {
            wgpu::LoadOp::Load
        };

        let (color_view, resolve_target) = if self.msaa_sample_count > 1 {
            if let Some(msaa_view) = &self.msaa_texture {
                (msaa_view, Some(view))
            } else {
                (view, None)
            }
        } else {
            (view, None)
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Self::LABEL,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target,
                    ops: wgpu::Operations {
                        load: load_op,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, self.bindgroup.as_ref().unwrap(), &[]);

            // Issue one draw call per element, sorted by z.
            // The instance_index encodes the slab offset of the
            // UiDrawCallDescriptor.
            for &idx in &sorted_indices {
                let dc = &self.draw_calls[idx];
                let inst = dc.descriptor.id().inner();
                render_pass.draw(0..dc.vertex_count, inst..inst + 1);
            }
        }

        queue.submit(Some(encoder.finish()));
    }

    // --- Private helpers ---

    fn ortho2d(width: f32, height: f32) -> Mat4 {
        Mat4::orthographic_rh(
            0.0,    // left
            width,  // right
            height, // bottom
            0.0,    // top
            -1.0,   // near
            1.0,    // far
        )
    }

    /// Build a default [`UiDrawCallDescriptor`] for the given element
    /// type, using the current viewport as the clip rect.
    fn default_descriptor(&self, element_type: UiElementType) -> UiDrawCallDescriptor {
        UiDrawCallDescriptor {
            element_type,
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 100.0),
            corner_radii: Vec4::ZERO,
            border_width: 0.0,
            border_color: Vec4::ZERO,
            fill_color: Vec4::ONE,
            gradient: GradientDescriptor::default(),
            atlas_texture_id: u32::MAX,
            atlas_descriptor_id: u32::MAX,
            clip_rect: Vec4::new(
                0.0,
                0.0,
                self.viewport_size.x as f32,
                self.viewport_size.y as f32,
            ),
            opacity: 1.0,
            z: 0.0,
        }
    }

    fn create_bindgroup_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Self::LABEL,
            entries: &[
                // Binding 0: Slab storage buffer.
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Binding 1: Atlas texture (2D array).
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                // Binding 2: Atlas sampler.
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    fn create_pipeline(
        device: &wgpu::Device,
        bindgroup_layout: &wgpu::BindGroupLayout,
        format: wgpu::TextureFormat,
        msaa_sample_count: u32,
    ) -> wgpu::RenderPipeline {
        let vertex_linkage = renderling::linkage::ui_vertex::linkage(device);
        let fragment_linkage = renderling::linkage::ui_fragment::linkage(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Self::LABEL,
            bind_group_layouts: &[bindgroup_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Self::LABEL,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_linkage.module,
                entry_point: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: msaa_sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_linkage.module,
                entry_point: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        })
    }

    fn create_msaa_texture(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        size: UVec2,
        sample_count: u32,
    ) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("renderling-ui-msaa"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Create a bind group using the given slab buffer and atlas
    /// texture.
    fn create_bindgroup(
        &self,
        buffer: &SlabBuffer<wgpu::Buffer>,
        atlas_tex: &renderling::texture::Texture,
    ) -> wgpu::BindGroup {
        self.slab
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Self::LABEL,
                layout: &self.bindgroup_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&atlas_tex.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&atlas_tex.sampler),
                    },
                ],
            })
    }

    /// Remove a draw call by its slab ID.
    fn remove_by_id(&mut self, id: Id<UiDrawCallDescriptor>) {
        self.draw_calls.retain(|dc| dc.descriptor.id() != id);
        // The Hybrid is dropped here (removed from draw_calls Vec),
        // which will cause the slab to reclaim its memory on the
        // next commit.
    }
}

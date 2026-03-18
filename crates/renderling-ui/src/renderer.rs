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
    compositor::Compositor,
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

    // --- Getters ---

    /// Returns the top-left position in screen pixels.
    pub fn position(&self) -> Vec2 {
        self.inner.get().position
    }

    /// Returns the size in screen pixels.
    pub fn size(&self) -> Vec2 {
        self.inner.get().size
    }

    /// Returns the fill color (RGBA).
    pub fn fill_color(&self) -> Vec4 {
        self.inner.get().fill_color
    }

    /// Returns the per-corner radii.
    pub fn corner_radii(&self) -> Vec4 {
        self.inner.get().corner_radii
    }

    /// Returns the border width in pixels.
    pub fn border_width(&self) -> f32 {
        self.inner.get().border_width
    }

    /// Returns the border color (RGBA).
    pub fn border_color(&self) -> Vec4 {
        self.inner.get().border_color
    }

    /// Returns the gradient descriptor.
    pub fn gradient(&self) -> GradientDescriptor {
        self.inner.get().gradient
    }

    /// Returns the opacity.
    pub fn opacity(&self) -> f32 {
        self.inner.get().opacity
    }

    /// Returns the z-depth.
    pub fn z(&self) -> f32 {
        self.inner.get().z
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

    // --- Getters ---

    /// Returns the center position in screen pixels.
    pub fn center(&self) -> Vec2 {
        let d = self.inner.get();
        d.position + d.size / 2.0
    }

    /// Returns the radius in screen pixels.
    pub fn radius(&self) -> f32 {
        self.inner.get().size.x / 2.0
    }

    /// Returns the fill color (RGBA).
    pub fn fill_color(&self) -> Vec4 {
        self.inner.get().fill_color
    }

    /// Returns the border width in pixels.
    pub fn border_width(&self) -> f32 {
        self.inner.get().border_width
    }

    /// Returns the border color (RGBA).
    pub fn border_color(&self) -> Vec4 {
        self.inner.get().border_color
    }

    /// Returns the gradient descriptor.
    pub fn gradient(&self) -> GradientDescriptor {
        self.inner.get().gradient
    }

    /// Returns the opacity.
    pub fn opacity(&self) -> f32 {
        self.inner.get().opacity
    }

    /// Returns the z-depth.
    pub fn z(&self) -> f32 {
        self.inner.get().z
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

    // --- Getters ---

    /// Returns the center position in screen pixels.
    pub fn center(&self) -> Vec2 {
        let d = self.inner.get();
        d.position + d.size / 2.0
    }

    /// Returns the radii (horizontal, vertical) in screen pixels.
    pub fn radii(&self) -> Vec2 {
        self.inner.get().size / 2.0
    }

    /// Returns the fill color (RGBA).
    pub fn fill_color(&self) -> Vec4 {
        self.inner.get().fill_color
    }

    /// Returns the border width in pixels.
    pub fn border_width(&self) -> f32 {
        self.inner.get().border_width
    }

    /// Returns the border color (RGBA).
    pub fn border_color(&self) -> Vec4 {
        self.inner.get().border_color
    }

    /// Returns the gradient descriptor.
    pub fn gradient(&self) -> GradientDescriptor {
        self.inner.get().gradient
    }

    /// Returns the opacity.
    pub fn opacity(&self) -> f32 {
        self.inner.get().opacity
    }

    /// Returns the z-depth.
    pub fn z(&self) -> f32 {
        self.inner.get().z
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

    // --- Getters ---

    /// Returns the top-left position in screen pixels.
    pub fn position(&self) -> Vec2 {
        self.inner.get().position
    }

    /// Returns the size in screen pixels.
    pub fn size(&self) -> Vec2 {
        self.inner.get().size
    }

    /// Returns the tint color (RGBA).
    pub fn tint(&self) -> Vec4 {
        self.inner.get().fill_color
    }

    /// Returns the opacity.
    pub fn opacity(&self) -> f32 {
        self.inner.get().opacity
    }

    /// Returns the z-depth.
    pub fn z(&self) -> f32 {
        self.inner.get().z
    }
}

// ---------------------------------------------------------------------------
// Path types (behind "path" feature)
// ---------------------------------------------------------------------------

#[cfg(feature = "path")]
mod path {
    use super::*;
    use craballoc::value::HybridArray;
    use lyon::{
        geom,
        math::Angle,
        path::{builder::BorderRadii, traits::PathBuilder, Winding},
        tessellation::{
            BuffersBuilder, FillTessellator, FillVertex, LineCap, LineJoin, StrokeTessellator,
            StrokeVertex, VertexBuffers,
        },
    };
    use renderling::{atlas::shader::AtlasTextureDescriptor, ui_slab::UiVertex};

    fn vec2_to_point(v: impl Into<Vec2>) -> geom::Point<f32> {
        let v = v.into();
        geom::point(v.x, v.y)
    }

    fn vec2_to_vec(v: impl Into<Vec2>) -> geom::Vector<f32> {
        let v = v.into();
        geom::Vector::new(v.x, v.y)
    }

    /// Number of per-vertex attributes (stroke_color[4] + fill_color[4]).
    const NUM_ATTRIBUTES: usize = 8;

    /// Per-vertex attributes passed through lyon's attribute system.
    #[derive(Clone, Copy)]
    struct PathAttributes {
        stroke_color: Vec4,
        fill_color: Vec4,
    }

    impl PathAttributes {
        fn to_array(self) -> [f32; NUM_ATTRIBUTES] {
            let s = self.stroke_color;
            let f = self.fill_color;
            [s.x, s.y, s.z, s.w, f.x, f.y, f.z, f.w]
        }

        fn from_slice(s: &[f32]) -> Self {
            Self {
                stroke_color: Vec4::new(s[0], s[1], s[2], s[3]),
                fill_color: Vec4::new(s[4], s[5], s[6], s[7]),
            }
        }
    }

    /// Stroke rendering options.
    pub struct StrokeConfig {
        /// Line width in pixels.
        pub line_width: f32,
        /// Line cap style.
        pub line_cap: LineCap,
        /// Line join style.
        pub line_join: LineJoin,
    }

    impl Default for StrokeConfig {
        fn default() -> Self {
            Self {
                line_width: 2.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
            }
        }
    }

    /// A builder for constructing 2D vector paths.
    ///
    /// Uses lyon for tessellation. Build a path with `begin`/`line_to`/
    /// `end` commands (or convenience methods like `add_rectangle`,
    /// `add_circle`, etc.), then call `fill()` or `stroke()` to
    /// tessellate and register the result with the renderer.
    ///
    /// ```ignore
    /// let path = ui.path_builder()
    ///     .with_fill_color(Vec4::new(1.0, 0.0, 0.0, 1.0))
    ///     .with_begin(Vec2::new(10.0, 10.0))
    ///     .with_line_to(Vec2::new(100.0, 10.0))
    ///     .with_line_to(Vec2::new(55.0, 80.0))
    ///     .with_end(true)
    ///     .fill(&mut ui);
    /// ```
    pub struct UiPathBuilder {
        inner: lyon::path::BuilderWithAttributes,
        attrs: PathAttributes,
        stroke_config: StrokeConfig,
        /// Atlas texture descriptor ID for image-filled paths.
        fill_image_id: Id<AtlasTextureDescriptor>,
    }

    impl UiPathBuilder {
        pub(crate) fn new() -> Self {
            Self {
                inner: lyon::path::Path::builder_with_attributes(NUM_ATTRIBUTES),
                attrs: PathAttributes {
                    stroke_color: Vec4::ZERO,
                    fill_color: Vec4::ONE,
                },
                stroke_config: StrokeConfig::default(),
                fill_image_id: Id::NONE,
            }
        }

        // --- Color setters ---

        /// Set the fill color for subsequent path commands.
        pub fn set_fill_color(&mut self, color: impl Into<Vec4>) -> &mut Self {
            self.attrs.fill_color = color.into();
            self
        }

        /// Set the fill color (builder).
        pub fn with_fill_color(mut self, color: impl Into<Vec4>) -> Self {
            self.set_fill_color(color);
            self
        }

        /// Set the stroke color for subsequent path commands.
        pub fn set_stroke_color(&mut self, color: impl Into<Vec4>) -> &mut Self {
            self.attrs.stroke_color = color.into();
            self
        }

        /// Set the stroke color (builder).
        pub fn with_stroke_color(mut self, color: impl Into<Vec4>) -> Self {
            self.set_stroke_color(color);
            self
        }

        /// Set stroke options.
        pub fn set_stroke_config(&mut self, config: StrokeConfig) -> &mut Self {
            self.stroke_config = config;
            self
        }

        /// Set stroke options (builder).
        pub fn with_stroke_config(mut self, config: StrokeConfig) -> Self {
            self.stroke_config = config;
            self
        }

        /// Set an image to fill the path with.
        ///
        /// The image is sampled using UVs computed from each vertex's
        /// position relative to the path's bounding box (0..1 range).
        /// The vertex color acts as a tint/modulator.
        ///
        /// The `AtlasTexture` should be obtained from
        /// [`UiRenderer::upload_image`].
        pub fn set_fill_image(&mut self, texture: &AtlasTexture) -> &mut Self {
            self.fill_image_id = texture.id();
            self
        }

        /// Set an image to fill the path with (builder).
        pub fn with_fill_image(mut self, texture: &AtlasTexture) -> Self {
            self.set_fill_image(texture);
            self
        }

        // --- Path commands ---

        /// Begin a new sub-path at the given point.
        pub fn begin(&mut self, at: impl Into<Vec2>) -> &mut Self {
            let _ = self.inner.begin(vec2_to_point(at), &self.attrs.to_array());
            self
        }

        /// Begin a new sub-path (builder).
        pub fn with_begin(mut self, at: impl Into<Vec2>) -> Self {
            self.begin(at);
            self
        }

        /// End the current sub-path, optionally closing it.
        pub fn end(&mut self, close: bool) -> &mut Self {
            self.inner.end(close);
            self
        }

        /// End the current sub-path (builder).
        pub fn with_end(mut self, close: bool) -> Self {
            self.end(close);
            self
        }

        /// Add a line segment to the given point.
        pub fn line_to(&mut self, to: impl Into<Vec2>) -> &mut Self {
            let _ = self
                .inner
                .line_to(vec2_to_point(to), &self.attrs.to_array());
            self
        }

        /// Add a line segment (builder).
        pub fn with_line_to(mut self, to: impl Into<Vec2>) -> Self {
            self.line_to(to);
            self
        }

        /// Add a quadratic Bezier curve.
        pub fn quadratic_bezier_to(
            &mut self,
            ctrl: impl Into<Vec2>,
            to: impl Into<Vec2>,
        ) -> &mut Self {
            let _ = self.inner.quadratic_bezier_to(
                vec2_to_point(ctrl),
                vec2_to_point(to),
                &self.attrs.to_array(),
            );
            self
        }

        /// Add a quadratic Bezier curve (builder).
        pub fn with_quadratic_bezier_to(
            mut self,
            ctrl: impl Into<Vec2>,
            to: impl Into<Vec2>,
        ) -> Self {
            self.quadratic_bezier_to(ctrl, to);
            self
        }

        /// Add a cubic Bezier curve.
        pub fn cubic_bezier_to(
            &mut self,
            ctrl1: impl Into<Vec2>,
            ctrl2: impl Into<Vec2>,
            to: impl Into<Vec2>,
        ) -> &mut Self {
            let _ = self.inner.cubic_bezier_to(
                vec2_to_point(ctrl1),
                vec2_to_point(ctrl2),
                vec2_to_point(to),
                &self.attrs.to_array(),
            );
            self
        }

        /// Add a cubic Bezier curve (builder).
        pub fn with_cubic_bezier_to(
            mut self,
            ctrl1: impl Into<Vec2>,
            ctrl2: impl Into<Vec2>,
            to: impl Into<Vec2>,
        ) -> Self {
            self.cubic_bezier_to(ctrl1, ctrl2, to);
            self
        }

        // --- Convenience shapes ---

        /// Add an axis-aligned rectangle.
        pub fn add_rectangle(&mut self, min: impl Into<Vec2>, max: impl Into<Vec2>) -> &mut Self {
            let min = min.into();
            let max = max.into();
            let rect = lyon::geom::Box2D::new(vec2_to_point(min), vec2_to_point(max));
            self.inner
                .add_rectangle(&rect, Winding::Positive, &self.attrs.to_array());
            self
        }

        /// Add a rectangle (builder).
        pub fn with_rectangle(mut self, min: impl Into<Vec2>, max: impl Into<Vec2>) -> Self {
            self.add_rectangle(min, max);
            self
        }

        /// Add a rounded rectangle.
        pub fn add_rounded_rectangle(
            &mut self,
            min: impl Into<Vec2>,
            max: impl Into<Vec2>,
            top_left: f32,
            top_right: f32,
            bottom_left: f32,
            bottom_right: f32,
        ) -> &mut Self {
            let min = min.into();
            let max = max.into();
            let rect = lyon::geom::Box2D::new(vec2_to_point(min), vec2_to_point(max));
            let radii = BorderRadii {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            };
            self.inner.add_rounded_rectangle(
                &rect,
                &radii,
                Winding::Positive,
                &self.attrs.to_array(),
            );
            self
        }

        /// Add a rounded rectangle (builder).
        pub fn with_rounded_rectangle(
            mut self,
            min: impl Into<Vec2>,
            max: impl Into<Vec2>,
            top_left: f32,
            top_right: f32,
            bottom_left: f32,
            bottom_right: f32,
        ) -> Self {
            self.add_rounded_rectangle(min, max, top_left, top_right, bottom_left, bottom_right);
            self
        }

        /// Add a circle.
        pub fn add_circle(&mut self, center: impl Into<Vec2>, radius: f32) -> &mut Self {
            self.inner.add_circle(
                vec2_to_point(center),
                radius,
                Winding::Positive,
                &self.attrs.to_array(),
            );
            self
        }

        /// Add a circle (builder).
        pub fn with_circle(mut self, center: impl Into<Vec2>, radius: f32) -> Self {
            self.add_circle(center, radius);
            self
        }

        /// Add an ellipse.
        pub fn add_ellipse(
            &mut self,
            center: impl Into<Vec2>,
            radii: impl Into<Vec2>,
            rotation: f32,
        ) -> &mut Self {
            let radii = radii.into();
            self.inner.add_ellipse(
                vec2_to_point(center),
                vec2_to_vec(radii),
                Angle::radians(rotation),
                Winding::Positive,
                &self.attrs.to_array(),
            );
            self
        }

        /// Add an ellipse (builder).
        pub fn with_ellipse(
            mut self,
            center: impl Into<Vec2>,
            radii: impl Into<Vec2>,
            rotation: f32,
        ) -> Self {
            self.add_ellipse(center, radii, rotation);
            self
        }

        /// Add a closed polygon from a series of points.
        pub fn add_polygon(&mut self, points: &[Vec2]) -> &mut Self {
            let pts: Vec<geom::Point<f32>> = points.iter().map(|p| vec2_to_point(*p)).collect();
            self.inner.add_polygon(
                lyon::path::Polygon {
                    points: &pts,
                    closed: true,
                },
                &self.attrs.to_array(),
            );
            self
        }

        /// Add a polygon (builder).
        pub fn with_polygon(mut self, points: &[Vec2]) -> Self {
            self.add_polygon(points);
            self
        }

        // --- Tessellation ---

        /// Tessellate the path as a filled shape and register it with the
        /// renderer. Consumes the builder.
        pub fn fill(self, renderer: &mut UiRenderer) -> UiPath {
            let fill_image_id = self.fill_image_id;
            let path = self.inner.build();
            let mut geometry = VertexBuffers::<UiVertex, u32>::new();
            let mut tessellator = FillTessellator::new();

            tessellator
                .tessellate_path(
                    path.as_slice(),
                    &Default::default(),
                    &mut BuffersBuilder::new(&mut geometry, |mut vertex: FillVertex| {
                        let p = vertex.position();
                        let attrs = PathAttributes::from_slice(vertex.interpolated_attributes());
                        UiVertex {
                            position: Vec2::new(p.x, p.y),
                            uv: Vec2::ZERO,
                            color: attrs.fill_color,
                        }
                    }),
                )
                .expect("fill tessellation failed");

            // If an image fill is set, compute UVs from the bounding box.
            if !fill_image_id.is_none() {
                Self::compute_bounding_box_uvs(&mut geometry);
            }

            Self::upload(renderer, &geometry, fill_image_id)
        }

        /// Tessellate the path as a stroked outline and register it with
        /// the renderer. Consumes the builder.
        pub fn stroke(self, renderer: &mut UiRenderer) -> UiPath {
            let fill_image_id = self.fill_image_id;
            let path = self.inner.build();
            let mut geometry = VertexBuffers::<UiVertex, u32>::new();
            let mut tessellator = StrokeTessellator::new();

            let opts = lyon::tessellation::StrokeOptions::default()
                .with_line_width(self.stroke_config.line_width)
                .with_line_cap(self.stroke_config.line_cap)
                .with_line_join(self.stroke_config.line_join);

            tessellator
                .tessellate_path(
                    path.as_slice(),
                    &opts,
                    &mut BuffersBuilder::new(&mut geometry, |mut vertex: StrokeVertex| {
                        let p = vertex.position();
                        let attrs = PathAttributes::from_slice(vertex.interpolated_attributes());
                        UiVertex {
                            position: Vec2::new(p.x, p.y),
                            uv: Vec2::ZERO,
                            color: attrs.stroke_color,
                        }
                    }),
                )
                .expect("stroke tessellation failed");

            // If an image fill is set, compute UVs from the bounding box.
            if !fill_image_id.is_none() {
                Self::compute_bounding_box_uvs(&mut geometry);
            }

            Self::upload(renderer, &geometry, fill_image_id)
        }

        /// Compute UVs from the bounding box of the tessellated vertices.
        ///
        /// Maps each vertex position into 0..1 UV space relative to the
        /// axis-aligned bounding box of all vertices.
        fn compute_bounding_box_uvs(geometry: &mut VertexBuffers<UiVertex, u32>) {
            if geometry.vertices.is_empty() {
                return;
            }
            let mut min = Vec2::splat(f32::INFINITY);
            let mut max = Vec2::splat(f32::NEG_INFINITY);
            for v in &geometry.vertices {
                min = min.min(v.position);
                max = max.max(v.position);
            }
            let extent = max - min;
            let inv_extent = Vec2::new(
                if extent.x > 0.0 { 1.0 / extent.x } else { 0.0 },
                if extent.y > 0.0 { 1.0 / extent.y } else { 0.0 },
            );
            for v in &mut geometry.vertices {
                v.uv = (v.position - min) * inv_extent;
            }
        }

        /// De-index the tessellated geometry, write vertices to the slab,
        /// and create a draw call.
        fn upload(
            renderer: &mut UiRenderer,
            geometry: &VertexBuffers<UiVertex, u32>,
            atlas_texture_id: Id<AtlasTextureDescriptor>,
        ) -> UiPath {
            // De-index: expand indexed triangles to flat vertex list.
            let expanded: Vec<UiVertex> = geometry
                .indices
                .iter()
                .map(|&i| geometry.vertices[i as usize])
                .collect();

            let vertex_count = expanded.len() as u32;
            let vertex_array = renderer.slab.new_array(expanded);
            let vertex_offset = vertex_array.array().starting_index() as u32;

            let mut desc = renderer.default_descriptor(UiElementType::Path);
            desc.atlas_descriptor_id = Id::new(vertex_offset);
            desc.atlas_texture_id = atlas_texture_id;
            let hybrid = renderer.slab.new_value(desc);
            renderer.draw_calls.push(DrawCall {
                descriptor: hybrid.clone(),
                vertex_count,
            });

            UiPath {
                inner: hybrid,
                _vertices: vertex_array,
            }
        }
    }

    /// A live handle to a tessellated path element in the renderer.
    ///
    /// **Dropping this handle does NOT remove the path** — call
    /// [`UiRenderer::remove_path`] explicitly.
    pub struct UiPath {
        inner: Hybrid<UiDrawCallDescriptor>,
        /// Kept alive so the slab doesn't reclaim the vertex data.
        _vertices: HybridArray<UiVertex>,
    }

    impl std::fmt::Debug for UiPath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UiPath")
                .field("inner", &self.inner)
                .finish_non_exhaustive()
        }
    }

    impl UiPath {
        /// Returns the slab [`Id`] of the underlying descriptor.
        pub fn id(&self) -> Id<UiDrawCallDescriptor> {
            self.inner.id()
        }

        /// Returns a copy of the underlying descriptor.
        pub fn descriptor(&self) -> UiDrawCallDescriptor {
            self.inner.get()
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

        // --- Getters ---

        /// Returns the opacity.
        pub fn opacity(&self) -> f32 {
            self.inner.get().opacity
        }

        /// Returns the z-depth.
        pub fn z(&self) -> f32 {
            self.inner.get().z
        }
    }
}

#[cfg(feature = "path")]
pub use path::{StrokeConfig, UiPath, UiPathBuilder};

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

        // --- Getters ---

        /// Returns the opacity (reads from the first glyph, or 1.0 if
        /// empty).
        pub fn opacity(&self) -> f32 {
            self.glyph_descriptors
                .first()
                .map(|h| h.get().opacity)
                .unwrap_or(1.0)
        }

        /// Returns the z-depth (reads from the first glyph, or 0.0 if
        /// empty).
        pub fn z(&self) -> f32 {
            self.glyph_descriptors
                .first()
                .map(|h| h.get().z)
                .unwrap_or(0.0)
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
    /// Non-MSAA intermediate texture for overlay compositing.
    /// Used when `background_color` is `None` and MSAA is active:
    /// the MSAA texture resolves here, then the compositor blends
    /// this onto the caller's target view.
    overlay_texture: Option<wgpu::TextureView>,
    /// Compositor for alpha-blending the overlay texture onto the
    /// final target.
    compositor: Compositor,

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
        let default_msaa = 4;
        let pipeline = Self::create_pipeline(device, &bindgroup_layout, format, default_msaa);
        let msaa_texture = Some(Self::create_msaa_texture(
            device,
            format,
            size,
            default_msaa,
        ));
        let overlay_texture = Some(Self::create_overlay_texture(device, format, size));
        let compositor = Compositor::new(device, format);

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
            msaa_sample_count: default_msaa,
            format,
            msaa_texture,
            overlay_texture,
            compositor,
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

    /// Set the MSAA sample count (builder).
    ///
    /// Higher values produce smoother edges. Common values are 1 (off)
    /// and 4 (default). The pipeline and MSAA texture are recreated.
    pub fn with_msaa_sample_count(mut self, count: u32) -> Self {
        self.msaa_sample_count = count;
        let device = self.slab.device();
        self.pipeline = Self::create_pipeline(device, &self.bindgroup_layout, self.format, count);
        if count > 1 {
            self.msaa_texture = Some(Self::create_msaa_texture(
                device,
                self.format,
                self.viewport_size,
                count,
            ));
            self.overlay_texture = Some(Self::create_overlay_texture(
                device,
                self.format,
                self.viewport_size,
            ));
        } else {
            self.msaa_texture = None;
            self.overlay_texture = None;
        }
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
                self.overlay_texture = Some(Self::create_overlay_texture(
                    self.slab.device(),
                    self.format,
                    size,
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
        desc.atlas_texture_id = atlas_texture.id();
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

    /// Upload an image to the atlas without creating a draw call.
    ///
    /// Returns the [`AtlasTexture`] handle, which can be passed to
    /// [`UiPathBuilder::with_fill_image`] for image-filled paths
    /// or used for other custom purposes.
    ///
    /// ```ignore
    /// let atlas_img = AtlasImage::from_path("icon.png").unwrap();
    /// let tex = ui.upload_image(atlas_img);
    /// let _path = ui.path_builder()
    ///     .with_fill_image(&tex)
    ///     .with_fill_color(Vec4::ONE)
    ///     .with_circle(Vec2::new(50.0, 50.0), 30.0)
    ///     .fill(&mut ui);
    /// ```
    pub fn upload_image(&mut self, image: impl Into<AtlasImage>) -> AtlasTexture {
        let image = image.into();
        let atlas_texture = self
            .atlas
            .add_image(&image)
            .expect("failed to add image to atlas");

        // Update the viewport with the (possibly new) atlas size.
        let atlas_extent = self.atlas.get_size();
        self.viewport.modify(|v| {
            v.atlas_size = UVec2::new(atlas_extent.width, atlas_extent.height);
        });

        atlas_texture
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
            desc.atlas_texture_id = glyph_atlas_hybrid.id();

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

    /// Create a new path builder for constructing vector paths.
    ///
    /// Use the builder's methods to define shapes, then call `.fill()`
    /// or `.stroke()` to tessellate and register the path.
    ///
    /// ```ignore
    /// let path = ui.path_builder()
    ///     .with_fill_color(Vec4::new(1.0, 0.0, 0.0, 1.0))
    ///     .with_circle(Vec2::new(50.0, 50.0), 30.0)
    ///     .fill(&mut ui);
    /// ```
    #[cfg(feature = "path")]
    pub fn path_builder(&self) -> UiPathBuilder {
        UiPathBuilder::new()
    }

    /// Remove a path element by its handle.
    #[cfg(feature = "path")]
    pub fn remove_path(&mut self, element: &UiPath) {
        self.remove_by_id(element.id());
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

        let is_overlay = self.background_color.is_none();
        let use_msaa = self.msaa_sample_count > 1;

        // Determine load op, color attachment, and resolve target.
        //
        // Overlay + MSAA: clear MSAA to transparent, resolve to
        //   intermediate overlay texture, then compositor blends onto
        //   the caller's view.
        // Overlay + no MSAA: load existing view content, render
        //   directly (alpha blending preserves the scene).
        // Standalone: clear to background color, resolve (or render)
        //   directly to the caller's view.
        let load_op = if let Some(bg) = self.background_color {
            wgpu::LoadOp::Clear(wgpu::Color {
                r: bg.x as f64,
                g: bg.y as f64,
                b: bg.z as f64,
                a: bg.w as f64,
            })
        } else if use_msaa {
            // Overlay + MSAA: clear the MSAA texture to transparent
            // so non-UI pixels resolve as fully transparent.
            wgpu::LoadOp::Clear(wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            })
        } else {
            wgpu::LoadOp::Load
        };

        let (color_view, resolve_target) = if use_msaa {
            if let Some(msaa_view) = &self.msaa_texture {
                if is_overlay {
                    // Overlay: resolve to intermediate texture
                    // (NOT the caller's view, which would overwrite
                    // the 3D scene).
                    let resolve = self.overlay_texture.as_ref().unwrap();
                    (msaa_view as &wgpu::TextureView, Some(resolve))
                } else {
                    // Standalone: resolve directly to the caller's
                    // view.
                    (msaa_view as &wgpu::TextureView, Some(view))
                }
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

        // Overlay + MSAA: alpha-blend the resolved UI texture onto
        // the caller's view, preserving the 3D scene underneath.
        if is_overlay && use_msaa {
            if let Some(overlay) = &self.overlay_texture {
                self.compositor.composite(device, queue, overlay, view);
            }
        }
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
            atlas_texture_id: Id::NONE,
            atlas_descriptor_id: Id::NONE,
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

    /// Create a non-MSAA intermediate texture for overlay compositing.
    ///
    /// When the UI is rendered as an overlay (no background clear) with
    /// MSAA enabled, the MSAA texture resolves into this intermediate
    /// texture, which is then alpha-blended onto the final target by
    /// the compositor.
    fn create_overlay_texture(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        size: UVec2,
    ) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("renderling-ui-overlay"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
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

//! Shared types for the 2D/UI rendering pipeline.
//!
//! These types are used by both the CPU (renderling-ui crate) and the GPU
//! (shader entry points in this crate). They are stored in a GPU slab buffer
//! and read by the UI vertex and fragment shaders.

use crabslab::SlabItem;
use glam::{Mat4, UVec2, Vec2, Vec4};

pub mod shader;


/// Identifies what kind of UI element is being rendered.
///
/// Used by the fragment shader to select the appropriate SDF / sampling logic.
#[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
#[repr(u32)]
pub enum UiElementType {
    /// A rectangle (optionally rounded).
    #[default]
    Rectangle = 0,
    /// A circle.
    Circle = 1,
    /// An ellipse.
    Ellipse = 2,
    /// A textured quad (atlas texture sampling).
    Image = 3,
    /// A text glyph quad (glyph atlas sampling).
    TextGlyph = 4,
    /// A pre-tessellated path triangle (uses vertex color directly).
    Path = 5,
}

impl UiElementType {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::Rectangle,
            1 => Self::Circle,
            2 => Self::Ellipse,
            3 => Self::Image,
            4 => Self::TextGlyph,
            5 => Self::Path,
            _ => Self::Rectangle,
        }
    }
}

/// Identifies the type of gradient fill.
#[derive(Clone, Copy, Default, PartialEq, core::fmt::Debug)]
#[repr(u32)]
pub enum GradientType {
    /// No gradient; use solid fill color.
    #[default]
    None = 0,
    /// Linear gradient from `start` to `end`.
    Linear = 1,
    /// Radial gradient from `center` outward.
    Radial = 2,
}

impl GradientType {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::None,
            1 => Self::Linear,
            2 => Self::Radial,
            _ => Self::None,
        }
    }
}

/// Describes a gradient fill for a UI element.
///
/// Stored on the GPU slab.
#[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
pub struct GradientDescriptor {
    /// The type of gradient (None, Linear, Radial).
    pub gradient_type: u32,
    /// For linear: start point (in element-local 0..1 space).
    /// For radial: center point.
    pub start: Vec2,
    /// For linear: end point.
    /// For radial: unused.
    pub end: Vec2,
    /// For radial: the radius. For linear: unused.
    pub radius: f32,
    /// Color at the start (or center for radial).
    pub color_start: Vec4,
    /// Color at the end (or edge for radial).
    pub color_end: Vec4,
}

/// Per-vertex data for the 2D/UI pipeline.
///
/// This is a lightweight vertex type (32 bytes) compared to the 3D
/// `Vertex` (~160 bytes).
#[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
pub struct UiVertex {
    /// Screen-space position (x, y).
    pub position: Vec2,
    /// UV coordinates (for texture sampling or SDF evaluation).
    pub uv: Vec2,
    /// Per-vertex RGBA color.
    pub color: Vec4,
}

impl UiVertex {
    pub fn with_position(mut self, position: impl Into<Vec2>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_uv(mut self, uv: impl Into<Vec2>) -> Self {
        self.uv = uv.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<Vec4>) -> Self {
        self.color = color.into();
        self
    }
}

/// Describes a single 2D UI element on the GPU.
///
/// This is the per-instance data stored in the GPU slab.
/// The vertex shader reads this to generate quad corners,
/// and the fragment shader reads it to evaluate SDF shapes,
/// gradients, textures, etc.
#[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
pub struct UiDrawCallDescriptor {
    /// The type of element (Rectangle, Circle, Ellipse, Image, TextGlyph,
    /// Path).
    pub element_type: UiElementType,
    /// Position of the element's top-left corner in screen space.
    pub position: Vec2,
    /// Size of the element in screen pixels (width, height).
    pub size: Vec2,
    /// Per-corner radii for rounded rectangles (top-left, top-right,
    /// bottom-right, bottom-left).
    pub corner_radii: Vec4,
    /// Border width in pixels. 0 means no border.
    pub border_width: f32,
    /// Border color (RGBA).
    pub border_color: Vec4,
    /// Fill color (RGBA). Used when gradient_type is None.
    pub fill_color: Vec4,
    /// Gradient fill descriptor.
    pub gradient: GradientDescriptor,
    /// ID of the atlas texture descriptor on the slab (for Image elements).
    /// Set to `Id::NONE` when unused.
    pub atlas_texture_id: u32,
    /// ID of the atlas descriptor on the slab.
    /// Set to `Id::NONE` when unused.
    pub atlas_descriptor_id: u32,
    /// Scissor/clip rectangle (x, y, width, height).
    /// Elements outside this rect are clipped. Set to (0,0, viewport_w,
    /// viewport_h) for no clipping.
    pub clip_rect: Vec4,
    /// Element opacity (0.0 = fully transparent, 1.0 = fully opaque).
    /// Multiplied with the final alpha.
    pub opacity: f32,
    /// Z-depth for sorting (painter's algorithm). Lower values are drawn
    /// first (further back).
    pub z: f32,
}

/// Camera/viewport descriptor for the 2D UI pipeline.
///
/// Contains the orthographic projection matrix, viewport dimensions,
/// and atlas texture dimensions.
#[derive(Clone, Copy, Default, PartialEq, SlabItem, core::fmt::Debug)]
pub struct UiViewport {
    /// Orthographic projection matrix (typically top-left origin, +Y down).
    pub projection: Mat4,
    /// Viewport size in pixels.
    pub size: UVec2,
    /// Atlas texture size in pixels.
    pub atlas_size: UVec2,
}

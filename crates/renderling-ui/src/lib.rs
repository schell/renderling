//! Lightweight 2D/UI renderer for renderling.
//!
//! This crate provides a dedicated 2D rendering pipeline that is separate
//! from renderling's 3D PBR pipeline. It features:
//!
//! - SDF-based shape rendering (rectangles, rounded rectangles, circles,
//!   ellipses) with anti-aliased edges
//! - Gradient fills (linear and radial)
//! - Texture/image rendering via the renderling atlas system
//! - Text rendering via `glyph_brush` (behind the `text` feature)
//! - Vector path rendering via `lyon` tessellation (behind the `path` feature)
//! - A lightweight vertex format (32 bytes vs ~160 bytes for 3D)
//! - Minimal GPU bindings (3 vs 13 for 3D)
//!
//! # Quick Start
//!
//! ```ignore
//! use renderling::context::Context;
//! use renderling_ui::UiRenderer;
//!
//! let ctx = futures_lite::future::block_on(Context::headless(800, 600));
//! let mut ui = UiRenderer::new(&ctx);
//!
//! // Add a rounded rectangle
//! let _rect = ui.add_rect()
//!     .with_position(glam::Vec2::new(10.0, 10.0))
//!     .with_size(glam::Vec2::new(200.0, 100.0))
//!     .with_corner_radii(glam::Vec4::splat(8.0))
//!     .with_fill_color(glam::Vec4::new(0.2, 0.3, 0.8, 1.0));
//!
//! let frame = ctx.get_next_frame().unwrap();
//! ui.render(&frame.view());
//! frame.present();
//! ```

mod renderer;
#[cfg(test)]
mod test;

// Re-export key types from renderling that users will need.
pub use renderling::{
    atlas::{AtlasImage, AtlasTexture},
    context::Context,
    glam,
    ui_slab::{
        GradientDescriptor, GradientType, UiDrawCallDescriptor, UiElementType, UiVertex, UiViewport,
    },
};

// Re-export our own types.
pub use renderer::{UiCircle, UiEllipse, UiImage, UiRect, UiRenderer};

// Re-export text types (behind "text" feature).
#[cfg(feature = "text")]
pub use renderer::{FontArc, FontId, Section, Text, UiText};

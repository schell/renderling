//! User interface rendering.
//!
//! # Getting Started
//! First we create a context, then we create a [`Ui`], which we can use to
//! "stage" our paths, text, etc:
//!
//! ```rust
//! use renderling::ui::prelude::*;
//! use glam::Vec2;
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
#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod sdf;

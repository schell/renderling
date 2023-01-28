//! A collection of renderers with a focus on simplicity and ease of use.
//! Backed by WGPU render pipelines and simple types for marshalling data to the GPU.
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
//!
//! # Features
//! Features are used to enable specific render pipelines and managed renderers.
//! Without any features, only [`renderling_core`] is exported.
//!
//! # renderlings 🍖
//! Individual renderers are called "renderlings" for maximum cuteness.
//! Renderlings manage their own resources and come in a couple flavors depending on the shader used.
//!
//! * [`ui::UiRenderling`] - used for simple colored or textured meshes, mostly for rendering user interfaces.
//!
//! ## shaders
//! You can also use the re-exported shaders and manage your own resources for maximum flexibility.
//!
//! ### forward
//! A simple forward shader that supports blinn-phong material shading and a few light sources.
//!
//! ### ui
//! A simple interface shader that supports color meshes and text by using a custom blend uniform.
pub use renderling_core::*;

#[cfg(feature = "forward")]
pub mod forward {
    //! A simple forward shader that supports blinn-phong material shading and a few light sources.
    //!
    //! See [renderling_forward]'s module documentation for more info.
    pub use renderling_forward::*;
}

#[cfg(feature = "ui")]
pub mod ui;

//#[cfg(feature = "forward")]
//pub mod forward;

mod mesh;
mod projection;
mod resources;
mod state;
mod texture;
mod transform;

pub use mesh::*;
pub use projection::*;
pub use state::*;
pub use texture::*;
pub use transform::*;

pub mod math;

#[cfg(test)]
mod img_diff;

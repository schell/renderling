//! A collection of renderers (callend "renderlings") with a focus on simplicity and ease of use.
//! Backed by WebGPU render pipelines and simple types for marshalling data to the GPU.
//!
//! # WARNING
//! This is very much a work in progress.
//! YMMV.
//! PRs are very welcomed :)
//!
//! # renderlings 🍖
//! Individual renderers are called "renderlings" for maximum cuteness.
//! Renderlings manage their own resources and come in a couple flavors depending on the shader used.
//!
//! ## Features
//! Features are used to enable specific renderlings, by default all renderlings are enabled.
//!
//! * **ui**
//!   - simple simple diffuse material
//!   - colored or textured mesh attributes
//!   - mostly for rendering user interfaces
//! * **forward**
//!   - blinn-phong material
//!   - textured mesh attribute
//!   - maximum 64 point, 32 spot and 8 directional lights
//!
//! ## Raw shaders
//! You can also use the [shaders module](crate::shaders) without renderlings and manage your own resources for maximum flexibility.
pub use renderling_core::*;

#[cfg(feature = "forward")]
mod forward;
#[cfg(feature = "forward")]
pub use forward::*;

#[cfg(feature = "ui")]
mod ui;
#[cfg(feature = "ui")]
pub use ui::*;

pub mod shaders {
    //! Raw `wgpu` shaders.

    #[cfg(feature = "forward")]
    pub mod forward {
        //! Forward shader `wgpu` types and operations.
        pub use renderling_forward::*;
    }
    #[cfg(feature = "ui")]
    pub mod ui {
        //! User interface shader `wgpu` types and operations.
        pub use renderling_ui::*;
    }
}

mod camera;
mod light;
mod material;
mod mesh;
mod pipeline;
mod resources;
mod renderer;
mod state;
mod texture;
mod transform;

pub use camera::*;
pub use light::*;
pub use material::*;
pub use mesh::*;
pub use pipeline::*;
pub use renderer::*;
pub use state::*;
pub use texture::*;
pub use transform::*;

pub mod math;

#[cfg(test)]
mod img_diff;

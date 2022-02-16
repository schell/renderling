//! Renderling
//!
//! A collection of WGPU render pipelines and simple types for marshalling data to the GPU.
//!
//! # WARNING
//! This is very much a work in progress. YMMV. PRs are very welcomed :)
//!
//! # Features
//! Features are used to enable specific render pipelines. Without any features, only [`renderling_core`]
//! is exported.
//!
//! ## forward
//! A simple forward shader that supports blinn-phong material shading and a few light sources.
//!
//! ## ui
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
pub mod ui {
    //! A simple interface shader that supports color meshes and text by using a custom blend uniform.
    //!
    //! See [renderling_ui]'s module documentation for more info.
    pub use renderling_ui::*;
}

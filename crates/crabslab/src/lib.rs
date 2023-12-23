//! Creating and crafting a tasty slab of memory.

mod array;
pub use array::*;

mod id;
pub use id::*;

mod slab;
pub use slab::*;

#[cfg(feature = "wgpu")]
mod wgpu_slab;
#[cfg(feature = "wgpu")]
pub use wgpu_slab::*;

pub use crabslab_derive::SlabItem;

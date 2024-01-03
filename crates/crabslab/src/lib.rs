#![cfg_attr(target_arch = "spirv", no_std)]
//! Creating and crafting a tasty slab of memory.
#![doc = include_str!("../README.md")]

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

#[cfg(doctest)]
pub mod doctest {
    #[doc = include_str!("../README.md")]
    pub struct ReadmeDoctests;
}

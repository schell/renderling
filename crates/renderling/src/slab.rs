// SPDX-FileCopyrightText: 2024 Schell Scivally <efsubenovex@gmail.com>>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! GPU and CPU slab allocation.
//!
//! Re-exports [`Array`], [`Id`], [`Slab`] and [`SlabItem`] from [`crabslab`](https://docs.rs/crabslab/latest/crabslab/).
//!
//! User types can automatically derive `SlabItem` in most cases. It is required
//! that your type's fields all implement `SlabItem` and `crabslab` must be in
//! scope.
//!
//! ```
//! use renderling::slab::SlabItem;
//!
//! #[derive(Clone, Copy, SlabItem)]
//! struct UserData {
//!     pos: (f32, f32, f32),
//!     acc: (f32, f32, f32),
//! }
//! ```
pub use crabslab::{Array, Id, Slab, SlabItem};

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

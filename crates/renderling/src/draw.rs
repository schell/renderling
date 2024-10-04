//! Handles queueing draw calls.
//!
//! [`DrawCalls`] is used to maintain the list of all staged [`Renderlet`]s.
//! It also performs frustum culling and issues draw calls during
//! [`Stage::render`].
use crabslab::{Id, SlabItem};

use crate::stage::Renderlet;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Argument buffer layout for draw_indirect commands.
#[derive(Clone, Copy, Default, SlabItem)]
pub struct DrawIndirectArgs {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: Id<Renderlet>,
}

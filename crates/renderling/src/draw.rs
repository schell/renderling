//! Handles queueing draw calls.
//!
//! [`DrawCalls`] is used to maintain the list of all staged [`Renderlet`]s.
//! It also performs frustum culling and issues draw calls during
//! [`Stage::render`].
use crabslab::SlabItem;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

/// Argument buffer layout for draw_indirect commands.
#[repr(C)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[derive(Clone, Copy, Default, SlabItem)]
pub struct DrawIndirectArgs {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

//! Handles queueing draw calls.
//!
//! [`DrawCalls`] is used to maintain the list of all staged
//! [`PrimitiveDescriptor`](crate::primitive::shader::PrimitiveDescriptor)s.
//! It also performs frustum culling and issues draw calls during
//! [`Stage::render`](crate::stage::Stage::render).
use crabslab::SlabItem;

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

/// Argument buffer layout for draw_indirect commands.
#[repr(C)]
#[cfg_attr(cpu, derive(bytemuck::Pod, bytemuck::Zeroable))]
#[derive(Clone, Copy, Default, SlabItem, core::fmt::Debug)]
pub struct DrawIndirectArgs {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

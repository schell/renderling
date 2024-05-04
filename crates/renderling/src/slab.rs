//! GPU and CPU slab allocation.
pub use crabslab::*;

#[cfg(not(target_arch = "spirv"))]
mod cpu;
#[cfg(not(target_arch = "spirv"))]
pub use cpu::*;

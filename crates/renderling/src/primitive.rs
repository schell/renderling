//! Mesh primitives

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

//! Atlas images, used for materials. CPU and GPU.

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

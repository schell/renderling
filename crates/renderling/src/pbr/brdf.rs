//! BRDF computation.
//!
//! Helpers for computing (and holding onto) a Bidirectional Reflectance Distribution Function.

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

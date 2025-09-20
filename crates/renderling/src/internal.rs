//! Internal types and functions.
//!
//! ## Note
//! The types and functions exposed by this module are used internally, and
//! are _not_ required to be used by users of this library.
//!
//! They are public here because they are needed for integration tests, and
//! on the off-chance that somebody wants to build something with them.

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

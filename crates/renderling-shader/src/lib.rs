//! Shader code for `renderling`.
//!
//! This gathers the various `renderling-shader` crates together and
//! exports them as a pair of uber shaders.
#![cfg_attr(target_arch = "spirv", no_std)]
#![deny(clippy::disallowed_methods)]

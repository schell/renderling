//! Image based lighting
//!
//! For more info on image based lighting, see <https://learnopengl.com/PBR/IBL/Diffuse-irradiance>.

#[cfg(cpu)]
mod cpu;
#[cfg(cpu)]
pub use cpu::*;

pub mod shader;

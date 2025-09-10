//! Shader resources for image based lighting.

pub mod diffuse_irradiance;
#[cfg(not(target_arch = "spirv"))]
pub mod prefiltered_environment;

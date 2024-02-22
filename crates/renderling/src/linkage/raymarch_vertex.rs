
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [sdf::raymarch::raymarch_vertex](crate::sdf::raymarch::raymarch_vertex).
//!
//! **source path**: `crates/renderling/src/linkage/sdf-raymarch-raymarch_vertex.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(raymarch_vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module =
        device.create_shader_module(wgpu::include_spirv!("sdf-raymarch-raymarch_vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(raymarch_vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::raymarch::raymarch_vertex",
    }
}

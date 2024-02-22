
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [stage::stage_fragment](crate::stage::stage_fragment).
//!
//! **source path**: `crates/renderling/src/linkage/stage-stage_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(stage_fragment));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("stage-stage_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(stage_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "stage::stage_fragment",
    }
}

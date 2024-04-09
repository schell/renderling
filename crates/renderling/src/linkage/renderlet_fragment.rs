
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [stage::renderlet_fragment](crate::stage::renderlet_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/stage-renderlet_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(renderlet_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("stage-renderlet_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(renderlet_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "stage::renderlet_fragment",
    }
}

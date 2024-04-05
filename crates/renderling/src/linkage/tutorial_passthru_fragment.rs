
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_passthru_fragment](crate::tutorial::tutorial_passthru_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_passthru_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(tutorial_passthru_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "tutorial-tutorial_passthru_fragment.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(tutorial_passthru_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "tutorial::tutorial_passthru_fragment",
    }
}


//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_slabbed_renderlet](crate::tutorial::tutorial_slabbed_renderlet).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_slabbed_renderlet.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(tutorial_slabbed_renderlet)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "tutorial-tutorial_slabbed_renderlet.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(tutorial_slabbed_renderlet)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "tutorial::tutorial_slabbed_renderlet",
    }
}

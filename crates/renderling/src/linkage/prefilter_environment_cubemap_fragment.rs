
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::prefilter_environment_cubemap_fragment](crate::convolution::prefilter_environment_cubemap_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! convolution-prefilter_environment_cubemap_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(prefilter_environment_cubemap_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "convolution-prefilter_environment_cubemap_fragment.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(prefilter_environment_cubemap_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::prefilter_environment_cubemap_fragment",
    }
}

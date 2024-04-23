
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [bloom::bloom_downsample_fragment](crate::bloom::bloom_downsample_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/bloom-bloom_downsample_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(bloom_downsample_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module =
        device.create_shader_module(wgpu::include_spirv!("bloom-bloom_downsample_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(bloom_downsample_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "bloom::bloom_downsample_fragment",
    }
}

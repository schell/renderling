
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [bloom::bloom_vertex](crate::bloom::bloom_vertex).
//!
//! **source path**: `crates/renderling/src/linkage/bloom-bloom_vertex.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!("creating shader module for {}", stringify!(bloom_vertex));
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("bloom-bloom_vertex.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(bloom_vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "bloom::bloom_vertex",
    }
}

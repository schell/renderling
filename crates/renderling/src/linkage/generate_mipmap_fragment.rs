
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::generate_mipmap_fragment](crate::convolution::generate_mipmap_fragment).
//!
//! **source path**: `crates/renderling/src/linkage/convolution-generate_mipmap_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(generate_mipmap_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "convolution-generate_mipmap_fragment.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(generate_mipmap_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "convolution::generate_mipmap_fragment",
    }
}

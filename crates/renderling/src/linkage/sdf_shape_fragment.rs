
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [sdf::sdf_shape_fragment](crate::sdf::sdf_shape_fragment).
//!
//! **source path**: `crates/renderling/src/linkage/sdf-sdf_shape_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(sdf_shape_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!("sdf-sdf_shape_fragment.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf_shape_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::sdf_shape_fragment",
    }
}

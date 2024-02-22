
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [sdf::sdf_prim_fragment_test](crate::sdf::sdf_prim_fragment_test).
//!
//! **source path**: `crates/renderling/src/linkage/sdf-sdf_prim_fragment_test.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(sdf_prim_fragment_test)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module =
        device.create_shader_module(wgpu::include_spirv!("sdf-sdf_prim_fragment_test.spv"));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(sdf_prim_fragment_test)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "sdf::sdf_prim_fragment_test",
    }
}

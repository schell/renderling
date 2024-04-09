
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [skybox::skybox_equirectangular_fragment](crate::skybox::skybox_equirectangular_fragment).
//!
//! **source path**:
//! `crates/renderling/src/linkage/skybox-skybox_equirectangular_fragment.spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(skybox_equirectangular_fragment)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "skybox-skybox_equirectangular_fragment.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(skybox_equirectangular_fragment)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "skybox::skybox_equirectangular_fragment",
    }
}

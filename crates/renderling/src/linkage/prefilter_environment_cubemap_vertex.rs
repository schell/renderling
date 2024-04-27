
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::prefilter_environment_cubemap_vertex](crate::convolution::prefilter_environment_cubemap_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/
//! convolution-prefilter_environment_cubemap_vertex.spv`
use super::ShaderLinkage;
use std::sync::{Arc, Mutex};
static PREFILTER_ENVIRONMENT_CUBEMAP_VERTEX: Mutex<Option<Arc<wgpu::ShaderModule>>> =
    Mutex::new(None);
fn get_module(device: &wgpu::Device) -> Arc<wgpu::ShaderModule> {
    let mut guard = PREFILTER_ENVIRONMENT_CUBEMAP_VERTEX.lock().unwrap();
    if let Some(module) = guard.as_ref() {
        module.clone()
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        log::debug!(
            "creating shader module for {}",
            stringify!(prefilter_environment_cubemap_vertex)
        );
        let module = Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-prefilter_environment_cubemap_vertex.spv"
        )));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let duration = std::time::Instant::now() - start;
            log::debug!(
                "...created shader module {} in {duration:?}",
                stringify!(prefilter_environment_cubemap_vertex)
            );
        }
        *guard = Some(module.clone());
        module
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: get_module(device),
        entry_point: "convolution::prefilter_environment_cubemap_vertex",
    }
}


//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [convolution::brdf_lut_convolution_vertex](crate::convolution::brdf_lut_convolution_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/convolution-brdf_lut_convolution_vertex.spv`
use super::ShaderLinkage;
use std::sync::{Arc, Mutex};
static BRDF_LUT_CONVOLUTION_VERTEX: Mutex<Option<Arc<wgpu::ShaderModule>>> = Mutex::new(None);
fn get_module(device: &wgpu::Device) -> Arc<wgpu::ShaderModule> {
    let mut guard = BRDF_LUT_CONVOLUTION_VERTEX.lock().unwrap();
    if let Some(module) = guard.as_ref() {
        module.clone()
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        log::debug!(
            "creating shader module for {}",
            stringify!(brdf_lut_convolution_vertex)
        );
        let module = Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "convolution-brdf_lut_convolution_vertex.spv"
        )));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let duration = std::time::Instant::now() - start;
            log::debug!(
                "...created shader module {} in {duration:?}",
                stringify!(brdf_lut_convolution_vertex)
            );
        }
        *guard = Some(module.clone());
        module
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: get_module(device),
        entry_point: "convolution::brdf_lut_convolution_vertex",
    }
}

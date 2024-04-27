
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [shader_test::array_test](crate::shader_test::array_test).
//!
//! **source path**: `crates/renderling/src/linkage/shader_test-array_test.spv`
use super::ShaderLinkage;
use std::sync::{Arc, Mutex};
static ARRAY_TEST: Mutex<Option<Arc<wgpu::ShaderModule>>> = Mutex::new(None);
fn get_module(device: &wgpu::Device) -> Arc<wgpu::ShaderModule> {
    let mut guard = ARRAY_TEST.lock().unwrap();
    if let Some(module) = guard.as_ref() {
        module.clone()
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        log::debug!("creating shader module for {}", stringify!(array_test));
        let module = Arc::new(
            device.create_shader_module(wgpu::include_spirv!("shader_test-array_test.spv")),
        );
        #[cfg(not(target_arch = "wasm32"))]
        {
            let duration = std::time::Instant::now() - start;
            log::debug!(
                "...created shader module {} in {duration:?}",
                stringify!(array_test)
            );
        }
        *guard = Some(module.clone());
        module
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: get_module(device),
        entry_point: "shader_test::array_test",
    }
}

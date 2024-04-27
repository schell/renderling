
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_slabbed_renderlet](crate::tutorial::tutorial_slabbed_renderlet).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_slabbed_renderlet.spv`
use super::ShaderLinkage;
use std::sync::{Arc, Mutex};
static TUTORIAL_SLABBED_RENDERLET: Mutex<Option<Arc<wgpu::ShaderModule>>> = Mutex::new(None);
fn get_module(device: &wgpu::Device) -> Arc<wgpu::ShaderModule> {
    let mut guard = TUTORIAL_SLABBED_RENDERLET.lock().unwrap();
    if let Some(module) = guard.as_ref() {
        module.clone()
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let start = std::time::Instant::now();
        log::debug!(
            "creating shader module for {}",
            stringify!(tutorial_slabbed_renderlet)
        );
        let module = Arc::new(device.create_shader_module(wgpu::include_spirv!(
            "tutorial-tutorial_slabbed_renderlet.spv"
        )));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let duration = std::time::Instant::now() - start;
            log::debug!(
                "...created shader module {} in {duration:?}",
                stringify!(tutorial_slabbed_renderlet)
            );
        }
        *guard = Some(module.clone());
        module
    }
}
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    ShaderLinkage {
        module: get_module(device),
        entry_point: "tutorial::tutorial_slabbed_renderlet",
    }
}

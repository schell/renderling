
//! Automatically generated with `cd shaders && cargo run --release`.
//!
//! Provides the shader linkage for
//! [tutorial::tutorial_implicit_isosceles_vertex](crate::tutorial::tutorial_implicit_isosceles_vertex).
//!
//! **source path**:
//! `crates/renderling/src/linkage/tutorial-tutorial_implicit_isosceles_vertex.
//! spv`
use super::ShaderLinkage;
pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
    log::debug!(
        "creating shader module for {}",
        stringify!(tutorial_implicit_isosceles_vertex)
    );
    #[cfg(not(target_arch = "wasm32"))]
    let start = std::time::Instant::now();
    let module = device.create_shader_module(wgpu::include_spirv!(
        "tutorial-tutorial_implicit_isosceles_vertex.spv"
    ));
    #[cfg(not(target_arch = "wasm32"))]
    {
        let duration = std::time::Instant::now() - start;
        log::debug!(
            "...created shader module {} in {duration:?}",
            stringify!(tutorial_implicit_isosceles_vertex)
        );
    }
    ShaderLinkage {
        module,
        entry_point: "tutorial::tutorial_implicit_isosceles_vertex",
    }
}

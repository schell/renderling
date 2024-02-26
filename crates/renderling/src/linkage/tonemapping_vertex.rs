
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [tonemapping::tonemapping_vertex](crate::tonemapping::tonemapping_vertex).
            //!
            //! **source path**: `crates/renderling/src/linkage/tonemapping-tonemapping_vertex.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (tonemapping_vertex)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("tonemapping-tonemapping_vertex.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (tonemapping_vertex)) ; } ShaderLinkage { module , entry_point : "tonemapping::tonemapping_vertex" , } }
            
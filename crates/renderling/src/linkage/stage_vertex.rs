
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [stage::stage_vertex](crate::stage::stage_vertex).
            //!
            //! **source path**: `crates/renderling/src/linkage/stage-stage_vertex.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (stage_vertex)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("stage-stage_vertex.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (stage_vertex)) ; } ShaderLinkage { module , entry_point : "stage::stage_vertex" , } }
            
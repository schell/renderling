
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [pbr::pbr_fragment](crate::pbr::pbr_fragment).
            //!
            //! **source path**: `crates/renderling/src/linkage/pbr-pbr_fragment.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (pbr_fragment)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("pbr-pbr_fragment.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (pbr_fragment)) ; } ShaderLinkage { module , entry_point : "pbr::pbr_fragment" , } }
            
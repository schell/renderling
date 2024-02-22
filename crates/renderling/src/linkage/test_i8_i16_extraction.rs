
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [stage::test_i8_i16_extraction](crate::stage::test_i8_i16_extraction).
            //!
            //! **source path**: `crates/renderling/src/linkage/stage-test_i8_i16_extraction.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (test_i8_i16_extraction)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("stage-test_i8_i16_extraction.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (test_i8_i16_extraction)) ; } ShaderLinkage { module , entry_point : "stage::test_i8_i16_extraction" , } }
            
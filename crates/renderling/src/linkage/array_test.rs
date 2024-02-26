
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [shader_test::array_test](crate::shader_test::array_test).
            //!
            //! **source path**: `crates/renderling/src/linkage/shader_test-array_test.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (array_test)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("shader_test-array_test.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (array_test)) ; } ShaderLinkage { module , entry_point : "shader_test::array_test" , } }
            
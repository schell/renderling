
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [skybox::skybox_cubemap_vertex](crate::skybox::skybox_cubemap_vertex).
            //!
            //! **source path**: `crates/renderling/src/linkage/skybox-skybox_cubemap_vertex.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (skybox_cubemap_vertex)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("skybox-skybox_cubemap_vertex.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (skybox_cubemap_vertex)) ; } ShaderLinkage { module , entry_point : "skybox::skybox_cubemap_vertex" , } }
            
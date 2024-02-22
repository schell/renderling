
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [sdf::raymarch::raymarch_rays_fragment](crate::sdf::raymarch::raymarch_rays_fragment).
            //!
            //! **source path**: `crates/renderling/src/linkage/sdf-raymarch-raymarch_rays_fragment.spv`
            use super::ShaderLinkage;
            pub fn linkage (device : & wgpu :: Device) -> ShaderLinkage { log :: debug ! ("creating shader module for {}" , stringify ! (raymarch_rays_fragment)) ; # [cfg (not (target_arch = "wasm32"))] let start = std :: time :: Instant :: now () ; let module = device . create_shader_module (wgpu :: include_spirv ! ("sdf-raymarch-raymarch_rays_fragment.spv")) ; # [cfg (not (target_arch = "wasm32"))] { let duration = std :: time :: Instant :: now () - start ; log :: debug ! ("...created shader module {} in {duration:?}" , stringify ! (raymarch_rays_fragment)) ; } ShaderLinkage { module , entry_point : "sdf::raymarch::raymarch_rays_fragment" , } }
            
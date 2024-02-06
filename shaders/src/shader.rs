//! A shader source and entry point that can be used to create renderling
//! shader linkage.
use quote::quote;

pub fn gen_all_shaders(shaders: impl IntoIterator<Item = (String, std::path::PathBuf)>) -> String {
    struct Linkage {
        source_path: std::path::PathBuf,
        entry_point: String,
    }

    impl Linkage {
        fn to_string(&self) -> String {
            let source_path = self.source_path.file_name().unwrap().to_str().unwrap();
            let entry_point = &self.entry_point;
            let fn_name = syn::parse_str::<syn::Ident>(&entry_point.replace("::", "__"))
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to parse entry point name `{}` as an identifier: {}",
                        entry_point, e
                    )
                });
            let quote = quote! {
                pub fn #fn_name(device: &wgpu::Device) -> ShaderLinkage {
                    log::debug!("creating shader module for {}", stringify!(#fn_name));
                    let start = std::time::Instant::now();
                    let module = device.create_shader_module(wgpu::include_spirv!(#source_path));
                    let duration = std::time::Instant::now() - start;
                    log::debug!("...created shader module {} in {duration:?}", stringify!(#fn_name));

                    ShaderLinkage {
                        module,
                        entry_point: #entry_point,
                    }
                }
            };
            format!(
                r#"
                   /// Returns the shader linkage for
                   /// [{entry_point}](crate::{entry_point}).
                   ///
                   /// **spv path**: `crates/renderling/src/linkage/{source_path}`
                   {quote}
                "#,
            )
        }
    }

    let linkage = shaders
        .into_iter()
        .map(|(entry_point, source_path)| {
            Linkage {
                source_path,
                entry_point,
            }
            .to_string()
        })
        .collect::<Vec<_>>();
    format!(
        r#"
            //! Shader linkage.
            //!
            //! This module is auto-generated.
            #![allow(non_snake_case)]
            use super::ShaderLinkage;
            {}
        "#,
        linkage.join("\n\n")
    )
}

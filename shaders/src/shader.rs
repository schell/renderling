//! A shader source and entry point that can be used to create renderling
//! shader linkage.
use super::ShaderLang;
use quote::quote;

pub fn gen_all_shaders(
    shaders: impl IntoIterator<Item = (String, std::path::PathBuf)>,
    lang: ShaderLang,
) -> String {
    struct Linkage {
        source_path: std::path::PathBuf,
        entry_point: String,
    }

    impl Linkage {
        fn to_string(&self, lang: ShaderLang) -> String {
            let source_path = match lang {
                ShaderLang::Spv => self.source_path.clone(),
                ShaderLang::Wgsl => self.source_path.with_extension("wgsl"),
            };

            let source_path = source_path.file_name().unwrap().to_str().unwrap();
            let entry_point = self.entry_point.clone();

            let fn_name = syn::parse_str::<syn::Ident>(&entry_point.replace("::", "__"))
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to parse entry point name `{}` as an identifier: {}",
                        entry_point, e
                    )
                });

            let entry_point = match lang {
                ShaderLang::Spv => entry_point,
                ShaderLang::Wgsl => entry_point.replace("::", ""),
            };

            let create_module = match lang {
                ShaderLang::Spv => quote! {
                    device.create_shader_module(wgpu::include_spirv!(#source_path))
                },
                ShaderLang::Wgsl => quote! {
                    device.create_shader_module(wgpu::include_wgsl!(#source_path))
                },
            };
            let quote = quote! {
                pub fn #fn_name(device: &wgpu::Device) -> ShaderLinkage {
                    log::debug!("creating shader module for {}", stringify!(#fn_name));
                    #[cfg(not(target_arch = "wasm32"))]
                    let start = std::time::Instant::now();
                    let module = #create_module;
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let duration = std::time::Instant::now() - start;
                        log::debug!("...created shader module {} in {duration:?}", stringify!(#fn_name));
                    }

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
            .to_string(lang)
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

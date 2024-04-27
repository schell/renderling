//! A shader source and entry point that can be used to create renderling
//! shader linkage.
use super::ShaderLang;
use quote::quote;

pub struct Linkage {
    pub source_path: std::path::PathBuf,
    pub entry_point: String,
}

impl Linkage {
    pub fn fn_name(&self) -> &str {
        self.entry_point.split("::").last().unwrap()
    }

    pub fn to_string(&self, lang: ShaderLang) -> String {
        let source_path = match lang {
            ShaderLang::Spv => self.source_path.clone(),
            ShaderLang::Wgsl => self.source_path.with_extension("wgsl"),
        };

        let source_path = source_path.file_name().unwrap().to_str().unwrap();
        let entry_point = self.entry_point.clone();

        let fn_name = syn::parse_str::<syn::Ident>(self.fn_name()).unwrap_or_else(|e| {
            panic!(
                "Failed to parse entry point name `{}` as an identifier: {}",
                entry_point, e
            )
        });
        let static_name = syn::parse_str::<syn::Ident>(&self.fn_name().to_uppercase()).unwrap();

        let entry_point = match lang {
            ShaderLang::Spv => entry_point,
            ShaderLang::Wgsl => entry_point.replace("::", ""),
        };

        let create_module = match lang {
            ShaderLang::Spv => quote! {
                Arc::new(device.create_shader_module(wgpu::include_spirv!(#source_path)))
            },
            ShaderLang::Wgsl => quote! {
                Arc:new(device.create_shader_module(wgpu::include_wgsl!(#source_path)))
            },
        };
        let quote = quote! {
            use std::sync::{Arc, Mutex};

            use super::ShaderLinkage;

            static #static_name: Mutex<Option<Arc<wgpu::ShaderModule>>> = Mutex::new(None);

            fn get_module(device: &wgpu::Device) -> Arc<wgpu::ShaderModule> {
                let mut guard = #static_name.lock().unwrap();
                if let Some(module) = guard.as_ref() {
                    module.clone()
                } else {
                    #[cfg(not(target_arch = "wasm32"))]
                    let start = std::time::Instant::now();

                    log::debug!(
                        "creating shader module for {}",
                        stringify!(#fn_name)
                    );
                    let module = #create_module;

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let duration = std::time::Instant::now() - start;
                        log::debug!(
                            "...created shader module {} in {duration:?}",
                            stringify!(#fn_name)
                        );
                    }
                    *guard = Some(module.clone());
                    module
                }
            }
            pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
                ShaderLinkage {
                    module: get_module(device),
                    entry_point: #entry_point,
                }
            }
        };
        format!(
            r#"
            //! Automatically generated with `cd shaders && cargo run --release`.
            //!
            //! Provides the shader linkage for
            //! [{entry_point}](crate::{entry_point}).
            //!
            //! **source path**: `crates/renderling/src/linkage/{source_path}`
            {quote}
            "#,
        )
    }
}

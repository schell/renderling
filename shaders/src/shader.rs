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
            use std::sync::Arc;

            use super::ShaderLinkage;

            pub const ENTRY_POINT: &str = #entry_point;

            pub fn linkage(device: &wgpu::Device) -> ShaderLinkage {
                ShaderLinkage {
                    module: #create_module,
                    entry_point: ENTRY_POINT,
                }
            }

            pub fn get_from_cache(
                device: &wgpu::Device,
                cache: &mut std::collections::HashMap<&'static str, Arc<ShaderLinkage>>
            ) -> Arc<ShaderLinkage> {
                cache
                    .entry(ENTRY_POINT)
                    .or_insert_with(|| linkage(device).into())
                    .clone()
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

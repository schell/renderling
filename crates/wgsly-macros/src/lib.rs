use quote::quote;
use syn::DeriveInput;

mod syntax;
use syntax::WgslConst;

#[proc_macro_derive(Constructable)]
pub fn derive_constructable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    todo!()
}

#[proc_macro_attribute]
pub fn wgsl_fn(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let fn_item: syn::ItemFn = syn::parse(input.clone()).unwrap();
    input
}

#[proc_macro_attribute]
pub fn wgsl_const(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match syntax::wgsl_const(attrs, input) {
        Err(e) => {
            let e = e.into_compile_error();
            quote! {#e}.into()
        }
        Ok(t) => t.into(),
    }
}

#[proc_macro_attribute]
pub fn wgsl(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parse_input = input.clone();
    let _ast: syn::Block = syn::parse_macro_input!(parse_input);
    let source = input.clone().to_string();
    match naga::front::wgsl::parse_str(&source) {
        Ok(_module) => quote! {
            wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(#source)),
            }
        }
        .into(),
        Err(err) => {
            let callsite = proc_macro2::Span::call_site();
            syn::Error::new(callsite, format!("{}", err.message()))
                .into_compile_error()
                .into()
        }
    }
}

#[proc_macro]
pub fn src(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syntax::src(input) {
        Err(e) => {
            let e = e.into_compile_error();
            quote! { #e }.into()
        }
        Ok(t) => {
            let s = t.to_string();
            quote! { #s }.into()
        }
    }
}

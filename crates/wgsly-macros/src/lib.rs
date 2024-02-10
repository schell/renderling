use quote::quote;
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index, Type, TypeTuple, WhereClause, WherePredicate,
};

#[proc_macro_derive(Constructable)]
pub fn derive_constructable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    todo!()
}

#[proc_macro]
pub fn wgsl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::Block = syn::parse_macro_input!(input.clone());
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
            let line = callsite.start();
            syn::Error::new(callsite, format!("{} on line {:?}", err.message(), line))
                .into_compile_error()
                .into()
        }
    }
}

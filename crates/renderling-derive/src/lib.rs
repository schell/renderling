//! Provides derive macros for `renderling`.
use quote::quote;
use syn::{
    spanned::Spanned, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Type, WhereClause, WherePredicate, Index,
};

enum FieldName {
    Index(Index),
    Ident(Ident)
}

struct Params {
    sizes: Vec<proc_macro2::TokenStream>,
    field_tys: Vec<Type>,
    field_names: Vec<FieldName>,
}

fn get_params(input: &DeriveInput) -> syn::Result<Params> {
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: ref x, .. }),
            ..
        }) => x,
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed: ref x, .. }),
            ..
        }) => x,
        _ => {
            return Err(syn::Error::new(
                name.span(),
                "deriving FromSlab only supports structs".to_string(),
            ))
        }
    };

    let sizes: Vec<_> = fields
        .iter()
        .map(|field| {
            let ty = &field.ty;
            quote! {
                <#ty as renderling_shader::slab::FromSlab>::slab_size()
            }
        })
        .collect();
    let field_tys: Vec<_> = fields.iter().map(|field| field.ty.clone()).collect();
    let field_names: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            field
                .ident
                .clone()
                .map(FieldName::Ident)
                .unwrap_or_else(|| FieldName::Index(Index{index:i as u32, span: field.span()}))
        })
        .collect();
    Ok(Params {
        sizes,
        field_tys,
        field_names,
    })
}

#[proc_macro_derive(FromSlab)]
pub fn derive_from_slab(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    let name = &input.ident;

    let Params {
        sizes,
        field_tys,
        field_names
    } = match get_params(&input) {
        Ok(c) => c,
        Err(e) => return e.into_compile_error().into(),
    };

    let mut generics = input.generics;
    {
        /// Adds a `CanFetch<'lt>` bound on each of the system data types.
        fn constrain_system_data_types(clause: &mut WhereClause, tys: &[Type]) {
            for ty in tys.iter() {
                let where_predicate: WherePredicate =
                    syn::parse_quote!(#ty : renderling_shader::slab::FromSlab);
                clause.predicates.push(where_predicate);
            }
        }

        let where_clause = generics.make_where_clause();
        constrain_system_data_types(where_clause, &field_tys)
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let set_index_field_names = field_names.into_iter().map(|name| match name {
        FieldName::Index(i) => quote!{
            let index = self.#i.read_slab(index, slab);
        },
        FieldName::Ident(field) => quote! {
            let index = self.#field.read_slab(index, slab);
        },
    })
    .collect::<Vec<_>>();
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics renderling_shader::slab::FromSlab for #name #ty_generics #where_clause
        {
            fn slab_size() -> usize {
                #(#sizes)+*
            }

            fn read_slab(&mut self, index: usize, slab: &[u32]) -> usize {
                if slab.len() < index + Self::slab_size() {
                    return index;
                }

                #(#set_index_field_names)*

                index
            }
        }
    };
    output.into()
}

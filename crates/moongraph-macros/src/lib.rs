//! Provides derive macros for `moongraph::Edges` and `moongraph::NodeResults`.
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, Ident, Type, WhereClause, WherePredicate,
};

fn collect_field_types(fields: &Punctuated<Field, Comma>) -> Vec<Type> {
    fields.iter().map(|x| x.ty.clone()).collect()
}

fn gen_identifiers(fields: &Punctuated<Field, Comma>) -> Vec<Ident> {
    fields.iter().map(|x| x.ident.clone().unwrap()).collect()
}

enum DataType {
    Struct,
    Tuple,
}

fn gen_edges_body(ast: &Data, name: &Ident) -> (proc_macro2::TokenStream, Vec<Type>) {
    let (body, fields) = match *ast {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: ref x, .. }),
            ..
        }) => (DataType::Struct, x),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed: ref x, .. }),
            ..
        }) => (DataType::Tuple, x),
        _ => panic!("Enums are not supported"),
    };

    let tys = collect_field_types(fields);

    let fetch_return = match body {
        DataType::Struct => {
            let identifiers = gen_identifiers(fields);

            quote! {
                #name {
                    #( #identifiers: moongraph::Edges::construct(resources)? ),*
                }
            }
        }
        DataType::Tuple => {
            let count = tys.len();
            let fetch = vec![quote! { moongraph::Edges::construct(resources)? }; count];

            quote! {
                #name ( #( #fetch ),* )
            }
        }
    };

    (fetch_return, tys)
}

#[proc_macro_derive(Edges)]
pub fn derive_edges(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    let name = input.ident;
    let (construct_return, tys) = gen_edges_body(&input.data, &name);
    let mut generics = input.generics;
    {
        /// Adds a `Edges` bound on each of the system data types.
        fn constrain_system_data_types(clause: &mut WhereClause, tys: &[Type]) {
            for ty in tys.iter() {
                let where_predicate: WherePredicate = syn::parse_quote!(#ty : moongraph::Edges);
                clause.predicates.push(where_predicate);
            }
        }

        let where_clause = generics.make_where_clause();
        constrain_system_data_types(where_clause, &tys)
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let output = quote! {
        impl #impl_generics moongraph::Edges for #name #ty_generics #where_clause {
            fn reads() -> Vec<moongraph::TypeKey> {
                let mut r = Vec::new();
                #({
                    r.extend(<#tys as moongraph::Edges>::reads());
                })*
                r
            }

            fn writes() -> Vec<moongraph::TypeKey> {
                let mut r = Vec::new();
                #({
                    r.extend(<#tys as moongraph::Edges>::writes());
                })*
                r
            }

            fn moves() -> Vec<moongraph::TypeKey> {
                let mut r = Vec::new();
                #({
                    r.extend(<#tys as moongraph::Edges>::moves());
                })*
                r
            }

            fn construct(resources: &mut moongraph::TypeMap) -> Result<Self, moongraph::GraphError> {
                Ok(#construct_return)
            }
        }
    };

    output.into()
}

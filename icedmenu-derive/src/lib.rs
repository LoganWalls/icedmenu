extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;
    let variants: Vec<_> = match ast.data {
        Data::Struct(data) => data
            .fields
            .into_iter()
            .filter_map(|f| Some(f.ident?.to_string()))
            .collect(),
        Data::Enum(data) => data
            .variants
            .into_iter()
            .map(|v| v.ident.to_string())
            .collect(),
        _ => panic!("Reflective can only be derived for structs and enums"),
    };

    quote! {
        impl Reflective for #ident {
            fn reflect_attr_names() -> Vec<&'static str> {
                vec![#(#variants),*]
            }
        }
    }
    .into()
}
#[proc_macro_derive(FromGenericStyle)]
pub fn from_generic_style_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;
    let field_idents: Vec<Ident> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        _ => panic!("From<GenericStyle> can only be derived for structs"),
    };

    let assignments = field_idents.into_iter().map(|i| {
        quote! {
            if let Some(#i) = value.#i { result.#i = #i }
        }
    });

    quote! {
        impl From<GenericStyle> for #ident {
            fn from(value: GenericStyle) -> Self {
                let mut result = Self::default();
                #(#assignments)*

                result
            }
        }
    }
    .into()
}

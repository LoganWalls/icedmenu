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
#[proc_macro_derive(UpdateFromOther)]
pub fn update_from_other_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let ident = ast.ident;
    let field_idents: Vec<Ident> = match ast.data {
        Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        _ => panic!("UpdateFromOther can only be derived for structs"),
    };

    let assignments = field_idents.into_iter().map(|i| {
        quote! {
            if other.#i.is_some() {
                self.#i = other.#i;
            }
        }
    });

    quote! {
        impl UpdateFromOther for #ident {
            fn update_from(&mut self, other: &Self){
                #(#assignments)*
            }
        }
    }
    .into()
}

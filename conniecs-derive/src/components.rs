use syn::{self, Body, VariantData};
use quote;

pub fn impl_components(ast: syn::DeriveInput) -> quote::Tokens {
    if ast.generics != Default::default() {
        panic!("There may not be generics attached to the Components struct");
    }

    let name = ast.ident;
    let fields = match ast.body {
        Body::Struct(VariantData::Struct(fields)) => fields,
        Body::Struct(VariantData::Unit) => vec![],
        Body::Struct(VariantData::Tuple(_)) => {
            panic!("Components may not be represented by a tuple struct.")
        }
        Body::Enum(_) => {
            panic!("Components may not be represented by an enum. Structs only.");
        }
    };

    let field_inits = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .map(|ident| {
            quote! { #ident : ::conniecs::traits::ComponentStorage::__new() }
        })
        .collect::<Vec<_>>();

    quote! {
        impl ::conniecs::traits::Components for #name {
            fn new() -> Self {
                #name {
                    #(#field_inits),*
                }
            }
        }
    }
}

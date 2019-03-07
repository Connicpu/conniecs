use quote;
use syn::{Data, Field, Fields, Ident, Meta};

pub fn impl_components(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    if ast.generics != Default::default() {
        panic!("There may not be generics attached to the Components struct");
    }

    let name = ast.ident;
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Some(&fields.named),
            Fields::Unit => None,
            Fields::Unnamed(_) => {
                panic!("Components may not be represented by a tuple struct.");
            }
        },
        Data::Union(_) => {
            panic!("Components may not be represented by a union. Structs only.");
        }
        Data::Enum(_) => {
            panic!("Components may not be represented by an enum. Structs only.");
        }
    };

    let init = if let Some(ref fields) = fields {
        let field_inits = fields
            .iter()
            .map(|field| field_info(field))
            .map(|(ident, kind)| {
                quote! { #ident: ::conniecs::component::ComponentList::#kind() }
            });

        quote! {
            #name {
                #(#field_inits),*
            }
        }
    } else {
        quote! { #name }
    };

    let wipe = if let Some(ref fields) = fields {
        let fields = fields.iter().map(|field| field.ident.clone());
        quote! {
            #(
                self.#fields.__wipe();
            )*
        }
    } else {
        quote! {}
    };

    quote! {
        impl ::conniecs::component::ComponentManager for #name {
            fn build_manager() -> Self {
                #init
            }

            #[doc(hidden)]
            fn __wipe_all(&mut self) {
                #wipe
            }

            #[doc(hidden)]
            fn __please_use_the_derive_attribute() {}
        }
    }
}

fn field_info(field: &Field) -> (&Ident, Ident) {
    let kind_attr = field
        .attrs
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .filter(|m| m.name() == "hot" || m.name() == "cold")
        .nth(0);

    match kind_attr {
        Some(Meta::Word(kind)) => (field.ident.as_ref().unwrap(), kind),
        _ => panic!("All component lists must be marked with either #[hot] or #[cold]"),
    }
}

use syn::{self, Attribute, Data, Fields};
use quote;

use crate::system::read_data;
use crate::quote_path;

pub fn impl_systems(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut cs_data = None;

    for attr in &ast.attrs {
        let meta = attr.parse_meta().unwrap();
        match meta.name().to_string().as_str() {
            "data" => cs_data = Some(read_data(&meta)),
            _ => (),
        }
    }

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Some(&fields.named),
            Fields::Unit => None,
            Fields::Unnamed(_) => {
                panic!("Components may not be represented by a tuple struct.")
            }
        }
        Data::Enum(_) => {
            panic!("Components may not be represented by an enum. Structs only.");
        }
        Data::Union(_) => {
            panic!("Components may not be represented by a union. Structs only.");
        }
    };

    let init = if let Some(ref fields) = fields {
        let field_inits = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .map(|ident| {
                quote! { #ident: ::conniecs::system::System::build_system() }
            });

        quote! {
            #name {
                #(#field_inits),*
            }
        }
    } else {
        quote! { #name }
    };

    let empty = syn::punctuated::Punctuated::new();
    let fields = fields.unwrap_or(&empty);

    let active_systems = fields
        .iter()
        .filter(|field| !is_passive(&field.attrs))
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let fields = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let fields = &fields;

    let (components, services) = match cs_data {
        Some((c, s)) => (c, s),
        None => (quote_path("crate::Components"), quote_path("crate::Services")),
    };

    let activated = quote! {
        fn activated(
            &mut self,
            entity: ::conniecs::EntityData<Self::Components>,
            components: &Self::Components,
            services: &mut Self::Services,
        ) {
            use conniecs::system::System;
            #(
                self.#fields.activated(entity, components, services);
            )*
        }
    };

    let reactivated = quote! {
        fn reactivated(
            &mut self,
            entity: ::conniecs::EntityData<Self::Components>,
            components: &Self::Components,
            services: &mut Self::Services,
        ) {
            use conniecs::system::System;
            #(
                self.#fields.reactivated(entity, components, services);
            )*
        }
    };

    let deactivated = quote! {
        fn deactivated(
            &mut self,
            entity: ::conniecs::EntityData<Self::Components>,
            components: &Self::Components,
            services: &mut Self::Services,
        ) {
            use conniecs::system::System;
            #(
                self.#fields.deactivated(entity, components, services);
            )*
        }
    };

    let update = quote! {
        fn update(&mut self, data: &mut ::conniecs::DataHelper<Self::Components, Self::Services>) {
            use conniecs::system::Process;
            #(
                Process::process(&mut self.#active_systems, data);
            )*
        }
    };

    quote! {
        impl ::conniecs::system::SystemManager for #name {
            type Components = #components;
            type Services = #services;

            fn build_manager() -> Self {
                #init
            }

            #activated
            #reactivated
            #deactivated
            #update

            #[doc(hidden)]
            fn __please_use_the_derive_attribute() {}
        }
    }
}

fn is_passive(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        if meta.name() == "passive" {
            return true;
        }
    }

    false
}

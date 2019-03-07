#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::DeriveInput;

mod aspect;
mod components;
mod services;
mod system;
mod systems;

#[proc_macro_derive(Aspect, attributes(aspect, components))]
pub fn derive_aspect(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let result = aspect::impl_aspect(ast);

    //panic!("{}", result);

    result.into()
}

#[proc_macro_derive(ComponentManager, attributes(hot, cold))]
pub fn derive_components(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let result = components::impl_components(ast);

    // Return the generated impl
    result.into()
}

#[proc_macro_derive(ServiceManager)]
pub fn derive_services(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let result = services::impl_services(ast);

    // Return the generated impl
    result.into()
}

#[proc_macro_derive(
    System,
    attributes(
        data,
        system_type,
        process,
        aspect,
        aspect_a,
        aspect_b,
        interval,
        timed_interval,
        activated,
        reactivated,
        deactivated
    )
)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let result = system::impl_system(ast);

    // Return the generated impl
    result.into()
}

#[proc_macro_derive(SystemManager, attributes(data, passive))]
pub fn derive_systems(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let result = systems::impl_systems(ast);

    // Return the generated impl
    result.into()
}

fn improper_attr_format(attr: &str, module: &str) -> ! {
    panic!(
        "{} was not in the correct format. Please refer to the {} \
         module documentation for more information.",
        attr, module
    )
}

fn read_path_item<F>(attr: &syn::Meta, fail: F) -> String
where
    F: FnOnce(),
{
    match attr {
        syn::Meta::Word(word) => word.to_string(),
        syn::Meta::List(list) => {
            let items = &list.nested;
            if items.len() != 1 {
                fail();
                unreachable!();
            }

            match items[0] {
                syn::NestedMeta::Literal(syn::Lit::Str(ref value)) => value.value(),
                syn::NestedMeta::Meta(syn::Meta::Word(ref word)) => word.to_string(),
                _ => {
                    fail();
                    unreachable!();
                }
            }
        }
        syn::Meta::NameValue(syn::MetaNameValue {
            lit: syn::Lit::Str(ref value),
            ..
        }) => value.value(),
        _ => {
            fail();
            unreachable!();
        }
    }
}

fn quote_path(path: &str) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    for (i, part) in path.split("::").enumerate() {
        use proc_macro2::{Ident, Punct, Spacing, Span};
        if i != 0 {
            tokens.append_all(&[
                Punct::new(':', Spacing::Joint),
                Punct::new(':', Spacing::Joint),
            ]);
        }
        if part.len() > 0 {
            tokens.append(Ident::new(part, Span::call_site()));
        }
    }
    tokens
}

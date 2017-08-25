extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod aspect;
mod components;
mod services;
mod system;
mod systems;

#[proc_macro_derive(Aspect, attributes(aspect, components))]
pub fn derive_aspect(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = aspect::impl_aspect(ast);

    // Return the generated impl
    let result = gen.parse().unwrap();

    //panic!("{}", result);

    result
}

#[proc_macro_derive(ComponentManager, attributes(hot, cold))]
pub fn derive_components(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = components::impl_components(ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(ServiceManager)]
pub fn derive_services(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = services::impl_services(ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(System, attributes(data, system_type, process, aspect))]
pub fn derive_system(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = system::impl_system(ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(SystemManager, attributes(data, passive))]
pub fn derive_systems(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = systems::impl_systems(ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn improper_attr_format(attr: &str, module: &str) -> ! {
    panic!(
        "{} was not in the correct format. Please refer to the {} \
         module documentation for more information.",
        attr,
        module
    )
}

fn read_path_item<F>(attr: &syn::MetaItem, fail: F) -> String
where
    F: FnOnce(),
{
    match attr {
        &syn::MetaItem::Word(ref word) => word.to_string(),
        &syn::MetaItem::List(_, ref items) => {
            if items.len() != 1 {
                fail();
                unreachable!();
            }

            match items[0] {
                syn::NestedMetaItem::Literal(syn::Lit::Str(ref value, _)) => value.clone(),
                syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref word)) => word.to_string(),
                _ => {
                    fail();
                    unreachable!();
                }
            }
        }
        &syn::MetaItem::NameValue(_, syn::Lit::Str(ref value, _)) => value.clone(),
        _ => {
            fail();
            unreachable!();
        }
    }
}

fn quote_path(path: &str) -> quote::Tokens {
    let mut tokens = quote::Tokens::new();
    for (i, part) in path.split("::").enumerate() {
        if i != 0 {
            tokens.append("::");
        }
        tokens.append(part);
    }
    tokens
}

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod components;
mod services;
mod systems;

#[proc_macro_derive(Components)]
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

#[proc_macro_derive(Services)]
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

#[proc_macro_derive(Systems)]
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

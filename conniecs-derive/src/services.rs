use syn;
use quote;

pub fn impl_services(ast: syn::DeriveInput) -> quote::Tokens {
    let name = ast.ident;

    quote!{
        impl ::conniecs::services::ServiceManager for #name {
            #[doc(hidden)]
            fn __please_use_the_derive_attribute() {}
        }
    }
}

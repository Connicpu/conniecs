//! TODO: Add documentation including describing how the derive macros work

pub trait ServiceManager: 'static {
    #[doc(hidden)]
    fn __please_use_the_derive_attribute();
}

impl ServiceManager for () {
    #[doc(hidden)]
    fn __please_use_the_derive_attribute() {}
}

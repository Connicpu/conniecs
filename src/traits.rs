pub trait Components {
    fn new() -> Self;
}

pub trait ComponentStorage {
    type Component;

    #[doc(hidden)]
    #[inline]
    fn __new() -> Self;
    #[doc(hidden)]
    #[inline]
    fn __insert(&mut self, index: usize, component: Self::Component);
    #[doc(hidden)]
    #[inline]
    fn __remove(&mut self, index: usize);
    #[doc(hidden)]
    #[inline]
    fn __contains(&self, index: usize) -> bool;
    #[doc(hidden)]
    #[inline]
    fn __get(&self, index: usize) -> Option<&Self::Component>;
    #[doc(hidden)]
    #[inline]
    fn __get_mut(&mut self, index: usize) -> Option<&mut Self::Component>;
}

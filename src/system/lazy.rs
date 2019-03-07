//! TODO: Add documentation including describing how the derive macros work

use std::ops::{Deref, DerefMut};

use crate::entity::EntityData;
use crate::system::{Process, System};
use crate::world::DataHelper;

pub struct LazySystem<T>
where
    T: System,
{
    pub inner: Option<T>,
}

impl<T> Deref for LazySystem<T>
where
    T: System,
{
    type Target = Option<T>;
    fn deref(&self) -> &Option<T> {
        &self.inner
    }
}

impl<T> DerefMut for LazySystem<T>
where
    T: System,
{
    fn deref_mut(&mut self) -> &mut Option<T> {
        &mut self.inner
    }
}

impl<T> LazySystem<T>
where
    T: System,
{
    #[inline]
    pub fn init(&mut self, sys: T) -> bool {
        if self.inner.is_none() {
            self.inner = Some(sys);
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn init_with<F>(&mut self, f: F) -> bool
    where
        F: FnOnce() -> T,
    {
        if self.inner.is_none() {
            self.inner = Some(f());
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn init_override(&mut self, sys: T) -> bool {
        let was = self.inner.is_some();
        self.inner = Some(sys);
        was
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.inner.is_some()
    }
}

impl<T> System for LazySystem<T>
where
    T: System,
{
    type Components = T::Components;
    type Services = T::Services;

    fn build_system() -> Self {
        LazySystem { inner: None }
    }

    fn activated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner
            .as_mut()
            .map(|inner| inner.activated(entity, components, services));
    }

    fn reactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner
            .as_mut()
            .map(|inner| inner.reactivated(entity, components, services));
    }

    fn deactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner
            .as_mut()
            .map(|inner| inner.deactivated(entity, components, services));
    }
}

impl<T> Process for LazySystem<T>
where
    T: Process,
{
    fn process(&mut self, data: &mut DataHelper<T::Components, T::Services>) {
        self.inner.as_mut().map(|inner| inner.process(data));
    }
}

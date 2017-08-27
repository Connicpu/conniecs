use std::ops::{Deref, DerefMut};

use aspect::Aspect;
use entity::{EntityData, EntityIter};
use system::{Process, System};
use system::watcher::Watcher;
use world::DataHelper;

pub trait InteractProcess: InteractSystemFilter {
    fn process<'a>(
        &mut self,
        entities_a: EntityIter<'a, Self::Components>,
        entities_b: EntityIter<'a, Self::Components>,
        data: &mut DataHelper<Self::Components, Self::Services>,
    );
}

pub trait InteractSystemFilter: System {
    fn create_filter_a() -> Aspect<Self::Components>;
    fn create_filter_b() -> Aspect<Self::Components>;
}

pub struct InteractSystem<T>
where
    T: InteractProcess,
{
    pub inner: T,
    watcher_a: Watcher<T::Components>,
    watcher_b: Watcher<T::Components>,
}

impl<T> Deref for InteractSystem<T>
where
    T: InteractProcess,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for InteractSystem<T>
where
    T: InteractProcess,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> System for InteractSystem<T>
where
    T: InteractProcess,
{
    type Components = T::Components;
    type Services = T::Services;

    fn build_system() -> Self {
        InteractSystem {
            inner: T::build_system(),
            watcher_a: Watcher::new(T::create_filter_a()),
            watcher_b: Watcher::new(T::create_filter_b()),
        }
    }

    fn activated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.watcher_b
            .activated(entity, components, services, &mut self.inner);
        self.watcher_a
            .activated(entity, components, services, &mut self.inner);
    }

    fn reactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.watcher_b
            .reactivated(entity, components, services, &mut self.inner);
        self.watcher_a
            .reactivated(entity, components, services, &mut self.inner);
    }

    fn deactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.watcher_b
            .deactivated(entity, components, services, &mut self.inner);
        self.watcher_a
            .deactivated(entity, components, services, &mut self.inner);
    }
}

impl<T> Process for InteractSystem<T>
where
    T: InteractProcess,
{
    fn process(&mut self, data: &mut DataHelper<T::Components, T::Services>) {
        let iter_a = EntityIter::Map(self.watcher_a.interested.values());
        let iter_b = EntityIter::Map(self.watcher_b.interested.values());
        self.inner.process(iter_a, iter_b, data);
    }
}

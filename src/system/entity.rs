//! Systems to specifically deal with entities.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use aspect::Aspect;
use entity::{Entity, EntityData, EntityIter, IndexedEntity};
use system::{Process, System};
use world::DataHelper;

pub trait EntityProcess: FilteredEntitySystem {
    fn process<'a>(
        &mut self,
        entities: EntityIter<'a, Self::Components>,
        data: &mut DataHelper<Self::Components, Self::Services>,
    );
}

/// This trait is implemented automatically when you `#[derive(System)]` with the following:
///
/// ```
///# #[macro_use] extern crate conniecs_derive; extern crate conniecs;
///# #[derive(ComponentManager)] struct Components { #[hot] pub foo: conniecs::ComponentList<Components, ()>, }
///# #[derive(System)]
///# #[data(Components, services = "()")]
/// #[system_type(entity)]
/// #[aspect(all(foo))]
///# struct MySystem;
///# fn main() {}
/// ```
///
/// or
///
/// ```
///# #[macro_use] extern crate conniecs_derive; extern crate conniecs;
///# #[derive(ComponentManager)] struct Components { #[hot] pub foo: conniecs::ComponentList<Components, ()>, }
///# #[derive(System)]
///# #[data(Components, services = "()")]
/// #[system_type(entity)]
/// #[aspect = "some::aspect::UnitStruct"]
///# struct MySystem;
///# fn main() {}
/// ```
pub trait FilteredEntitySystem: System {
    fn create_aspect() -> Aspect<Self::Components>;
}

pub struct EntitySystem<T: EntityProcess> {
    pub inner: T,
    interested: HashMap<Entity, IndexedEntity<T::Components>>,
    aspect: Aspect<T::Components>,
}

impl<T: EntityProcess> EntitySystem<T> {
    pub fn new() -> EntitySystem<T> {
        EntitySystem {
            interested: HashMap::new(),
            aspect: T::create_aspect(),
            inner: T::build_system(),
        }
    }
}

impl<T: EntityProcess> Deref for EntitySystem<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: EntityProcess> DerefMut for EntitySystem<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: EntityProcess> System for EntitySystem<T> {
    type Components = T::Components;
    type Services = T::Services;

    fn build_system() -> Self {
        EntitySystem::new()
    }

    fn activated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        if self.aspect.check(entity, components) {
            self.interested.insert(**entity, entity.__clone());
            self.inner.activated(entity, components, services);
        }
    }

    fn reactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        match (
            self.interested.contains_key(&entity),
            self.aspect.check(entity, components),
        ) {
            (true, true) => self.inner.reactivated(entity, components, services),
            (true, false) => {
                self.interested.remove(&entity);
                self.inner.deactivated(entity, components, services);
            }
            (false, true) => {
                self.interested.insert(**entity, entity.__clone());
                self.inner.activated(entity, components, services);
            }
            (false, false) => {}
        }
    }

    fn deactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        if self.interested.remove(&entity).is_some() {
            self.inner.deactivated(entity, components, services);
        }
    }
}

impl<T: EntityProcess> Process for EntitySystem<T> {
    fn process(&mut self, data: &mut DataHelper<T::Components, T::Services>) {
        self.inner
            .process(EntityIter::Map(self.interested.values()), data);
    }
}

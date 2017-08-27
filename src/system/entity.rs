//! Systems to specifically deal with entities.

use std::ops::{Deref, DerefMut};

use aspect::Aspect;
use entity::{EntityData, EntityIter};
use system::{Process, System};
use system::watcher::Watcher;
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
/// # #[macro_use] extern crate conniecs_derive; extern crate conniecs;
/// # use conniecs::{EntityIter, DataHelper};
/// # #[derive(ComponentManager)] struct Components { #[hot] pub foo: conniecs::ComponentList<Components, ()>, }
/// #[derive(System, Default)]
/// #[system_type(entity)]
/// #[aspect = "some::aspect::UnitStruct"]
/// #[process(process)]
/// struct MySystem;
/// # fn process(_: &mut MySystem, _: EntityIter<Components>, _: &mut DataHelper<Components, Services>) {}
/// # mod some { pub mod aspect { #[derive(Aspect, Copy, Clone)] #[aspect(all(foo))] pub struct UnitStruct; } }
/// # #[derive(ServiceManager, Default)] struct Services {}
/// # #[derive(SystemManager)] struct Systems { #[passive] sys: conniecs::system::EntitySystem<MySystem> }
/// # fn main() { conniecs::World::<Systems>::new(); }
/// ```
///
/// or
///
/// ```
/// # #[macro_use] extern crate conniecs_derive; extern crate conniecs;
/// # use conniecs::{EntityIter};
/// # #[derive(ComponentManager)] struct Components { #[hot] pub foo: conniecs::ComponentList<Components, String>, }
/// #[derive(System, Default)]
/// #[system_type(entity)]
/// #[aspect(all(foo))]
/// #[process = "process"]
/// struct MySystem;
///
/// type DataHelper = conniecs::DataHelper<Components, Services>;
/// fn process(_: &mut MySystem, entities: EntityIter<Components>, data: &mut DataHelper) {
///     for entity in entities {
///         println!("boop the {}", &data.components.foo[entity]);
///     }
/// }
/// # #[derive(ServiceManager, Default)] struct Services {}
/// # #[derive(SystemManager)] struct Systems { #[passive] sys: conniecs::system::EntitySystem<MySystem> }
/// # fn main() { conniecs::World::<Systems>::new(); }
/// ```
pub trait FilteredEntitySystem: System {
    fn create_aspect() -> Aspect<Self::Components>;
}

pub struct EntitySystem<T>
where
    T: EntityProcess,
{
    pub inner: T,
    watcher: Watcher<T::Components>,
}

impl<T> Deref for EntitySystem<T>
where
    T: EntityProcess,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for EntitySystem<T>
where
    T: EntityProcess,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> EntitySystem<T>
where
    T: EntityProcess,
{
    pub fn new() -> EntitySystem<T> {
        EntitySystem {
            inner: T::build_system(),
            watcher: Watcher::new(T::create_aspect()),
        }
    }
}

impl<T> System for EntitySystem<T>
where
    T: EntityProcess,
{
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
        self.watcher
            .activated(entity, components, services, &mut self.inner);
    }

    fn reactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.watcher
            .reactivated(entity, components, services, &mut self.inner);
    }

    fn deactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.watcher
            .deactivated(entity, components, services, &mut self.inner);
    }
}

impl<T> Process for EntitySystem<T>
where
    T: EntityProcess,
{
    fn process(&mut self, data: &mut DataHelper<T::Components, T::Services>) {
        self.inner
            .process(EntityIter::Map(self.watcher.interested.values()), data);
    }
}

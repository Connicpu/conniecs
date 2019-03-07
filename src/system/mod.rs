//! TODO: Add documentation including describing how the derive macros work

use crate::component::ComponentManager;
use crate::entity::EntityData;
use crate::services::ServiceManager;
use crate::world::DataHelper;

pub use crate::system::entity::{EntityProcess, EntitySystem};
pub use crate::system::interact::{InteractProcess, InteractSystem};
pub use crate::system::interval::IntervalSystem;
pub use crate::system::lazy::LazySystem;

pub mod entity;
pub mod interact;
pub mod interval;
pub mod lazy;
pub mod watcher;

pub trait System {
    type Components: ComponentManager;
    type Services: ServiceManager;

    fn build_system() -> Self;

    #[inline]
    fn activated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    ) {
        let (_, _, _) = (entity, components, services);
    }

    #[inline]
    fn reactivated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    ) {
        self.deactivated(entity, components, services);
        self.activated(entity, components, services);
    }

    #[inline]
    fn deactivated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    ) {
        let (_, _, _) = (entity, components, services);
    }
}

pub trait Process: System {
    fn process(&mut self, data: &mut DataHelper<Self::Components, Self::Services>);
}

pub trait SystemManager {
    type Components: ComponentManager;
    type Services: ServiceManager;

    fn build_manager() -> Self;

    fn activated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    );

    fn reactivated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    );

    fn deactivated(
        &mut self,
        entity: EntityData<Self::Components>,
        components: &Self::Components,
        services: &mut Self::Services,
    );

    fn update(&mut self, data: &mut DataHelper<Self::Components, Self::Services>);

    #[doc(hidden)]
    fn __please_use_the_derive_attribute();
}

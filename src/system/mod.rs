use entity::EntityData;
use component::ComponentManager;
use services::ServiceManager;
use world::DataHelper;

pub mod entity;

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

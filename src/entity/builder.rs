use crate::component::ComponentManager;
use crate::entity::{BuildData, ModifyData};
use crate::services::ServiceManager;

pub trait EntityBuilder<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    fn build<'a>(self, entity: BuildData<'a, C>, components: &mut C, services: &mut M);
}

impl<C, M> EntityBuilder<C, M> for ()
where
    C: ComponentManager,
    M: ServiceManager,
{
    fn build(self, _: BuildData<C>, _: &mut C, _: &mut M) {}
}

impl<C, M, F> EntityBuilder<C, M> for F
where
    C: ComponentManager,
    M: ServiceManager,
    F: FnOnce(BuildData<C>, &mut C, &mut M),
{
    fn build(self, entity: BuildData<C>, components: &mut C, services: &mut M) {
        self(entity, components, services)
    }
}

pub trait EntityModifier<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    fn modify<'a>(self, entity: ModifyData<'a, C>, components: &mut C, services: &mut M);
}

impl<C, M> EntityModifier<C, M> for ()
where
    C: ComponentManager,
    M: ServiceManager,
{
    fn modify(self, _: ModifyData<C>, _: &mut C, _: &mut M) {}
}

impl<C, M, F> EntityModifier<C, M> for F
where
    C: ComponentManager,
    M: ServiceManager,
    F: FnOnce(ModifyData<C>, &mut C, &mut M),
{
    fn modify(self, entity: ModifyData<C>, components: &mut C, services: &mut M) {
        self(entity, components, services)
    }
}

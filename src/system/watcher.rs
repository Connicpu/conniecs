//! TODO: Add documentation including describing how the derive macros work

use fnv::FnvHashMap;

use std::collections::HashMap;

use aspect::Aspect;
use component::ComponentManager;
use entity::{Entity, EntityData, IndexedEntity};
use services::ServiceManager;
use system::System;

pub struct Watcher<C>
where
    C: ComponentManager,
{
    pub aspect: Aspect<C>,
    pub interested: FnvHashMap<Entity, IndexedEntity<C>>,
}

impl<C> Watcher<C>
where
    C: ComponentManager,
{
    pub fn new(aspect: Aspect<C>) -> Self {
        Watcher {
            aspect,
            interested: HashMap::with_hasher(Default::default()),
        }
    }

    pub fn activated<M, T>(
        &mut self,
        entity: EntityData<C>,
        components: &C,
        services: &mut M,
        inner: &mut T,
    ) where
        M: ServiceManager,
        T: System<Components = C, Services = M>,
    {
        if self.aspect.check(entity, components) {
            self.interested.insert(**entity, entity.__clone());
            inner.activated(entity, components, services);
        }
    }

    pub fn reactivated<M, T>(
        &mut self,
        entity: EntityData<C>,
        components: &C,
        services: &mut M,
        inner: &mut T,
    ) where
        M: ServiceManager,
        T: System<Components = C, Services = M>,
    {
        match (
            self.interested.contains_key(&entity),
            self.aspect.check(entity, components),
        ) {
            (true, true) => inner.reactivated(entity, components, services),
            (true, false) => {
                self.interested.remove(&entity);
                inner.deactivated(entity, components, services);
            }
            (false, true) => {
                self.interested.insert(**entity, entity.__clone());
                inner.activated(entity, components, services);
            }
            (false, false) => {}
        }
    }

    pub fn deactivated<M, T>(
        &mut self,
        entity: EntityData<C>,
        components: &C,
        services: &mut M,
        inner: &mut T,
    ) where
        M: ServiceManager,
        T: System<Components = C, Services = M>,
    {
        if self.interested.remove(&entity).is_some() {
            inner.deactivated(entity, components, services);
        }
    }
}

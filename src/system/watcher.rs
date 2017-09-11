//! TODO: Add documentation including describing how the derive macros work

use free_ranges::{FreeRanges, Range};
use fnv::FnvHashMap;

use std::fmt;

use aspect::Aspect;
use component::ComponentManager;
use entity::{EntityData, EntityIter, IndexedEntity, WatchedEntityIter};
use services::ServiceManager;
use system::System;

pub struct Watcher<C>
where
    C: ComponentManager,
{
    pub aspect: Aspect<C>,
    pub interested: FnvHashMap<usize, IndexedEntity<C>>,
    pub ranges: FreeRanges,
}

impl<C> Watcher<C>
where
    C: ComponentManager,
{
    pub fn iter(&self) -> EntityIter<C> {
        let watched = WatchedEntityIter {
            current_range: Range { min: 1, max: 0 },
            indices: self.ranges.free_ranges(),
            entities: &self.interested,
        };

        EntityIter::Watched(watched)
    }
}

impl<C> Watcher<C>
where
    C: ComponentManager,
{
    pub fn new(aspect: Aspect<C>) -> Self {
        Watcher {
            aspect,
            interested: Default::default(),
            ranges: Default::default(),
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
            if self.interested
                .insert(entity.index(), entity.__clone())
                .is_none()
            {
                self.ranges.set_free(entity.index());
            }

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
            self.interested.contains_key(&entity.index()),
            self.aspect.check(entity, components),
        ) {
            (true, true) => inner.reactivated(entity, components, services),
            (true, false) => {
                self.interested.remove(&entity.index());
                self.ranges.set_used(entity.index());
                inner.deactivated(entity, components, services);
            }
            (false, true) => {
                self.interested.insert(entity.index(), entity.__clone());
                self.ranges.set_free(entity.index());
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
        if self.interested.remove(&entity.index()).is_some() {
            self.ranges.set_used(entity.index());

            inner.deactivated(entity, components, services);
        }
    }
}

impl<C> fmt::Debug for Watcher<C>
where
    C: ComponentManager,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Watcher")
            .field("interested", &self.interested)
            .field("ranges", &self.ranges)
            .finish()
    }
}

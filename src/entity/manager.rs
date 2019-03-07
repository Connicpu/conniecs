use index_pool::IndexPool;
use vec_map::VecMap;

use std::collections::hash_map::HashMap;
use std::marker::PhantomData;
use std::mem;

use crate::component::ComponentManager;
use crate::entity::iter::{EntityIter, IndexedEntityIter};
use crate::entity::{BuildData, Entity, EntityBuilder, EntityData, Id, IndexedEntity};
use crate::services::ServiceManager;
use crate::system::SystemManager;

enum Event {
    BuildEntity(Entity),
    RemoveEntity(Entity),
}

pub struct EntityManager<C>
where
    C: ComponentManager,
{
    indices: IndexPool,
    indexed_entities: VecMap<IndexedEntity<C>>,
    entities: HashMap<Entity, IndexedEntity<C>>,
    event_queue: Vec<Event>,
    next_id: Id,
}

impl<C> EntityManager<C>
where
    C: ComponentManager,
{
    pub fn new() -> Self {
        EntityManager {
            indices: IndexPool::new(),
            indexed_entities: VecMap::new(),
            entities: HashMap::new(),
            event_queue: Vec::new(),
            next_id: 0,
        }
    }

    pub fn flush_queue<M, S>(&mut self, components: &mut C, services: &mut M, systems: &mut S)
    where
        M: ServiceManager,
        S: SystemManager<Components = C, Services = M>,
    {
        use self::Event::*;

        let mut queue = mem::replace(&mut self.event_queue, Vec::new());
        for e in queue.drain(..) {
            match e {
                BuildEntity(entity) => {
                    systems.activated(EntityData(self.indexed(entity)), components, services);
                }
                RemoveEntity(entity) => {
                    systems.deactivated(EntityData(self.indexed(entity)), components, services);
                }
            }
        }
        // queue is the one with the nice big chunk of memory still
        // laying around. Don't wanna waste that ;)
        self.event_queue = queue;
    }

    pub fn create_entity<B, M>(
        &mut self,
        builder: B,
        components: &mut C,
        services: &mut M,
    ) -> Entity
    where
        B: EntityBuilder<C, M>,
        M: ServiceManager,
    {
        let entity = self.create();
        builder.build(BuildData(self.indexed(entity)), components, services);
        self.event_queue.push(Event::BuildEntity(entity));
        entity
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        if self.entities.contains_key(&entity) {
            self.event_queue.push(Event::RemoveEntity(entity));
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> EntityIter<C> {
        EntityIter::Indexed(IndexedEntityIter {
            iter: self.indices.all_indices(),
            values: &self.indexed_entities,
        })
    }

    pub fn count(&self) -> usize {
        self.indices.maximum()
    }

    pub fn indexed(&self, entity: Entity) -> &IndexedEntity<C> {
        &self.entities[&entity]
    }

    /// Creates a new `Entity`, assigning it the first available index.
    pub fn create(&mut self) -> Entity {
        self.next_id += 1;
        let entity = Entity { id: self.next_id };
        let ie = IndexedEntity {
            index: self.indices.new_id(),
            entity,
            _marker: PhantomData,
        };
        self.indexed_entities.insert(ie.index, ie.__clone());
        self.entities.insert(entity, ie);
        entity
    }

    /// Returns true if an entity is valid (not removed from the manager).
    #[inline]
    pub fn is_valid(&self, entity: Entity) -> bool {
        self.entities.contains_key(&entity)
    }

    /// Deletes an entity from the manager.
    pub fn remove(&mut self, entity: Entity) {
        self.entities
            .remove(&entity)
            .map(|e| self.indices.return_id(e.index()));
    }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.indices = IndexPool::new();
    }
}

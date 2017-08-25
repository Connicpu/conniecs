use index_pool::IndexPool;

use std::collections::hash_map::{HashMap, Values};
use std::marker::PhantomData;
use std::ops::Deref;
use std::mem;

use aspect::Aspect;
use component::ComponentManager;
use services::ServiceManager;
use system::SystemManager;

pub type Id = u64;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: Id,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedEntity<C>
where
    C: ComponentManager,
{
    index: usize,
    entity: Entity,
    _marker: PhantomData<C>,
}

impl Entity {
    #[inline]
    pub fn nil() -> Entity {
        Default::default()
    }

    #[inline]
    pub fn id(self) -> Id {
        self.id
    }
}

impl<C> IndexedEntity<C>
where
    C: ComponentManager,
{
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    #[doc(hidden)]
    #[inline]
    pub fn __clone(&self) -> Self {
        IndexedEntity {
            index: self.index,
            entity: self.entity,
            _marker: PhantomData,
        }
    }
}

impl<C> Deref for IndexedEntity<C>
where
    C: ComponentManager,
{
    type Target = Entity;
    fn deref(&self) -> &Entity {
        &self.entity
    }
}

pub struct FilteredEntityIter<'a, C>
where
    C: ComponentManager,
{
    inner: EntityIter<'a, C>,
    aspect: Aspect<C>,
    components: &'a C,
}

// Inner Entity Iterator
pub enum EntityIter<'a, C>
where
    C: ComponentManager,
{
    Map(Values<'a, Entity, IndexedEntity<C>>),
}

impl<'a, C> EntityIter<'a, C>
where
    C: ComponentManager,
{
    pub fn filter(self, aspect: Aspect<C>, components: &'a C) -> FilteredEntityIter<'a, C> {
        FilteredEntityIter {
            inner: self,
            aspect: aspect,
            components: components,
        }
    }

    pub fn clone(&self) -> Self {
        let EntityIter::Map(ref values) = *self;
        EntityIter::Map(values.clone())
    }
}

impl<'a, C> Iterator for EntityIter<'a, C>
where
    C: ComponentManager,
{
    type Item = EntityData<'a, C>;
    fn next(&mut self) -> Option<EntityData<'a, C>> {
        match *self {
            EntityIter::Map(ref mut values) => values.next().map(|x| EntityData(x)),
        }
    }
}

impl<'a, C> Iterator for FilteredEntityIter<'a, C>
where
    C: ComponentManager,
{
    type Item = EntityData<'a, C>;
    fn next(&mut self) -> Option<EntityData<'a, C>> {
        for x in self.inner.by_ref() {
            if self.aspect.check(x, self.components) {
                return Some(x);
            } else {
                continue;
            }
        }
        None
    }
}

enum Event {
    BuildEntity(Entity),
    RemoveEntity(Entity),
}

pub struct EntityManager<C>
where
    C: ComponentManager,
{
    indices: IndexPool,
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
        EntityIter::Map(self.entities.values())
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
        self.entities.insert(
            entity,
            IndexedEntity {
                index: self.indices.new_id(),
                entity,
                _marker: PhantomData,
            },
        );
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
}

pub struct BuildData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);
pub struct ModifyData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);
pub struct EntityData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);

impl<'a, C: ComponentManager> Deref for EntityData<'a, C> {
    type Target = IndexedEntity<C>;
    #[inline]
    fn deref(&self) -> &IndexedEntity<C> {
        &self.0
    }
}

impl<'a, C: ComponentManager> Copy for BuildData<'a, C> {}
impl<'a, C: ComponentManager> Copy for ModifyData<'a, C> {}
impl<'a, C: ComponentManager> Copy for EntityData<'a, C> {}

impl<'a, C: ComponentManager> Clone for BuildData<'a, C> {
    #[inline]
    fn clone(&self) -> BuildData<'a, C> {
        *self
    }
}

impl<'a, C: ComponentManager> Clone for ModifyData<'a, C> {
    #[inline]
    fn clone(&self) -> ModifyData<'a, C> {
        *self
    }
}

impl<'a, C: ComponentManager> Clone for EntityData<'a, C> {
    #[inline]
    fn clone(&self) -> EntityData<'a, C> {
        *self
    }
}

pub trait EditData<C: ComponentManager> {
    #[doc(hidden)]
    fn entity(&self) -> &IndexedEntity<C>;
    #[doc(hidden)]
    fn can_insert_components() -> bool;
}

impl<'a, C: ComponentManager> EditData<C> for ModifyData<'a, C> {
    #[doc(hidden)]
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        true
    }
}

impl<'a, C: ComponentManager> EditData<C> for EntityData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        false
    }
}

impl<'a, 'b, C: ComponentManager> EditData<C> for &'b ModifyData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        true
    }
}

impl<'a, 'b, C: ComponentManager> EditData<C> for &'b EntityData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        false
    }
}

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

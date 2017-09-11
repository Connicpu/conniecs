use index_pool::iter::IndexIter;
use free_ranges::Range;
use fnv::FnvHashMap;
use vec_map::VecMap;

use std::collections::hash_map::Values;
use std::collections::btree_set::Iter as BIter;

use aspect::Aspect;
use component::ComponentManager;
use entity::{Entity, EntityData, IndexedEntity};

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
    Indexed(IndexedEntityIter<'a, C>),
    Watched(WatchedEntityIter<'a, C>),
}

impl<'a, C> Clone for EntityIter<'a, C>
where
    C: ComponentManager,
{
    fn clone(&self) -> Self {
        match *self {
            EntityIter::Map(ref map) => EntityIter::Map(map.clone()),
            EntityIter::Indexed(ref ind) => EntityIter::Indexed(ind.clone()),
            EntityIter::Watched(ref wat) => EntityIter::Watched(wat.clone()),
        }
    }
}

pub struct IndexedEntityIter<'a, C>
where
    C: ComponentManager,
{
    pub(crate) iter: IndexIter<'a>,
    pub(crate) values: &'a VecMap<IndexedEntity<C>>,
}

impl<'a, C> Clone for IndexedEntityIter<'a, C>
where
    C: ComponentManager,
{
    fn clone(&self) -> Self {
        IndexedEntityIter {
            iter: self.iter.clone(),
            values: self.values.clone(),
        }
    }
}

pub struct WatchedEntityIter<'a, C>
where
    C: ComponentManager,
{
    pub(crate) current_range: Range,
    pub(crate) indices: BIter<'a, Range>,
    pub(crate) entities: &'a FnvHashMap<usize, IndexedEntity<C>>,
}

impl<'a, C> Clone for WatchedEntityIter<'a, C>
where
    C: ComponentManager,
{
    fn clone(&self) -> Self {
        WatchedEntityIter {
            current_range: self.current_range,
            indices: self.indices.clone(),
            entities: self.entities,
        }
    }
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
}

impl<'a, C> Iterator for EntityIter<'a, C>
where
    C: ComponentManager,
{
    type Item = EntityData<'a, C>;
    fn next(&mut self) -> Option<EntityData<'a, C>> {
        match *self {
            EntityIter::Map(ref mut values) => values.next().map(|x| EntityData(x)),
            EntityIter::Indexed(ref mut iter) => iter.iter
                .next()
                .map(|i| iter.values.get(i).unwrap())
                .map(|x| EntityData(x)),
            EntityIter::Watched(ref mut iter) => {
                if iter.current_range.empty() {
                    if let Some(&range) = iter.indices.next() {
                        iter.current_range = range;
                    } else {
                        return None;
                    }
                    print!("{:?}", iter.current_range);
                } else {
                    print!("{:?}", iter.current_range);
                }

                let index = iter.current_range.min;
                let data = EntityData(&iter.entities[&index]);
                iter.current_range = iter.current_range.pop_front();
                println!(" => {:?}", iter.current_range);
                Some(data)
            }
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

//! TODO: Add documentation including describing how the derive macros work

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

use crate::component::ComponentManager;

pub use crate::entity::builder::*;
pub use crate::entity::data::*;
pub use crate::entity::iter::*;
pub use crate::entity::manager::*;

pub mod builder;
pub mod data;
pub mod iter;
pub mod manager;

pub type Id = u64;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    id: Id,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedEntity<C>
where
    C: ComponentManager,
{
    index: usize,
    entity: Entity,
    _marker: PhantomData<C>,
}

impl<C> fmt::Debug for IndexedEntity<C>
where
    C: ComponentManager,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("IndexedEntity")
            .field("index", &self.index)
            .field("entity", &self.entity)
            .finish()
    }
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

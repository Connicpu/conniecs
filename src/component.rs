//! TODO: Add documentation including describing how the derive macros work

use vec_map::VecMap;
use fnv::FnvHashMap;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use self::InnerComponentList::{Cold, Hot};
use entity::{BuildData, EditData, IndexedEntity, ModifyData};

/// Marks types which are suitable for being components. It is implemented for all
/// types which are `'static`.
pub trait Component: 'static {}
impl<T: 'static> Component for T {}

/// This is the trait implemented for your struct containing all of your
/// component lists. You should not try to implement this manually. Use
/// `#[derive(Components)]` instead. See the module documentation for more
/// information.
pub trait ComponentManager: 'static {
    fn build_manager() -> Self;

    #[doc(hidden)]
    fn __wipe_all(&mut self);

    #[doc(hidden)]
    fn __please_use_the_derive_attribute();
}

#[derive(Debug)]
pub struct ComponentList<C, T>
where
    C: ComponentManager,
    T: Component,
{
    pub(crate) inner: InnerComponentList<T>,
    _marker: PhantomData<C>,
}

#[derive(Debug)]
pub(crate) enum InnerComponentList<T>
where
    T: Component,
{
    Hot(VecMap<T>),
    Cold(FnvHashMap<usize, T>),
}

impl<C, T> ComponentList<C, T>
where
    C: ComponentManager,
    T: Component,
{
    pub fn hot() -> Self {
        ComponentList {
            inner: Hot(VecMap::new()),
            _marker: PhantomData,
        }
    }

    pub fn cold() -> Self {
        ComponentList {
            inner: Cold(HashMap::with_hasher(Default::default())),
            _marker: PhantomData,
        }
    }

    pub fn add(&mut self, entity: BuildData<C>, component: T) -> Option<T> {
        self.inner.insert(entity.0.index(), component)
    }

    pub fn remove(&mut self, entity: ModifyData<C>) -> Option<T> {
        self.inner.remove(entity.0.index())
    }

    pub fn set<E>(&mut self, entity: E, component: T) -> Option<T>
    where
        E: EditData<C>,
    {
        let result = self.inner.insert(entity.entity().index(), component);
        if result.is_none() && !E::can_insert_components() {
            panic!("ComponentList::set was used to insert a new component when modification of activated components was not allowed");
        }
        result
    }

    pub fn get<E>(&self, entity: E) -> Option<T>
    where
        E: EditData<C>,
        T: Clone,
    {
        self.inner.get(entity.entity().index()).cloned()
    }

    pub fn has<E>(&self, entity: E) -> bool
    where
        E: EditData<C>,
    {
        self.inner.contains_key(entity.entity().index())
    }

    pub fn borrow<E>(&mut self, entity: E) -> Option<&mut T>
    where
        E: EditData<C>,
    {
        self.inner.get_mut(entity.entity().index())
    }

    #[doc(hidden)]
    pub fn __clear(&mut self, entity: &IndexedEntity<C>) {
        self.inner.remove(entity.index());
    }

    #[doc(hidden)]
    pub fn __wipe(&mut self) {
        self.inner.clear();
    }
}

impl<C, T, E> Index<E> for ComponentList<C, T>
where
    C: ComponentManager,
    T: Component,
    E: EditData<C>,
{
    type Output = T;
    fn index(&self, entity: E) -> &T {
        self.inner.index(entity.entity().index())
    }
}

impl<C, T, E> IndexMut<E> for ComponentList<C, T>
where
    C: ComponentManager,
    T: Component,
    E: EditData<C>,
{
    fn index_mut(&mut self, entity: E) -> &mut T {
        self.inner.index_mut(entity.entity().index())
    }
}

impl<T> InnerComponentList<T>
where
    T: Component,
{
    pub(crate) fn insert(&mut self, index: usize, component: T) -> Option<T> {
        match *self {
            Hot(ref mut map) => map.insert(index, component),
            Cold(ref mut map) => map.insert(index, component),
        }
    }

    pub(crate) fn remove(&mut self, index: usize) -> Option<T> {
        match *self {
            Hot(ref mut map) => map.remove(index),
            Cold(ref mut map) => map.remove(&index),
        }
    }

    pub(crate) fn contains_key(&self, index: usize) -> bool {
        match *self {
            Hot(ref map) => map.contains_key(index),
            Cold(ref map) => map.contains_key(&index),
        }
    }

    pub(crate) fn index(&self, index: usize) -> &T {
        self.get(index).unwrap_or_else(|| {
            panic!("Entity at index {} does not have this component attached")
        })
    }

    pub(crate) fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap_or_else(|| {
            panic!("Entity at index {} does not have this component attached")
        })
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        match *self {
            Hot(ref map) => map.get(index),
            Cold(ref map) => map.get(&index),
        }
    }

    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match *self {
            Hot(ref mut map) => map.get_mut(index),
            Cold(ref mut map) => map.get_mut(&index),
        }
    }

    pub(crate) fn clear(&mut self) {
        match *self {
            Hot(ref mut map) => map.clear(),
            Cold(ref mut map) => map.clear(),
        }
    }
}

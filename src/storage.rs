use vec_map::VecMap;

use std::collections::HashMap;

use traits::ComponentStorage;

pub struct VecStorage<T> {
    storage: VecMap<T>,
}

impl<T> ComponentStorage for VecStorage<T> {
    type Component = T;

    #[doc(hidden)]
    fn __new() -> Self {
        VecStorage {
            storage: VecMap::new(),
        }
    }

    #[doc(hidden)]
    fn __insert(&mut self, index: usize, component: Self::Component) {
        self.storage.insert(index, component);
    }

    #[doc(hidden)]
    fn __remove(&mut self, index: usize) {
        self.storage.remove(index);
    }

    #[doc(hidden)]
    fn __contains(&self, index: usize) -> bool {
        self.storage.contains_key(index)
    }

    #[doc(hidden)]
    fn __get(&self, index: usize) -> Option<&Self::Component> {
        self.storage.get(index)
    }

    #[doc(hidden)]
    fn __get_mut(&mut self, index: usize) -> Option<&mut Self::Component> {
        self.storage.get_mut(index)
    }
}

pub struct HashStorage<T> {
    storage: HashMap<usize, T>,
}

impl<T> ComponentStorage for HashStorage<T> {
    type Component = T;

    #[doc(hidden)]
    fn __new() -> Self {
        HashStorage {
            storage: HashMap::new(),
        }
    }

    #[doc(hidden)]
    fn __insert(&mut self, index: usize, component: Self::Component) {
        self.storage.insert(index, component);
    }

    #[doc(hidden)]
    fn __remove(&mut self, index: usize) {
        self.storage.remove(&index);
    }

    #[doc(hidden)]
    fn __contains(&self, index: usize) -> bool {
        self.storage.contains_key(&index)
    }

    #[doc(hidden)]
    fn __get(&self, index: usize) -> Option<&Self::Component> {
        self.storage.get(&index)
    }

    #[doc(hidden)]
    fn __get_mut(&mut self, index: usize) -> Option<&mut Self::Component> {
        self.storage.get_mut(&index)
    }
}

pub struct MarkerStorage {
    storage: HashMap<usize, ()>,
}

impl ComponentStorage for MarkerStorage {
    type Component = ();

    #[doc(hidden)]
    fn __new() -> Self {
        MarkerStorage {
            storage: HashMap::new(),
        }
    }

    #[doc(hidden)]
    fn __insert(&mut self, index: usize, component: Self::Component) {
        self.storage.insert(index, component);
    }

    #[doc(hidden)]
    fn __remove(&mut self, index: usize) {
        self.storage.remove(&index);
    }

    #[doc(hidden)]
    fn __contains(&self, index: usize) -> bool {
        self.storage.contains_key(&index)
    }

    #[doc(hidden)]
    fn __get(&self, index: usize) -> Option<&Self::Component> {
        self.storage.get(&index)
    }

    #[doc(hidden)]
    fn __get_mut(&mut self, index: usize) -> Option<&mut Self::Component> {
        self.storage.get_mut(&index)
    }
}

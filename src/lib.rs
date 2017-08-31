//! TODO: Add documentation including describing how the derive macros work

#![cfg_attr(feature = "coroutines", feature(generators, generator_trait, conservative_impl_trait))]

extern crate vec_map;
extern crate index_pool;
extern crate fnv;
extern crate time;

#[cfg(feature = "coroutines")]
extern crate odds;

pub use aspect::Aspect;
pub use component::{ComponentList, ComponentManager};
pub use entity::{BuildData, EditData, EntityData, ModifyData};
pub use entity::{Entity, EntityIter, IndexedEntity};
pub use services::ServiceManager;
pub use system::{EntitySystem, InteractSystem, IntervalSystem, LazySystem, Process, System,
                 SystemManager};
pub use world::{DataHelper, World};

pub mod aspect;
pub mod component;
pub mod entity;
pub mod services;
pub mod system;
pub mod world;

#[cfg(feature = "coroutines")]
pub mod coroutines;

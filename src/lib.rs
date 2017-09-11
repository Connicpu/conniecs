//! TODO: Add documentation including describing how the derive macros work

extern crate vec_map;
extern crate index_pool;
extern crate free_ranges;
extern crate fnv;
extern crate time;

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

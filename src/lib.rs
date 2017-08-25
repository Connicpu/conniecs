extern crate vec_map;
extern crate index_pool;
extern crate fnv;

pub use aspect::Aspect;
pub use component::{Component, ComponentList, ComponentManager};
pub use entity::{BuildData, EditData, EntityData, ModifyData};
pub use entity::{Entity, EntityIter, IndexedEntity};
pub use services::ServiceManager;
pub use system::{Process, System, SystemManager};
pub use world::{DataHelper, World};

pub mod aspect;
pub mod component;
pub mod entity;
pub mod services;
pub mod system;
pub mod world;

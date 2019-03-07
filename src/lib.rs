//! TODO: Add documentation including describing how the derive macros work

pub use crate::aspect::Aspect;
pub use crate::component::{ComponentList, ComponentManager};
pub use crate::entity::{BuildData, EditData, EntityData, ModifyData};
pub use crate::entity::{Entity, EntityIter, IndexedEntity};
pub use crate::services::ServiceManager;
pub use crate::system::{
    EntitySystem, InteractSystem, IntervalSystem, LazySystem, Process, System, SystemManager,
};
pub use crate::world::{DataHelper, World};

pub use conniecs_derive::{Aspect, ComponentManager, ServiceManager, System, SystemManager};

pub mod aspect;
pub mod component;
pub mod entity;
pub mod services;
pub mod system;
pub mod world;

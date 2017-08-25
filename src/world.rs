use component::ComponentManager;
use entity::{Entity, EntityBuilder, EntityData, EntityIter, EntityManager, EntityModifier,
             ModifyData};
use services::ServiceManager;
use system::SystemManager;

pub struct World<S>
where
    S: SystemManager,
{
    pub systems: S,
    pub data: DataHelper<S::Components, S::Services>,
}

pub struct DataHelper<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    pub components: C,
    pub services: M,
    pub(crate) entities: EntityManager<C>,
}

impl<C, M> DataHelper<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    pub fn with_entity_data<F, R>(&mut self, entity: Entity, closure: F) -> Option<R>
    where
        F: FnOnce(EntityData<C>, &mut C, &mut M) -> R,
    {
        if self.entities.is_valid(entity) {
            Some(closure(
                EntityData(self.entities.indexed(entity)),
                &mut self.components,
                &mut self.services,
            ))
        } else {
            None
        }
    }

    pub fn create_entity<B>(&mut self, builder: B) -> Entity
    where
        B: EntityBuilder<C, M>,
    {
        self.entities
            .create_entity(builder, &mut self.components, &mut self.services)
    }

    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        self.entities.remove_entity(entity)
    }

    pub fn entities(&self) -> EntityIter<C> {
        self.entities.iter()
    }
}

impl<S> World<S>
where
    S: SystemManager,
{
    pub fn new() -> Self
    where
        S::Services: Default,
    {
        World::with_services(Default::default())
    }

    pub fn with_services(services: S::Services) -> Self {
        World {
            systems: S::build_manager(),
            data: DataHelper {
                services,
                components: S::Components::build_manager(),
                entities: EntityManager::new(),
            },
        }
    }

    pub fn entities(&self) -> EntityIter<S::Components> {
        self.data.entities.iter()
    }

    pub fn modify_entity<M>(&mut self, entity: Entity, modifier: M)
    where
        M: EntityModifier<S::Components, S::Services>,
    {
        let indexed = self.data.entities.indexed(entity);
        modifier.modify(
            ModifyData(indexed),
            &mut self.data.components,
            &mut self.data.services,
        );
        self.systems.reactivated(
            EntityData(indexed),
            &self.data.components,
            &mut self.data.services,
        );
    }

    pub fn refresh(&mut self) {
        self.flush_queue();

        for entity in self.data.entities.iter() {
            self.systems
                .reactivated(entity, &self.data.components, &mut self.data.services);
        }
    }

    pub fn flush_queue(&mut self) {
        self.data.entities.flush_queue(
            &mut self.data.components,
            &mut self.data.services,
            &mut self.systems,
        );
    }

    pub fn update(&mut self) {
        self.flush_queue();
        self.systems.update(&mut self.data);
        self.flush_queue();
    }
}

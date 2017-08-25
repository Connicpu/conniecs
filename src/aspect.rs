use component::ComponentManager;
use entity::EntityData;

pub struct Aspect<C: ComponentManager>(Box<AspectFilter<C> + 'static>);

impl<C: ComponentManager> Aspect<C> {
    pub fn all() -> Self {
        Aspect(Box::new(All))
    }

    pub fn none() -> Self {
        Aspect(Box::new(None))
    }

    pub fn new<A>(aspect_filter: A) -> Self
    where
        A: AspectFilter<C>,
    {
        Aspect(Box::new(aspect_filter))
    }

    pub fn check<'a>(&self, entity: EntityData<'a, C>, components: &C) -> bool {
        self.0.check(entity, components)
    }
}

pub trait AspectFilter<C: ComponentManager>: 'static {
    fn check<'a>(&self, entity: EntityData<'a, C>, components: &C) -> bool;
}

impl<F, C> AspectFilter<C> for F
where
    C: ComponentManager,
    F: Fn(EntityData<C>, &C) -> bool + 'static,
{
    #[inline]
    fn check<'a>(&self, entity: EntityData<'a, C>, components: &C) -> bool {
        (*self)(entity, components)
    }
}

struct All;
struct None;

impl<C> AspectFilter<C> for All
where
    C: ComponentManager,
{
    #[inline]
    fn check<'a>(&self, _: EntityData<'a, C>, _: &C) -> bool {
        true
    }
}

impl<C> AspectFilter<C> for None
where
    C: ComponentManager,
{
    #[inline]
    fn check<'a>(&self, _: EntityData<'a, C>, _: &C) -> bool {
        false
    }
}

use std::fmt;
use std::ops::Deref;

use crate::component::ComponentManager;
use crate::entity::IndexedEntity;

pub struct BuildData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);
pub struct ModifyData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);
pub struct EntityData<'a, C: ComponentManager>(pub(crate) &'a IndexedEntity<C>);

impl<'a, C: ComponentManager> Deref for EntityData<'a, C> {
    type Target = IndexedEntity<C>;
    #[inline]
    fn deref(&self) -> &IndexedEntity<C> {
        &self.0
    }
}

impl<'a, C: ComponentManager> Copy for BuildData<'a, C> {}
impl<'a, C: ComponentManager> Copy for ModifyData<'a, C> {}
impl<'a, C: ComponentManager> Copy for EntityData<'a, C> {}

impl<'a, C: ComponentManager> Clone for BuildData<'a, C> {
    #[inline]
    fn clone(&self) -> BuildData<'a, C> {
        *self
    }
}

impl<'a, C: ComponentManager> Clone for ModifyData<'a, C> {
    #[inline]
    fn clone(&self) -> ModifyData<'a, C> {
        *self
    }
}

impl<'a, C: ComponentManager> Clone for EntityData<'a, C> {
    #[inline]
    fn clone(&self) -> EntityData<'a, C> {
        *self
    }
}

pub trait EditData<C: ComponentManager> {
    #[doc(hidden)]
    fn entity(&self) -> &IndexedEntity<C>;
    #[doc(hidden)]
    fn can_insert_components() -> bool;
}

impl<'a, C: ComponentManager> EditData<C> for ModifyData<'a, C> {
    #[doc(hidden)]
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        true
    }
}

impl<'a, C: ComponentManager> EditData<C> for EntityData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        false
    }
}

impl<'a, 'b, C: ComponentManager> EditData<C> for &'b ModifyData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        true
    }
}

impl<'a, 'b, C: ComponentManager> EditData<C> for &'b EntityData<'a, C> {
    #[inline]
    fn entity(&self) -> &IndexedEntity<C> {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    fn can_insert_components() -> bool {
        false
    }
}

impl<'a, C> fmt::Debug for EntityData<'a, C>
where
    C: ComponentManager,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("EntityData").field(&self.0).finish()
    }
}

impl<'a, C> fmt::Debug for BuildData<'a, C>
where
    C: ComponentManager,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("BuildData").field(&self.0).finish()
    }
}

impl<'a, C> fmt::Debug for ModifyData<'a, C>
where
    C: ComponentManager,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("ModifyData").field(&self.0).finish()
    }
}

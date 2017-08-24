extern crate conniecs;
#[macro_use]
extern crate conniecs_derive;

use conniecs::traits::ComponentStorage;
use conniecs::storage::{VecStorage, HashStorage, MarkerStorage};

#[derive(Components)]
pub struct Components {
    pub pos: VecStorage<[f32; 2]>,
    pub name: HashStorage<String>,
    pub flag: MarkerStorage,
}

#[test]
fn do_stuff() {
    let mut components: Components = conniecs::traits::Components::new();

    components.flag.__insert(0, ());
    assert_eq!(components.flag.__contains(0), true);
}

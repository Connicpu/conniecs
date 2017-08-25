#[macro_use]
extern crate conniecs_derive;
extern crate conniecs;

use conniecs::ComponentList;
use conniecs::BuildData;

type CList<T> = ComponentList<Components, T>;
type DataHelper = conniecs::DataHelper<Components, Services>;

#[derive(Aspect)]
#[aspect(all(foo, bar))]
pub struct FooBarAspect;

#[derive(Default, ServiceManager)]
pub struct Services;

#[derive(ComponentManager)]
pub struct Components {
    #[cold]
    pub foo: CList<String>,
    #[hot]
    pub bar: CList<f32>,
    #[hot]
    pub baz: CList<[f32; 3]>,
    #[cold]
    pub qux: CList<Vec<i32>>,
}

#[derive(SystemManager)]
pub struct Systems {
    update: Update,

    #[passive]
    panicker: Panicker,
}

#[derive(Default, System)]
#[process(process_update)]
pub struct Update;

fn process_update(_: &mut Update, data: &mut DataHelper) {
    for entity in data.entities() {
        if data.components.foo.has(entity) {
            println!("{}", data.components.foo[entity]);
        }
    }
}

#[derive(Default, System)]
#[process(panicker_update)]
pub struct Panicker;

fn panicker_update(_: &mut Panicker, _: &mut DataHelper) {
    panic!("this shouldn't get called");
}

#[test]
pub fn simulate() {
    let mut world = conniecs::World::<Systems>::new();
    world
        .data
        .create_entity(|e: BuildData<_>, c: &mut Components, _: &mut Services| {
            c.foo.add(e, "asdf".to_string());
        });
    world.update();
}

#[macro_use]
extern crate conniecs_derive;
extern crate conniecs;

use conniecs::ComponentList;
use conniecs::system::EntitySystem;
use conniecs::system::InteractSystem;
use conniecs::system::IntervalSystem;

type Comps<T> = ComponentList<Components, T>;
type EntityIter<'a> = conniecs::EntityIter<'a, Components>;
type DataHelper = conniecs::DataHelper<Components, Services>;
type EntityData<'a> = conniecs::EntityData<'a, Components>;

#[derive(Aspect)]
#[aspect(all(foo, bar))]
pub struct FooBarAspect;

#[derive(Default, ServiceManager)]
pub struct Services;

#[derive(ComponentManager)]
pub struct Components {
    #[cold]
    pub foo: Comps<String>,

    #[hot]
    pub bar: Comps<f32>,
    #[hot]
    pub baz: Comps<[f32; 3]>,

    #[cold]
    pub qux: Comps<Vec<i32>>,
}

#[derive(SystemManager)]
pub struct Systems {
    update: Update,
    esystem: EntitySystem<ESystem>,
    isystem: InteractSystem<ISystem>,
    ivsystem: IntervalSystem<IVSystem>,

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

#[derive(Default, System)]
#[system_type(Entity)]
#[process = "eprocess"]
#[aspect(all(foo), none(qux))]
pub struct ESystem;

fn eprocess(_: &mut ESystem, entities: EntityIter, data: &mut DataHelper) {
    for entity in entities {
        data.components.foo[entity].push_str("ghjkl");
    }
}

#[derive(Default, System)]
#[system_type(Interact)]
#[process = "iprocess"]
#[aspect_a(all(bar), none(baz))]
#[aspect_b(all(baz), none(bar))]
pub struct ISystem;

fn iprocess(_: &mut ISystem, bars: EntityIter, bazes: EntityIter, data: &mut DataHelper) {
    for bar_entity in bars {
        for baz_entity in bazes.clone() {
            data.components.baz[baz_entity][0] += data.components.bar[bar_entity];
        }
    }
}

#[derive(Default, System)]
#[system_type(Interval)]
#[process = "interval_process"]
#[activated = "activated"]
#[interval = "3"]
pub struct IVSystem {
    pub booped: bool,
}

fn interval_process(iv: &mut IVSystem, _: &mut DataHelper) {
    iv.booped = !iv.booped;
}

static ATOMIC_BOOP: std::sync::atomic::AtomicBool = std::sync::atomic::ATOMIC_BOOL_INIT;

fn activated(_: &mut IVSystem, _: EntityData, _: &Components, _: &mut Services) {
    ATOMIC_BOOP.store(true, std::sync::atomic::Ordering::SeqCst);
}

#[test]
pub fn simulate() {
    let mut world = conniecs::World::<Systems>::new();
    assert_eq!(world.systems.ivsystem.booped, false);
    assert_eq!(ATOMIC_BOOP.load(std::sync::atomic::Ordering::SeqCst), false);

    let asdf = world.data.create_entity(|e, c, _| {
        // We need a foo!
        c.foo.add(e, "asdf".to_string());
    });

    let bar = world.data.create_entity(|e, c, _| {
        // We need a bar
        c.bar.add(e, 0.25);
    });
    let _bar1 = world.data.create_entity(|e, c, _| {
        // We need another bar
        c.bar.add(e, 0.25);
    });

    let baz = world.data.create_entity(|e, c, _| {
        // We need a baz
        c.baz.add(e, [0.0, 0.0, 0.0]);
    });

    world.update();
    assert_eq!(world.systems.ivsystem.booped, false);
    assert_eq!(ATOMIC_BOOP.load(std::sync::atomic::Ordering::SeqCst), true);

    world.data.with_entity_data(asdf, |e, c, _| {
        assert_eq!(&c.foo[e], "asdfghjkl");
    });

    world.data.with_entity_data(baz, |e, c, _| {
        assert_eq!(c.baz[e][0], 0.5);
    });

    world.modify_entity(asdf, |e, c, _| {
        c.qux.set(e, vec![1, 2, 3]);
    });

    world.update();
    assert_eq!(world.systems.ivsystem.booped, false);

    world.data.with_entity_data(asdf, |e, c, _| {
        assert_eq!(&c.foo[e], "asdfghjkl");
    });

    world.data.with_entity_data(baz, |e, c, _| {
        assert_eq!(c.baz[e][0], 1.0);
    });

    world.data.remove_entity(bar);

    world.update();
    assert_eq!(world.systems.ivsystem.booped, true);

    world.data.with_entity_data(baz, |e, c, _| {
        assert_eq!(c.baz[e][0], 1.25);
    });

    world.wipe();
}

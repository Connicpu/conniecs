//! TODO: Add documentation including describing how the derive macros work

use time;

use std::ops::{Deref, DerefMut};

use crate::entity::EntityData;
use crate::system::{Process, System};
use crate::world::DataHelper;

pub trait SystemInterval: System {
    fn create_interval() -> TickerState;
}

#[derive(Copy, Clone, Debug)]
pub struct IntervalSystem<T>
where
    T: SystemInterval,
{
    pub inner: T,
    pub ticker: TickerState,
}

impl<T> Deref for IntervalSystem<T>
where
    T: SystemInterval,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> DerefMut for IntervalSystem<T>
where
    T: SystemInterval,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> System for IntervalSystem<T>
where
    T: SystemInterval,
{
    type Components = T::Components;
    type Services = T::Services;

    fn build_system() -> Self {
        IntervalSystem {
            inner: T::build_system(),
            ticker: T::create_interval(),
        }
    }

    fn activated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner.activated(entity, components, services);
    }

    fn reactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner.reactivated(entity, components, services);
    }

    fn deactivated(
        &mut self,
        entity: EntityData<T::Components>,
        components: &T::Components,
        services: &mut T::Services,
    ) {
        self.inner.deactivated(entity, components, services);
    }
}

impl<T> Process for IntervalSystem<T>
where
    T: Process + SystemInterval,
{
    fn process(&mut self, data: &mut DataHelper<T::Components, T::Services>) {
        if self.ticker.tick() {
            self.inner.process(data);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TickerState {
    Frames {
        interval: u64,
        ticks: u64,
    },
    Timed {
        interval: u64,
        next_tick: Option<u64>,
    },
}

impl TickerState {
    pub fn tick(&mut self) -> bool {
        match *self {
            TickerState::Frames {
                interval,
                ref mut ticks,
            } => {
                *ticks += 1;
                if *ticks >= interval {
                    *ticks = 0;
                    true
                } else {
                    false
                }
            }
            TickerState::Timed {
                interval,
                ref mut next_tick,
            } => {
                let now = time::precise_time_ns();
                let next_tick = match next_tick {
                    &mut Some(ref mut tick) => tick,
                    next_tick => {
                        *next_tick = Some(now + interval);
                        return false;
                    }
                };

                if now >= *next_tick {
                    *next_tick += interval;
                    true
                } else {
                    false
                }
            }
        }
    }
}

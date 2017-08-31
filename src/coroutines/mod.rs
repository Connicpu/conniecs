use odds::vec::VecExt;
use time::precise_time_ns;

use std::ops::{Generator, GeneratorState};

use world::DataHelper;
use component::ComponentManager;
use services::ServiceManager;

pub struct CoroutineManager<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    data_ptr: *mut DataHelper<C, M>,
    coroutines: Vec<CoroutineState>,
}

impl<C, M> Default for CoroutineManager<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    fn default() -> Self {
        CoroutineManager {
            data_ptr: ::std::ptr::null_mut(),
            coroutines: vec![],
        }
    }
}

impl<C, M> CoroutineManager<C, M>
where
    C: ComponentManager,
    M: ServiceManager,
{
    pub fn update(&mut self, data: &mut DataHelper<C, M>) {
        if self.data_ptr.is_null() {
            self.data_ptr = data;
        }

        assert_eq!(data as *mut _, self.data_ptr, 
                   "World appears to have been moved since the first update call. This is not allowed");
        let now = precise_time_ns();

        for coro in data.future_coroutines.drain(..) {
            self.coroutines.push(CoroutineState {
                wait_until: now,
                coro,
            });
        }

        self.coroutines.retain_mut(|coro| {
            if coro.wait_until > now {
                return true;
            }

            match coro.coro.resume() {
                GeneratorState::Yielded(action) => match action {
                    CoroAction::Wait(ms) => {
                        coro.wait_until = now + ms * 1_000_000;
                    }
                }
                GeneratorState::Complete(_) => return false,
            }

            true
        });
    }
}

struct CoroutineState {
    wait_until: u64,
    coro: Box<Generator<Yield = CoroAction, Return = ()>>,
}

pub enum CoroAction {
    Wait(u64),
}

pub type BoxCoro = Box<Generator<Yield = CoroAction, Return = ()> + 'static>;

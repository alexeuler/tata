use once_cell::sync::Lazy;
use primitives::{Event, RingVec};
use std::sync::Mutex;
use std::task::Waker;

const BUFFER_SIZE: usize = 32;
pub static REACTOR: Lazy<Mutex<Reactor>> = Lazy::new(|| Mutex::new(Reactor::new()));

pub struct Reactor {
    top: usize,
    events: RingVec<Event>,
    wakers: Vec<Waker>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor {
            wakers: vec![],
            events: RingVec::new(BUFFER_SIZE),
            top: 0,
        }
    }

    pub fn register(&mut self, waker: Waker) {
        self.wakers.push(waker);
    }

    pub fn push_event(&mut self, event: Event) {
        self.events.push(event);
        self.top += 1;
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn get_event(&self, idx: usize) -> Option<&Event> {
        self.events.get(idx)
    }
}

pub fn use_reactor(f: impl FnOnce(&mut Reactor)) {
    if let Ok(mut reactor_mut) = REACTOR.lock() {
        f(&mut *reactor_mut)
    } else {
        println!("Poisoned mutex for reactor");
    }
}

pub fn event_callback(event: Event) {
    let mut wakers = vec![];
    use_reactor(|reactor| {
        reactor.push_event(event);
        for waker in reactor.wakers.drain(..) {
            wakers.push(waker)
        }
    });
    for waker in wakers.drain(..) {
        waker.wake();
    }
}

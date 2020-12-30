//! Holds a static place for accumulating events
//! and waking subscribed futures

use once_cell::sync::Lazy;
use primitives::{PeerEvent, RingVec};
use std::sync::Mutex;
use std::task::Waker;

const BUFFER_SIZE: usize = 32;
static REACTOR: Lazy<Mutex<Reactor>> = Lazy::new(|| Mutex::new(Reactor::new()));

/// Reactor holds all incoming events and wakes subscribed futures if necessary
pub struct Reactor {
    total: usize,
    events: RingVec<PeerEvent>,
    wakers: Vec<Waker>,
}

impl Reactor {
    /// Add a waker to be woken up on new event
    pub fn subscribe(&mut self, waker: Waker) {
        self.wakers.push(waker);
    }

    /// Get the number of all seen events
    pub fn total(&self) -> usize {
        self.total
    }

    /// Get the event with this number.
    pub fn get_event(&self, idx: usize) -> Option<&PeerEvent> {
        self.events.get(idx)
    }

    fn new() -> Self {
        Reactor {
            wakers: vec![],
            events: RingVec::new(BUFFER_SIZE),
            total: 0,
        }
    }
    fn push_event(&mut self, event: PeerEvent) {
        self.events.push(event);
        self.total += 1;
    }
}

pub fn use_reactor(f: impl FnOnce(&mut Reactor)) {
    if let Ok(mut reactor_mut) = REACTOR.lock() {
        f(&mut *reactor_mut)
    } else {
        println!("Poisoned mutex for reactor");
    }
}

pub fn event_callback(event: PeerEvent) {
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

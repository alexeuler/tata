use once_cell::sync::Lazy;
use primitives::Event;
use std::sync::Mutex;
use std::task::Waker;

const BUFFER_SIZE: usize = 32;
pub static REACTOR: Lazy<Mutex<Reactor>> = Lazy::new(|| Mutex::new(Reactor::new()));

pub struct Reactor {
    events: CircularVec<Event>,
    wakers: Vec<Waker>,
}

impl Reactor {
    pub fn new() -> Self {
        Reactor {
            wakers: vec![],
            events: CircularVec::new(BUFFER_SIZE),
        }
    }

    pub fn register(&mut self, waker: Waker) {
        self.wakers.push(waker);
    }
}

pub fn event_callback(event: Event) {
    if let Ok(mut reactor_mut) = REACTOR.lock() {
        reactor_mut.events.push(event);
        for waker in reactor_mut.wakers.drain(..) {
            waker.wake()
        }
    } else {
        println!("Poisoned mutex for reactor");
    }
}

struct CircularVec<T> {
    size: usize,
    pos: usize,
    data: Vec<T>,
}

impl<T> CircularVec<T> {
    pub fn new(size: usize) -> Self {
        CircularVec {
            size,
            pos: 0,
            data: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.pos += 1;
        if self.data.len() != self.size {
            self.data.push(elem);
        } else {
            self.data[self.pos % self.size] = elem;
        }
    }

    pub fn get(&self, pos: usize) -> Option<&T> {
        if pos >= self.pos {
            return None;
        }
        Some(
            self.data
                .get(pos)
                .expect("The element is in the buffer; qed"),
        )
    }
}

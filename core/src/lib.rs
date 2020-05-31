mod event;
pub mod ffi;
mod utils;

use event::Event;
use futures::stream::{empty, Stream};

/// Main struct that drives network behavior
struct Core {
    pub events: Box<dyn Stream<Item = Event> + Send + Unpin>,
}

impl Core {
    /// Create new core
    pub fn new() -> Self {
        Core {
            events: Box::new(empty()),
        }
    }
}

pub fn start(event_callback: fn(Event) -> ()) {}

mod event;

pub use event::Event;
use futures::stream::{empty, Stream};

/// Main struct that drives network behavior
pub struct Core {
    pub events: Box<dyn Stream<Item = Event> + Unpin>,
}

impl Core {
    /// Create new core
    pub fn new() -> Self {
        Core {
            events: Box::new(empty()),
        }
    }
}

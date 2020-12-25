mod core;
mod events;
mod reactor;

use crate::models::Secret;
use events::NetworkEventStream;

pub use self::core::{create_keypair, send};

pub fn start(secret: Secret, name: String) -> NetworkEventStream {
    core::start(secret, name);
    NetworkEventStream::new()
}

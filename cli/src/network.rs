mod core;
mod events;
mod reactor;

use crate::models::Secret;
use events::NetworkEventStream;

pub use self::core::create_keypair;

pub fn start(secret: Secret) -> NetworkEventStream {
    core::start(secret);
    NetworkEventStream::new()
}

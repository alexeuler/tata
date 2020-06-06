use crate::models::*;
use futures::channel::mpsc::{channel, Receiver, Sender};
use lazy_static::lazy_static;
use std::sync::Mutex;

mod tata_core;

use tata_core::ByteArray;
pub use tata_core::CEvent;

static BUFFER_SIZE: usize = 10;

struct Reactor {
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl Reactor {
    pub fn new() -> Self {
        let (tx, rx) = channel(BUFFER_SIZE);
        Reactor { tx, rx }
    }
}

fn event_callback(ev: CEvent) {
    let reactor = REACTOR.lock().expect("Poisoned mutex");
    reactor.tx.try_send(ev.into());
}

lazy_static! {
    static ref REACTOR: Mutex<Reactor> = Mutex::new(Reactor::new());
}

pub fn start_network(secret: Secret, callback: fn(Event)) {
    fn cb(ev: CEvent) {
        callback(ev.into())
    }
    let secret_bytes: Vec<u8> = secret.into();
    let secret_byte_array: ByteArray = secret_bytes.into();
    unsafe {
        if !tata_core::start_network(secret_byte_array, cb) {
            println!("There was an error starting network");
        }
    }
}

pub fn generate_pair() -> (Secret, PeerId) {
    let (secret_bytes, peer_id_bytes) = generate_pair_bytes();
    (Secret::new(secret_bytes), peer_id_bytes.into())
}

fn generate_pair_bytes() -> (Vec<u8>, Vec<u8>) {
    let tata_core::CPair { secret, peer_id } = unsafe { tata_core::generate_pair() };
    let res = (secret.clone().into(), peer_id.clone().into());
    unsafe {
        tata_core::free_array(secret);
        tata_core::free_array(peer_id);
    }
    res
}

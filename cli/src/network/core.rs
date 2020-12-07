//! Bindings for core network lib

use crate::models::*;
use primitives::{
    ffi::{ByteArray, Event, KeyPair},
    LogLevel,
};
use std::convert::TryInto;

extern "C" {
    pub fn start_network(
        secret_array: ByteArray,
        callback: extern "C" fn(Event),
        log_level: LogLevel,
    ) -> bool;
    pub fn generate_keypair() -> KeyPair;
}

pub fn start(secret: Secret) {
    let secret_bytes: Vec<u8> = secret.into();
    let secret_byte_array: ByteArray = secret_bytes.into();
    unsafe {
        if !start_network(secret_byte_array, callback, LogLevel::Info) {
            println!("There was an error starting network");
        }
    }
}

pub fn create_keypair() -> (Secret, PeerId) {
    let (secret_bytes, peer_id_bytes) = generate_keypair_bytes();
    (Secret::new(secret_bytes), peer_id_bytes.into())
}

#[no_mangle]
extern "C" fn callback(ev: Event) {
    match ev.try_into() {
        Ok(event) => super::reactor::event_callback(event),
        Err(e) => println!("Error converting event: {}", e),
    }
}

fn generate_keypair_bytes() -> (Vec<u8>, Vec<u8>) {
    let KeyPair { secret, peer_id } = unsafe { generate_keypair() };
    let res = (secret.into(), peer_id.into());
    res
}

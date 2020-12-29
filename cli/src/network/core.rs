//! Bindings for core network lib

use crate::models::*;
use primitives::{
    ffi::{ByteArray, KeyPair},
    LogLevel,
};
use std::convert::TryInto;
use std::time::SystemTime;

extern "C" {
    pub fn start_network(
        secret_array: ByteArray,
        name: ByteArray,
        callback: extern "C" fn(ByteArray),
        enable_logs: bool,
        log_level: LogLevel,
    ) -> bool;
    pub fn send_message(peer_id: ByteArray, message: ByteArray, timestamp: u64) -> bool;
    pub fn generate_keypair() -> KeyPair;
}

pub fn start(secret: Secret, name: String) {
    let secret_bytes: Vec<u8> = secret.into();
    let secret_byte_array: ByteArray = secret_bytes.into();
    unsafe {
        if !start_network(
            secret_byte_array,
            name.into(),
            callback,
            true,
            LogLevel::Debug,
        ) {
            println!("There was an error starting network");
        }
    }
}

pub fn send(peer: String, message: String) -> bool {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Infallible timestamp; qed")
        .as_millis() as u64;
    println!("Sending message: {}, {}, {}", peer, message, now);
    unsafe { send_message(peer.into(), message.into(), now) }
}

pub fn create_keypair() -> (Secret, PeerId) {
    let (secret_bytes, peer_id_bytes) = generate_keypair_bytes();
    (Secret::new(secret_bytes), peer_id_bytes.into())
}

#[no_mangle]
extern "C" fn callback(ev: ByteArray) {
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

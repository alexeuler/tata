use crate::models::*;
use primitives::{ByteArray, CEvent, CPair, Event};
use std::convert::TryInto;

extern "C" {
    pub fn start_network(secret_array: ByteArray, callback: fn(CEvent)) -> bool;
    pub fn free_array(array: ByteArray);
    pub fn generate_pair() -> CPair;
}

pub fn start(secret: Secret) {
    let secret_bytes: Vec<u8> = secret.into();
    let secret_byte_array: ByteArray = secret_bytes.into();
    unsafe {
        if !start_network(secret_byte_array, callback) {
            println!("There was an error starting network");
        }
    }
}

pub fn generate_keypair() -> (Secret, PeerId) {
    let (secret_bytes, peer_id_bytes) = generate_pair_bytes();
    (Secret::new(secret_bytes), peer_id_bytes.into())
}

fn callback(ev: CEvent) {
    match ev.try_into() {
        Ok(event) => crate::reactor::event_callback(event),
        Err(e) => println!("Error converting event: {}", e),
    }
}

fn generate_pair_bytes() -> (Vec<u8>, Vec<u8>) {
    let CPair { secret, peer_id } = unsafe { generate_pair() };
    let res = (secret.into(), peer_id.into());
    res
}

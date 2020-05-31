use crate::models::*;

mod tata_core;

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

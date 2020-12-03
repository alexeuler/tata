//! Exports for `C` library

use primitives::{
    ffi::{ByteArray, Event, KeyPair},
    LogLevel,
};

/// Start network, see [start_network](../fn.start.html)
#[no_mangle]
pub extern "C" fn start_network(
    secret_array: ByteArray,
    callback: fn(Event),
    log_level: LogLevel,
) -> bool {
    let secret_bytes: Vec<u8> = secret_array.into();
    let secret = match libp2p::identity::secp256k1::SecretKey::from_bytes(secret_bytes) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return false;
        }
    };
    if let Err(e) = crate::start(secret, move |ev| callback(ev.into()), log_level) {
        log::error!("Error: {}", e);
        return false;
    }
    true
}

/// Free allocated ByteArray memory. This needs to be called e.g. after start function for `secret_array`
/// if you're using the library from C.
#[no_mangle]
pub extern "C" fn free_array(array: ByteArray) {
    unsafe {
        array.free();
    }
}

/// Generate secret keypair (to derive PeerId, i.e. p2p identity)
#[no_mangle]
pub extern "C" fn generate_keypair() -> KeyPair {
    let (secret, peer_id) = super::utils::generate_secret();
    let secret_bytes = secret.to_bytes().to_vec();
    let peer_id_bytes = peer_id.into_bytes();
    KeyPair {
        secret: secret_bytes.into(),
        peer_id: peer_id_bytes.into(),
    }
}

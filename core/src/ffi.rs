use primitives::{ByteArray, CEvent, CPair};

#[no_mangle]
pub extern "C" fn start_network(secret_array: ByteArray, callback: fn(CEvent)) -> bool {
    let secret_bytes: Vec<u8> = secret_array.into();
    let secret = match libp2p::identity::secp256k1::SecretKey::from_bytes(secret_bytes) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return false;
        }
    };
    if let Err(e) = crate::start(secret, move |ev| callback(ev.into())) {
        println!("Error: {}", e);
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn free_array(array: ByteArray) {
    unsafe {
        array.free();
    }
}

#[no_mangle]
pub extern "C" fn generate_pair() -> CPair {
    let (secret, peer_id) = super::utils::generate_secret();
    let secret_bytes = secret.to_bytes().to_vec();
    let peer_id_bytes = peer_id.into_bytes();
    CPair {
        secret: secret_bytes.into(),
        peer_id: peer_id_bytes.into(),
    }
}

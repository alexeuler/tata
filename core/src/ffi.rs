//! Exports for `C` library

use async_std::{future::poll_fn, task::Poll};
use futures::stream::StreamExt;
use once_cell::sync::OnceCell;
use std::convert::TryInto;
use std::sync::Mutex;

use primitives::{
    ffi::{ByteArray, Event, KeyPair},
    LogLevel, PlainTextMessage,
};

use crate::network::CoreNetworkBehaviour;
static SWARM: OnceCell<Mutex<libp2p::Swarm<CoreNetworkBehaviour>>> = OnceCell::new();

/// Start network, see [start_network](../fn.start.html)
#[no_mangle]
pub extern "C" fn start_network(
    secret_array: ByteArray,
    name: ByteArray,
    callback: extern "C" fn(Event),
    log_level: LogLevel,
) -> bool {
    let name: Result<String, _> = name.try_into();
    let name = match name {
        Ok(name) => name,
        Err(e) => {
            log::error!("Error parsing peer name: {}", e);
            return false;
        }
    };
    let secret_bytes: Vec<u8> = secret_array.into();
    let secret = match libp2p::identity::secp256k1::SecretKey::from_bytes(secret_bytes) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Error: {}", e);
            return false;
        }
    };
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
    log::debug!("Starting network layer");
    let (swarm, mut events) = match crate::create_swarm(secret, name) {
        Ok(x) => x,
        Err(e) => {
            log::error!("Error creating swarm: {}", e);
            return false;
        }
    };
    if let Err(_) = SWARM.set(Mutex::new(swarm)) {
        log::error!("Error setting static swarm");
        return false;
    };

    async_std::task::spawn(poll_fn(move |cx| {
        match events.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => {
                log::debug!("Sending event: {:?}", event);
                callback(event.into());
                return Poll::Ready(());
            }
            _ => (),
        }
        if let Some(swarm_mutex) = SWARM.get() {
            if let Ok(mut swarm) = swarm_mutex.lock() {
                match swarm.poll_next_unpin(cx) {
                    _ => (),
                }
            }
        }
        Poll::Pending
    }));

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

/// Send a message to peer
#[no_mangle]
pub extern "C" fn send_message(peer_id: ByteArray, message: ByteArray, timestamp: u64) -> bool {
    if let Some(swarm_mutex) = SWARM.get() {
        if let Ok(mut swarm) = swarm_mutex.lock() {
            let from = match peer_id.try_into() {
                Ok(from) => from,
                Err(e) => {
                    log::error!("Error converting `peer_id` bytearray: {}", e);
                    return false;
                }
            };
            let text = match message.try_into() {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Error converting `message` bytearray: {}", e);
                    return false;
                }
            };
            let message = PlainTextMessage {
                from,
                timestamp,
                text,
            };
            swarm.private_chat.send_message(message);
            return true;
        }
    }
    log::error!("Couldn't extract swarm from static cell");
    false
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

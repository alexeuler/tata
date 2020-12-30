//! Provides underlying mechanics for p2p communcations. There are
//! 2 types of exports in this module. One is is the root for using in Rust.
//! The other is in the [ffi](./ffi/index.html) module for using in as a `C` library;
mod error;
pub mod ffi;
mod network;
mod utils;

use error::Result;
use futures::channel::mpsc::Receiver;
use libp2p::identity::secp256k1::{Keypair, SecretKey};
use libp2p::{PeerId, Swarm};
use network::{CoreNetworkBehaviour, HandshakeMetadata};
use primitives::{Metadata, PeerEvent};

const CHANNEL_BUFFER_SIZE: usize = 10;

/// Create a libp2p swarm for polling
///
/// # Arguments
/// `secret` - secret key for the current peer
///
/// `name` - The username for the current user
pub fn create_swarm(
    secret: SecretKey,
    name: String,
) -> Result<(Swarm<CoreNetworkBehaviour>, Receiver<PeerEvent>)> {
    let metadata = Metadata { name };
    let keypair: Keypair = secret.into();
    let libp2p_keypair = libp2p::identity::Keypair::Secp256k1(keypair);
    let public_key = libp2p_keypair.public().clone();
    let peer_id = PeerId::from_public_key(public_key);
    let transport = libp2p::build_development_transport(libp2p_keypair)?;
    let (tx, rx) = futures::channel::mpsc::channel(CHANNEL_BUFFER_SIZE);
    let behaviour = CoreNetworkBehaviour::new(
        HandshakeMetadata {
            name: metadata.name,
        },
        tx,
    )?;

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
    Ok((swarm, rx))
}

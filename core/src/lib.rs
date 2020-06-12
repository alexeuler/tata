mod error;
mod ffi;
mod network;
mod utils;

use error::Result;
use futures::stream::StreamExt;
use libp2p::identity::secp256k1::{Keypair, SecretKey};
use libp2p::{mdns::Mdns, PeerId, Swarm};
use network::CoreNetworkBehaviour;
use primitives::{Event, LogLevel};

const CHANNEL_BUFFER_SIZE: usize = 10;

/// Starts networking
pub fn start(
    secret: SecretKey,
    callback: impl Fn(Event) + Send + Sync + 'static,
    log_level: LogLevel,
) -> Result<()> {
    env_logger::Builder::from_default_env().filter_level(log_level);
    log::info!("Starting network layer...");
    let keypair: Keypair = secret.into();
    let libp2p_keypair = libp2p::identity::Keypair::Secp256k1(keypair);
    let public_key = libp2p_keypair.public().clone();
    let peer_id = PeerId::from_public_key(public_key);
    let mdns = Mdns::new()?;
    let transport = libp2p::build_development_transport(libp2p_keypair)?;
    let (tx, rx) = futures::channel::mpsc::channel(CHANNEL_BUFFER_SIZE);
    let events = rx.for_each(move |ev| {
        callback(ev);
        futures::future::ready(())
    });
    let behaviour = CoreNetworkBehaviour {
        mdns,
        event_sink: tx,
    };

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
    let swarm = swarm.for_each(|ev| {
        // callback(ev);
        futures::future::ready(())
    });
    async_std::task::spawn(swarm);
    async_std::task::spawn(events);
    Ok(())
}

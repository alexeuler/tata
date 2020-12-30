//! Network behaviour implementation

use futures::channel::mpsc::Sender;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use primitives::{Event, PeerEvent};
use std::collections::HashSet;

use crate::error::Result;

use super::private_chat::{HandshakeMetadata, PrivateChatBehaviour};

/// Implementation of networking behaviour for core
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "PeerEvent")]
pub struct CoreNetworkBehaviour {
    pub mdns: Mdns,
    pub private_chat: PrivateChatBehaviour,
    #[behaviour(ignore)]
    pub event_sink: Sender<PeerEvent>,
}

impl CoreNetworkBehaviour {
    pub fn new(local_metadata: HandshakeMetadata, event_sink: Sender<PeerEvent>) -> Result<Self> {
        let mdns = Mdns::new()?;
        let private_chat = PrivateChatBehaviour::new(local_metadata);
        Ok(CoreNetworkBehaviour {
            mdns,
            event_sink,
            private_chat,
        })
    }
}

impl NetworkBehaviourEventProcess<PeerEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: PeerEvent) {
        if let Err(e) = self.event_sink.start_send(event) {
            log::error!("Error sending message to event sink: {}", e);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                let peer_ids = list
                    .into_iter()
                    .map(|(peer_id, _)| peer_id)
                    .collect::<HashSet<_>>();
                for peer_id in peer_ids {
                    let peer_id = peer_id.to_base58().into();
                    let event = PeerEvent {
                        peer_id,
                        event: Event::PeerDiscovered,
                    };
                    if let Err(e) = self.event_sink.try_send(event) {
                        log::error!("Error sending message to event sink: {}", e);
                    }
                }
            }
            MdnsEvent::Expired(list) => {
                let peer_ids = list
                    .into_iter()
                    .map(|(peer_id, _)| peer_id)
                    .collect::<HashSet<_>>();
                for peer_id in peer_ids {
                    let peer_id = peer_id.to_base58().into();
                    let event = PeerEvent {
                        peer_id,
                        event: Event::PeerDiscovered,
                    };
                    if let Err(e) = self.event_sink.try_send(event) {
                        log::error!("Error sending message to event sink: {}", e);
                    }
                }
            }
        }
    }
}

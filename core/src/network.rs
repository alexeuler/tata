//! Network behaviour implementation

use futures::channel::mpsc::Sender;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use primitives::{Event, PeerDiscoveryMessage};

/// Implementation of networking behaviour for core
#[derive(NetworkBehaviour)]
pub struct CoreNetworkBehaviour {
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub event_sink: Sender<Event>,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _) in list {
                    if let Err(e) =
                        self.event_sink
                            .try_send(Event::PeerDiscovered(PeerDiscoveryMessage {
                                peer_id: peer_id.to_base58().into(),
                            }))
                    {
                        log::error!("Error sending message to event sink: {}", e);
                    }
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _) in list {
                    if let Err(e) =
                        self.event_sink
                            .try_send(Event::PeerGone(PeerDiscoveryMessage {
                                peer_id: peer_id.to_base58().into(),
                            }))
                    {
                        log::error!("Error sending message to event sink: {}", e);
                    }
                }
            }
        }
    }
}

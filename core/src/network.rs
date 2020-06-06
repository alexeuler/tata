use super::event::Event;
use crate::event::PeerDiscoverMessage;
use futures::channel::mpsc::Sender;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};

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
                    self.event_sink
                        .try_send(Event::PeerDiscovered(PeerDiscoverMessage { peer_id }));
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _) in list {
                    self.event_sink
                        .try_send(Event::PeerGone(PeerDiscoverMessage { peer_id }));
                }
            }
        }
    }
}

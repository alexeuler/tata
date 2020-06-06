use futures::channel::mpsc::Sender;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use primitives::{Event, PeerDiscoverMessage};

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
                            .try_send(Event::PeerDiscovered(PeerDiscoverMessage {
                                peer_id: peer_id.to_base58().into(),
                            }))
                    {
                        println!("Error sending message to event sink: {}", e);
                    }
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _) in list {
                    if let Err(e) = self
                        .event_sink
                        .try_send(Event::PeerGone(PeerDiscoverMessage {
                            peer_id: peer_id.to_base58().into(),
                        }))
                    {
                        println!("Error sending message to event sink: {}", e);
                    }
                }
            }
        }
    }
}

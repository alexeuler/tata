//! Contains network behavior for establishing connections with newly discovered peers

use libp2p::{
    core::connection::{ConnectedPoint, ConnectionId},
    swarm::protocols_handler::DummyProtocolsHandler,
    swarm::DialPeerCondition,
    swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters, ProtocolsHandler},
    Multiaddr, PeerId,
};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::handler::MetadataHandler;

/// Network behaviour for adding new connections
pub struct MetadataBehaviour {
    pending_peers: Vec<PeerId>,
}

impl MetadataBehaviour {
    /// Create new behaviour
    pub fn new() -> Self {
        Self {
            pending_peers: Vec::new(),
        }
    }
}

impl NetworkBehaviour for MetadataBehaviour {
    type ProtocolsHandler = MetadataHandler;
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        MetadataHandler::default()
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, _: &PeerId) {}

    fn inject_connection_established(&mut self, _: &PeerId, _: &ConnectionId, _: &ConnectedPoint) {}

    fn inject_disconnected(&mut self, _: &PeerId) {}

    fn inject_connection_closed(&mut self, _: &PeerId, _: &ConnectionId, _: &ConnectedPoint) {}

    fn inject_event(
        &mut self,
        _: PeerId,
        _: ConnectionId,
        _: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Some(peer_id) = self.pending_peers.pop() {
            let self_peer_id = params.local_peer_id();
            // Avoid duplex connections
            if self_peer_id < &peer_id {
                log::debug!("Connecting peer {:?}", peer_id);
                return Poll::Ready(NetworkBehaviourAction::DialPeer {
                    peer_id,
                    condition: DialPeerCondition::Disconnected,
                });
            } else {
                log::debug!("Waiting for connection from peer {:?}", peer_id);
                return Poll::Pending;
            }
        }
        Poll::Pending
    }
}

//! Contains network behavior for private chat

use super::{
    handler::{InEvent, PrivateChatHandler},
    protocol::HandshakeMetadata,
};
use crate::error::Result;
use libp2p::{
    core::connection::{ConnectedPoint, ConnectionId},
    swarm::DialPeerCondition,
    swarm::{
        NetworkBehaviour, NetworkBehaviourAction, NotifyHandler, PollParameters, ProtocolsHandler,
    },
    Multiaddr, PeerId,
};
use primitives::{PeerEvent, PlainTextMessage};
use std::collections::{HashSet, VecDeque};
use std::task::{Context, Poll};

/// Network behaviour for private chat
pub struct PrivateChatBehaviour {
    local_metadata: HandshakeMetadata,
    pending_events: VecDeque<PeerEvent>,
    pending_messages: VecDeque<(PeerId, PlainTextMessage)>,
    pending_connections: HashSet<PeerId>,
    connected: HashSet<PeerId>,
}

impl PrivateChatBehaviour {
    /// Create new behaviour
    pub fn new(local_metadata: HandshakeMetadata) -> Self {
        Self {
            pending_events: VecDeque::new(),
            pending_messages: VecDeque::new(),
            pending_connections: HashSet::new(),
            connected: HashSet::new(),
            local_metadata,
        }
    }

    /// Send message to peer
    pub fn send_message(&mut self, message: PlainTextMessage) -> Result<()> {
        let peer_bytes = bs58::decode(message.to.clone()).into_vec()?;
        let peer_id = PeerId::from_bytes(peer_bytes)?;
        self.pending_messages.push_back((peer_id, message));
        Ok(())
    }
}

impl NetworkBehaviour for PrivateChatBehaviour {
    type ProtocolsHandler = PrivateChatHandler;
    type OutEvent = PeerEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        PrivateChatHandler::new(self.local_metadata.clone())
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, _: &PeerId) {}

    fn inject_connection_established(
        &mut self,
        peer_id: &PeerId,
        _: &ConnectionId,
        _: &ConnectedPoint,
    ) {
        self.pending_connections.remove(peer_id);
        self.connected.insert(peer_id.clone());
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId) {
        self.pending_connections.remove(peer_id);
        self.connected.remove(peer_id);
    }

    fn inject_connection_closed(&mut self, _: &PeerId, _: &ConnectionId, _: &ConnectedPoint) {}

    fn inject_event(
        &mut self,
        peer_id: PeerId,
        _: ConnectionId,
        event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
        let peer_id = peer_id.to_base58().to_string();
        self.pending_events.push_back(PeerEvent { peer_id, event });
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        _: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        // Handle all pending messages
        for _ in 0..self.pending_messages.len() {
            if let Some((peer_id, message)) = self.pending_messages.pop_front() {
                if self.connected.contains(&peer_id) {
                    return Poll::Ready(NetworkBehaviourAction::NotifyHandler {
                        peer_id: peer_id.clone(),
                        handler: NotifyHandler::Any,
                        event: InEvent::SendMessage(message.clone()),
                    });
                }
                self.pending_messages.push_back((peer_id.clone(), message));
                if !self.pending_connections.contains(&peer_id) {
                    self.pending_connections.insert(peer_id.clone());
                    return Poll::Ready(NetworkBehaviourAction::DialPeer {
                        peer_id: peer_id.clone(),
                        condition: DialPeerCondition::Disconnected,
                    });
                }
            }
        }
        if let Some(event) = self.pending_events.pop_front() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(event));
        }
        Poll::Pending
    }
}

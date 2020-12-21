//! Contains network behavior for establishing connections with newly discovered peers

use libp2p::{
    core::connection::{ConnectedPoint, ConnectionId},
    swarm::protocols_handler::DummyProtocolsHandler,
    swarm::DialPeerCondition,
    swarm::{
        NetworkBehaviour, NetworkBehaviourAction, NotifyHandler, PollParameters, ProtocolsHandler,
    },
    Multiaddr, PeerId,
};
use primitives::{Event, PlainTextMessage};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use super::{
    handler::{InEvent, PrivateChatHandler},
    protocol::HandshakeMetadata,
};

/// Network behaviour for adding new connections
pub struct PrivateChatBehaviour {
    local_metadata: HandshakeMetadata,
    pending_events: VecDeque<Event>,
    pending_messages: VecDeque<PlainTextMessage>,
    pending_connections: HashSet<PeerId>,
    connected: HashMap<PeerId, HashSet<Multiaddr>>,
}

impl PrivateChatBehaviour {
    /// Create new behaviour
    pub fn new(local_metadata: HandshakeMetadata) -> Self {
        Self {
            pending_events: VecDeque::new(),
            pending_messages: VecDeque::new(),
            pending_connections: HashSet::new(),
            connected: HashMap::new(),
            local_metadata,
        }
    }

    /// Send message to peer
    pub fn send_message(&mut self, message: PlainTextMessage) {
        self.pending_messages.push_back(message);
    }
}

impl NetworkBehaviour for PrivateChatBehaviour {
    type ProtocolsHandler = PrivateChatHandler;
    type OutEvent = Event;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        PrivateChatHandler::new(self.local_metadata.clone())
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        self.connected
            .get(peer_id)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or(Vec::new())
    }

    fn inject_connected(&mut self, _: &PeerId) {}

    fn inject_connection_established(
        &mut self,
        peer_id: &PeerId,
        _: &ConnectionId,
        connected_point: &ConnectedPoint,
    ) {
        self.pending_connections.remove(peer_id);
        let addresses = self
            .connected
            .entry(peer_id.clone())
            .or_insert(HashSet::new());
        addresses.insert(connected_point.get_remote_address().clone());
    }

    fn inject_disconnected(&mut self, _: &PeerId) {}

    fn inject_connection_closed(
        &mut self,
        peer_id: &PeerId,
        _: &ConnectionId,
        connected_point: &ConnectedPoint,
    ) {
        self.pending_connections.remove(peer_id);
        self.connected.entry(peer_id.clone()).and_modify(|set| {
            set.remove(connected_point.get_remote_address());
        });
    }

    fn inject_event(
        &mut self,
        _: PeerId,
        _: ConnectionId,
        event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
        self.pending_events.push_back(event);
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
        if let Some(message) = self.pending_messages.pop_front() {
            let peer_id =
                PeerId::from_bytes(message.from.as_bytes().to_vec()).expect("Infallible; qed");
            if self.connected.contains_key(&peer_id) {
                return Poll::Ready(NetworkBehaviourAction::NotifyHandler {
                    peer_id: peer_id.clone(),
                    handler: NotifyHandler::Any,
                    event: InEvent::SendMessage(message),
                });
            }
            if !self.pending_connections.contains(&peer_id) {
                self.pending_connections.insert(peer_id.clone());
                self.pending_messages.push_back(message);
                return Poll::Ready(NetworkBehaviourAction::DialPeer {
                    peer_id: peer_id.clone(),
                    condition: DialPeerCondition::Disconnected,
                });
            }
        }
        if let Some(event) = self.pending_events.pop_front() {
            return Poll::Ready(NetworkBehaviourAction::GenerateEvent(event));
        }
        Poll::Pending
    }
}

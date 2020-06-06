use libp2p::PeerId;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Event {
    PlainTextMessage(PlainTextMessage),
    PeerDiscovered(PeerDiscoverMessage),
    PeerGone(PeerDiscoverMessage),
}

#[derive(Debug, Clone, Copy)]
pub enum PlainEvent {
    PlainTextMessage,
    PeerDiscovered,
    PeerGone,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlainTextMessage {
    #[serde(serialize_with = "serialize_peer_id")]
    pub from: PeerId,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeerDiscoverMessage {
    #[serde(serialize_with = "serialize_peer_id")]
    pub peer_id: PeerId,
}

fn serialize_peer_id<S>(x: &PeerId, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_base58())
}

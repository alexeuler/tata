use libp2p::PeerId;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Event {
    PlainTextMessage(PlainTextMessage),
}

#[derive(Debug, Clone, Serialize)]
pub struct PlainTextMessage {
    #[serde(serialize_with = "serialize_peer_id")]
    from: PeerId,
    text: String,
}

#[derive(Debug, Clone, Copy)]
pub enum PlainEvent {
    PlainTextMessage,
}

fn serialize_peer_id<S>(x: &PeerId, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&x.to_base58())
}

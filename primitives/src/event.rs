use crate::{ByteArray, CEvent, OpaquePeerId, PlainEvent};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainTextMessage {
    pub from: OpaquePeerId,
    pub text: String,
}

impl Into<ByteArray> for PlainTextMessage {
    fn into(self) -> ByteArray {
        serde_json::to_vec(&self)
            .expect("infallible conversion; qed")
            .into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerDiscoverMessage {
    pub peer_id: OpaquePeerId,
}

impl Into<ByteArray> for PeerDiscoverMessage {
    fn into(self) -> ByteArray {
        serde_json::to_vec(&self)
            .expect("infallible conversion; qed")
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    PlainTextMessage(PlainTextMessage),
    PeerDiscovered(PeerDiscoverMessage),
    PeerGone(PeerDiscoverMessage),
}

impl TryFrom<CEvent> for Event {
    type Error = serde_json::error::Error;
    fn try_from(ev: CEvent) -> Result<Event, serde_json::error::Error> {
        let CEvent { tag, data } = ev;
        let data: Vec<u8> = data.into();
        let res = match tag {
            PlainEvent::PlainTextMessage => {
                let payload = serde_json::from_slice(&data)?;
                Event::PlainTextMessage(payload)
            }
            PlainEvent::PeerDiscovered => {
                let payload = serde_json::from_slice(&data)?;
                Event::PeerDiscovered(payload)
            }
            PlainEvent::PeerGone => {
                let payload = serde_json::from_slice(&data)?;
                Event::PeerGone(payload)
            }
        };
        Ok(res)
    }
}

impl Into<CEvent> for Event {
    fn into(self) -> CEvent {
        match self {
            Event::PlainTextMessage(data) => CEvent {
                tag: PlainEvent::PlainTextMessage,
                data: data.into(),
            },
            Event::PeerDiscovered(data) => CEvent {
                tag: PlainEvent::PeerDiscovered,
                data: data.into(),
            },
            Event::PeerGone(data) => CEvent {
                tag: PlainEvent::PeerGone,
                data: data.into(),
            },
        }
    }
}

use crate::ffi::CEvent;
use crate::models::PeerId;
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize)]
pub struct PlainTextMessage {
    pub from: PeerId,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PeerDiscoverMessage {
    pub peer_id: PeerId,
}

impl From<CEvent> for Event {
    fn from(ev: CEvent) -> Event {
        let CEvent { tag, data } = ev;
        let data: Vec<u8> = data.into();
        match tag {
            PlainEvent::PlainTextMessage => {
                let payload = serde_json::from_slice(&data).expect("Json expected");
                Event::PlainTextMessage(payload)
            }
            PlainEvent::PeerDiscovered => {
                let payload = serde_json::from_slice(&data).expect("Json expected");
                Event::PeerDiscovered(payload)
            }
            PlainEvent::PeerGone => {
                let payload = serde_json::from_slice(&data).expect("Json expected");
                Event::PeerGone(payload)
            }
        }
    }
}

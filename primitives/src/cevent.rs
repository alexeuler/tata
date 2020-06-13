use crate::ByteArray;

/// Ffi representation of event
#[repr(C)]
pub struct CEvent {
    /// Union tag
    pub tag: PlainEvent,
    /// Binary serialized json data
    pub data: ByteArray,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PlainEvent {
    PlainTextMessage,
    PeerDiscovered,
    PeerGone,
}

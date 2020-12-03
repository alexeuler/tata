use super::ByteArray;

/// FFI representation of event
#[repr(C)]
pub struct Event {
    /// Union tag
    pub tag: EventTag,
    /// Binary serialized json data
    pub data: ByteArray,
}

/// Union tag for different types of event. Variants are
/// identical to [Event](../enum.Event.html).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EventTag {
    PlainTextMessage,
    PeerDiscovered,
    PeerGone,
}

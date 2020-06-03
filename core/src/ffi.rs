use crate::event::{Event, PlainEvent};
use serde::Serialize;
use std::mem::ManuallyDrop;

/// Ffi representation of event
#[repr(C)]
pub struct CEvent {
    /// Union tag
    tag: PlainEvent,
    /// Binary serialized json data
    data: ByteArray,
}

impl From<Event> for CEvent {
    fn from(ev: Event) -> CEvent {
        match ev {
            Event::PlainTextMessage(data) => CEvent {
                tag: PlainEvent::PlainTextMessage,
                data: serde_json::to_vec(&data)
                    .expect("infallible conversion; qed")
                    .into(),
            },
        }
    }
}

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for ByteArray {
    fn from(v: Vec<u8>) -> Self {
        let mut v = ManuallyDrop::new(v);
        ByteArray {
            data: v.as_mut_ptr(),
            len: v.len(),
        }
    }
}

#[repr(C)]
pub struct CPair {
    pub secret: ByteArray,
    pub peer_id: ByteArray,
}

#[no_mangle]
pub extern "C" fn free_array(array: ByteArray) {
    let s = unsafe { std::slice::from_raw_parts_mut(array.data, array.len) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn generate_pair() -> CPair {
    let (secret, peer_id) = super::utils::generate_secret();
    let secret_bytes = secret.to_bytes().to_vec();
    let peer_id_bytes = peer_id.into_bytes();
    CPair {
        secret: secret_bytes.into(),
        peer_id: peer_id_bytes.into(),
    }
}

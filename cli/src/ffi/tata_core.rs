use std::mem::ManuallyDrop;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl Into<Vec<u8>> for ByteArray {
    fn into(self) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(self.data, self.len).to_vec() }
    }
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

#[repr(C)]
pub struct CEvent {
    /// Union tag
    pub tag: crate::models::PlainEvent,
    /// Binary serialized json data
    pub data: ByteArray,
}

extern "C" {
    pub fn start_network(secret_array: ByteArray, callback: fn(CEvent)) -> bool;
    pub fn free_array(array: ByteArray);
    pub fn generate_pair() -> CPair;
}

use crate::ByteArray;

#[repr(C)]
pub struct CPair {
    pub secret: ByteArray,
    pub peer_id: ByteArray,
}

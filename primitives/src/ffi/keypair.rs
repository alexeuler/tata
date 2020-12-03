use super::ByteArray;

/// FFI representation of KeyPair
#[repr(C)]
pub struct KeyPair {
    pub secret: ByteArray,
    pub peer_id: ByteArray,
}

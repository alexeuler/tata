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

#[repr(C)]
pub struct CPair {
    pub secret: ByteArray,
    pub peer_id: ByteArray,
}

extern "C" {
    pub fn free_array(array: ByteArray);
    pub fn generate_pair() -> CPair;
}

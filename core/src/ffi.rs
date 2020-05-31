use std::mem::ManuallyDrop;

#[repr(C)]
struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for ByteArray {
    fn from(v: Vec<u8>) -> Self {
        let v = ManuallyDrop::new(v);
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
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
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

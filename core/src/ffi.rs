use std::mem::ManuallyDrop;

#[repr(C)]
pub enum CEvent {
    Message(*const u8, usize),
}

#[repr(C)]
pub struct CPair {
    pub secret_data: *const u8,
    pub secret_len: usize,
    pub peer_id_data: *const u8,
    pub peer_id_len: usize,
}

#[no_mangle]
pub extern "C" fn start(callback: fn(CEvent) -> ()) {}

#[no_mangle]
pub extern "C" fn generate_pair() -> CPair {
    let (secret, peer_id) = super::utils::generate_secret();
    let secret_bytes = secret.to_bytes();
    let secret_bytes = ManuallyDrop::new(secret_bytes);
    let peer_id_bytes = peer_id.into_bytes();
    let peer_id_bytes = ManuallyDrop::new(peer_id_bytes);
    CPair {
        secret_data: secret_bytes.as_ptr(),
        secret_len: secret_bytes.len(),
        peer_id_data: peer_id_bytes.as_ptr(),
        peer_id_len: peer_id_bytes.len(),
    }
}

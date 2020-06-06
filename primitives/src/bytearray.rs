use std::mem::ManuallyDrop;

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

impl Into<Vec<u8>> for ByteArray {
    fn into(self) -> Vec<u8> {
        unsafe {
            let res = std::slice::from_raw_parts(self.data, self.len).to_vec();
            self.free();
            res
        }
    }
}

impl ByteArray {
    pub unsafe fn free(self) {
        let s = std::slice::from_raw_parts_mut(self.data, self.len);
        let s = s.as_mut_ptr();
        Box::from_raw(s);
    }
}

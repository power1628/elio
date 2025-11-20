use bytes::{Bytes, BytesMut};

#[derive(Clone, Debug)]
pub struct Mask {
    bits: Bytes,
    len: usize,
    // Whether all bits are set to 1 or 0.
    all_set: Option<bool>,
}

impl Mask {
    pub fn new_set(len: usize) -> Self {
        Self {
            bits: Bytes::from(vec![0xFF; len.div_ceil(8)]),
            len,
            all_set: Some(true),
        }
    }

    pub fn new_unset(len: usize) -> Self {
        Self {
            bits: Bytes::from(vec![0x00; len.div_ceil(8)]),
            len,
            all_set: Some(false),
        }
    }

    pub fn empty() -> Self {
        Self {
            bits: Bytes::new(),
            len: 0,
            all_set: None,
        }
    }

    pub fn full(value: bool, len: usize) -> Self {
        Self {
            bits: Bytes::from(vec![if value { 0xFF } else { 0x00 }; len.div_ceil(8)]),
            len,
            all_set: Some(value),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, idx: usize) -> bool {
        match self.all_set {
            Some(v) => v,
            None => self.bits[idx / 8] & (0x01 << (idx % 8)) != 0,
        }
    }

    pub fn set_count(&self) -> usize {
        self.len - self.unset_count()
    }

    pub fn unset_count(&self) -> usize {
        match self.all_set {
            Some(true) => 0,
            Some(false) => self.len,
            None => {
                let full_bytes = self.len / 8;
                let remainder_bits = self.len % 8;
                let mut count = 0;

                for i in 0..full_bytes {
                    count += self.bits[i].count_zeros() as usize;
                }

                if remainder_bits > 0 {
                    let last_byte = self.bits[full_bytes + 1];
                    let mask = (1 << remainder_bits) - 1;
                    let relevant_bits = last_byte & mask;
                    count += relevant_bits.count_zeros() as usize;
                }

                count
            }
        }
    }

    pub fn into_mut(self) -> MaskMut {
        MaskMut {
            bits: self.bits.into(),
            len: self.len,
            all_set: self.all_set,
        }
    }

    pub fn all_set(&self) -> bool {
        self.all_set.unwrap_or(false)
    }

    pub fn all_unset(&self) -> bool {
        !self.all_set()
    }
}

pub struct MaskMut {
    bits: BytesMut,
    len: usize,
    all_set: Option<bool>,
}

impl MaskMut {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bits: BytesMut::with_capacity(capacity.div_ceil(8)),
            len: 0,
            all_set: None,
        }
    }

    pub fn new_set(len: usize) -> Self {
        let bytes = len.div_ceil(8);
        let mut bits = BytesMut::with_capacity(bytes);
        unsafe {
            let dst: *mut u8 = bits.as_mut_ptr();
            std::ptr::write_bytes(dst, 0xFF, bytes);
        }
        Self {
            bits,
            len,
            all_set: Some(true),
        }
    }

    pub fn new_unset(len: usize) -> Self {
        let bytes = len.div_ceil(8);
        let mut bits = BytesMut::with_capacity(bytes);
        unsafe {
            let dst: *mut u8 = bits.as_mut_ptr();
            std::ptr::write_bytes(dst, 0x00, bytes);
        }
        Self {
            bits,
            len,
            all_set: Some(false),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.bits.len() * 8
    }

    pub fn push(&mut self, value: bool) {
        self.bits.reserve((self.len + 1).div_ceil(8));
        let idx = self.len / 8;
        let bit = self.len % 8;
        if value {
            self.bits[idx] |= 0x01 << bit;
        } else {
            self.bits[idx] &= !(0x01 << bit);
        }
        self.len += 1;
    }

    pub fn freeze(self) -> Mask {
        Mask {
            bits: self.bits.freeze(),
            len: self.len,
            all_set: self.all_set,
        }
    }
}

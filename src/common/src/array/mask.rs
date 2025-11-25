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
                    let last_byte = self.bits[full_bytes];
                    let mask = (1 << remainder_bits) - 1;
                    let relevant_bits = last_byte & mask;
                    count += remainder_bits - relevant_bits.count_ones() as usize;
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
        self.all_set.map(|x| !x).unwrap_or(false)
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
        bits.resize(bytes, 0xFF);
        Self {
            bits,
            len,
            all_set: Some(true),
        }
    }

    pub fn new_unset(len: usize) -> Self {
        let bytes = len.div_ceil(8);
        let mut bits = BytesMut::with_capacity(bytes);
        bits.resize(bytes, 0x00);
        Self {
            bits,
            len,
            all_set: Some(false),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.bits.len() * 8
    }

    pub fn push(&mut self, value: bool) {
        if self.len == 0 {
            self.all_set = Some(value);
        } else if self.all_set.is_some_and(|v| v != value) {
            self.all_set = None;
        }

        let byte_idx = self.len / 8;
        let bit_idx = self.len % 8;

        if bit_idx == 0 {
            self.bits.resize(byte_idx + 1, 0);
        }

        if value {
            self.bits[byte_idx] |= 0x01 << bit_idx;
        } else {
            self.bits[byte_idx] &= !(0x01 << bit_idx);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_new_set() {
        let mask = Mask::new_set(10);
        assert_eq!(mask.len(), 10);
        assert!(mask.all_set());
        for i in 0..10 {
            assert!(mask.get(i));
        }
        assert_eq!(mask.set_count(), 10);
        assert_eq!(mask.unset_count(), 0);
    }

    #[test]
    fn test_mask_new_unset() {
        let mask = Mask::new_unset(10);
        assert_eq!(mask.len(), 10);
        assert!(!mask.all_set());
        for i in 0..10 {
            assert!(!mask.get(i));
        }
        assert_eq!(mask.set_count(), 0);
        assert_eq!(mask.unset_count(), 10);
    }

    #[test]
    fn test_mask_empty() {
        let mask = Mask::empty();
        assert_eq!(mask.len(), 0);
        assert!(mask.is_empty());
    }

    #[test]
    fn test_mask_full() {
        let mask = Mask::full(true, 12);
        assert_eq!(mask.len(), 12);
        assert!(mask.all_set());
        assert_eq!(mask.set_count(), 12);

        let mask = Mask::full(false, 12);
        assert_eq!(mask.len(), 12);
        assert!(!mask.all_set());
        assert_eq!(mask.unset_count(), 12);
    }

    #[test]
    fn test_mask_from_mut() {
        let mut mask_mut = MaskMut::with_capacity(10);
        mask_mut.push(true);
        mask_mut.push(false);
        mask_mut.push(true);
        mask_mut.push(true);
        mask_mut.push(false);
        mask_mut.push(false);
        mask_mut.push(true);
        mask_mut.push(false);
        mask_mut.push(true);
        mask_mut.push(true);

        let mask = mask_mut.freeze();
        assert_eq!(mask.len(), 10);
        assert!(!mask.all_set());
        assert!(!mask.all_unset());

        assert!(mask.get(0));
        assert!(!mask.get(1));
        assert!(mask.get(2));
        assert!(mask.get(3));
        assert!(!mask.get(4));
        assert!(!mask.get(5));
        assert!(mask.get(6));
        assert!(!mask.get(7));
        assert!(mask.get(8));
        assert!(mask.get(9));

        assert_eq!(mask.set_count(), 6);
        assert_eq!(mask.unset_count(), 4);
    }

    #[test]
    fn test_mask_unset_count_partial_byte() {
        let mut mask_mut = MaskMut::with_capacity(3);
        mask_mut.push(true);
        mask_mut.push(false);
        mask_mut.push(true);

        let mask = mask_mut.freeze();
        assert_eq!(mask.len(), 3);
        assert_eq!(mask.unset_count(), 1);
        assert_eq!(mask.set_count(), 2);
    }

    #[test]
    fn test_mask_mut_push_all_set_logic() {
        let mut mask_mut = MaskMut::with_capacity(4);
        assert_eq!(mask_mut.all_set, None);

        mask_mut.push(true);
        assert_eq!(mask_mut.all_set, Some(true));
        mask_mut.push(true);
        assert_eq!(mask_mut.all_set, Some(true));

        let mask = mask_mut.freeze();
        assert!(mask.all_set());

        let mut mask_mut = MaskMut::with_capacity(4);
        mask_mut.push(false);
        assert_eq!(mask_mut.all_set, Some(false));
        mask_mut.push(false);
        assert_eq!(mask_mut.all_set, Some(false));
        let mask = mask_mut.freeze();
        assert!(!mask.all_set());
        assert!(mask.all_unset());

        let mut mask_mut = MaskMut::with_capacity(4);
        mask_mut.push(true);
        assert_eq!(mask_mut.all_set, Some(true));
        mask_mut.push(false);
        assert_eq!(mask_mut.all_set, None);
        let mask = mask_mut.freeze();
        assert!(!mask.all_set());
        assert!(!mask.all_unset());
    }

    #[test]
    fn test_mask_mut_new() {
        let mut mask_mut = MaskMut::new_set(5);
        assert_eq!(mask_mut.len(), 5);
        assert_eq!(mask_mut.all_set, Some(true));
        mask_mut.push(true);
        assert_eq!(mask_mut.len(), 6);
        assert_eq!(mask_mut.all_set, Some(true));
        mask_mut.push(false);
        assert_eq!(mask_mut.len(), 7);
        assert_eq!(mask_mut.all_set, None);

        let mut mask_mut = MaskMut::new_unset(5);
        assert_eq!(mask_mut.len(), 5);
        assert_eq!(mask_mut.all_set, Some(false));
        mask_mut.push(false);
        assert_eq!(mask_mut.len(), 6);
        assert_eq!(mask_mut.all_set, Some(false));
        mask_mut.push(true);
        assert_eq!(mask_mut.len(), 7);
        assert_eq!(mask_mut.all_set, None);
    }
}

use bitvec::vec::BitVec;

use crate::array::{Array, PhysicalType};

#[derive(Debug, Clone)]
pub struct BoolArray {
    data: BitVec,
    valid: BitVec,
}

impl Array for BoolArray {
    type RefItem<'a> = bool;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid
            .get(idx)
            .and_then(|valid| if *valid { Some(self.data[idx]) } else { None })
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn physical_type(&self) -> PhysicalType {
        PhysicalType::VirtualNode
    }
}

impl BoolArray {
    pub fn valid_map(&self) -> &BitVec {
        &self.valid
    }

    pub fn set_valid_map(&mut self, valid: BitVec) {
        self.valid = valid;
    }

    pub fn to_filter_mask(&self) -> BitVec {
        self.data.clone() & self.valid.clone()
    }
}

#[derive(Debug)]
pub struct BoolArrayBuilder {
    data: BitVec,
    valid: BitVec,
}

impl BoolArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BitVec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, value: Option<bool>, repeat: usize) {
        if let Some(value) = value {
            self.data.extend(std::iter::repeat_n(value, repeat));
            self.valid.extend(std::iter::repeat_n(true, repeat));
        } else {
            self.data.extend(std::iter::repeat_n(false, repeat));
            self.valid.extend(std::iter::repeat_n(false, repeat));
        }
    }

    pub fn push(&mut self, value: Option<bool>) {
        self.push_n(value, 1);
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> BoolArray {
        let data = self.data;
        let valid = self.valid;
        BoolArray { data, valid }
    }
}

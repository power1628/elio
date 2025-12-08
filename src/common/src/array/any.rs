use std::sync::Arc;

use bitvec::prelude::*;

use crate::array::PhysicalType;
use crate::array::datum::Datum;

#[derive(Clone)]
pub struct AnyArray {
    data: Arc<[Datum]>,
    valid: BitVec,
}

impl AnyArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::Any
    }

    pub fn valid_map(&self) -> &BitVec {
        &self.valid
    }

    pub fn set_valid_map(&mut self, valid: BitVec) {
        self.valid = valid;
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }
}

#[derive(Debug)]
pub struct AnyArrayBuilder {
    data: Vec<Datum>,
    valid: BitVec,
}

impl AnyArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, item: Option<&Datum>, repeat: usize) {
        if let Some(item) = item {
            self.data.extend(std::iter::repeat_n(item.to_owned(), repeat));
            self.valid.extend(std::iter::repeat_n(true, repeat));
        } else {
            self.data.extend(std::iter::repeat_n(Datum::default(), repeat));
            self.valid.extend(std::iter::repeat_n(false, repeat));
        }
    }

    pub fn push(&mut self, item: Option<&Datum>) {
        self.push_n(item, 1);
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> AnyArray {
        AnyArray {
            data: self.data.into(),
            valid: self.valid,
        }
    }
}

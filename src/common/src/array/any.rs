use std::iter;

use bitvec::prelude::*;

use crate::array::datum::Datum;

pub struct AnyArray {
    data: Box<[Datum]>,
    valid: BitVec,
}

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
            self.data.extend(iter::repeat(item.to_owned()).take(repeat));
            self.valid.extend(iter::repeat(true).take(repeat));
        } else {
            self.data.extend(iter::repeat(Datum::default()).take(repeat));
            self.valid.extend(iter::repeat(false).take(repeat));
        }
    }

    pub fn finish(self) -> AnyArray {
        AnyArray {
            data: self.data.into_boxed_slice(),
            valid: self.valid,
        }
    }
}

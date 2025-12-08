use std::sync::Arc;

use bitvec::prelude::*;

use crate::array::datum::{ScalarRef, ScalarValue};
use crate::array::{Array, ArrayBuilder, PhysicalType};

#[derive(Debug, Clone)]
pub struct AnyArray {
    data: Arc<[ScalarValue]>,
    valid: BitVec,
}

impl Array for AnyArray {
    type Builder = AnyArrayBuilder;
    type RefItem<'a> = ScalarRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).map(|_| ScalarRef::from(&self.data[idx]))
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn physical_type(&self) -> PhysicalType {
        PhysicalType::Any
    }
}

impl AnyArray {
    pub fn valid_map(&self) -> &BitVec {
        &self.valid
    }

    pub fn set_valid_map(&mut self, valid: BitVec) {
        self.valid = valid;
    }
}

#[derive(Debug)]
pub struct AnyArrayBuilder {
    data: Vec<ScalarValue>,
    valid: BitVec,
}

impl ArrayBuilder for AnyArrayBuilder {
    type Array = AnyArray;

    fn push_n(&mut self, item: Option<ScalarRef<'_>>, repeat: usize) {
        if let Some(item) = item {
            self.data.extend(std::iter::repeat_n(item.to_owned(), repeat));
            self.valid.extend(std::iter::repeat_n(true, repeat));
        } else {
            self.data.extend(std::iter::repeat_n(ScalarValue::default(), repeat));
            self.valid.extend(std::iter::repeat_n(false, repeat));
        }
    }

    fn finish(self) -> Self::Array {
        todo!()
    }
}

impl AnyArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, item: Option<&ScalarValue>, repeat: usize) {
        if let Some(item) = item {
            self.data.extend(std::iter::repeat_n(item.to_owned(), repeat));
            self.valid.extend(std::iter::repeat_n(true, repeat));
        } else {
            self.data.extend(std::iter::repeat_n(ScalarValue::default(), repeat));
            self.valid.extend(std::iter::repeat_n(false, repeat));
        }
    }

    pub fn push(&mut self, item: Option<&ScalarValue>) {
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

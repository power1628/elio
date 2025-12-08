use std::sync::Arc;

use bitvec::prelude::*;

use crate::array::{ArrayBuilderImpl, ArrayImpl, PhysicalType};

#[derive(Clone)]
pub struct ListArray {
    offsets: Arc<[usize]>,
    child: Box<ArrayImpl>,
    valid: BitVec,
}

impl ListArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::List(Box::new(self.child.physical_type()))
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
pub struct ListArrayBuilder {
    offsets: Vec<usize>,
    child: Box<ArrayBuilderImpl>,
    valid: BitVec,
}

impl ListArrayBuilder {
    pub fn new(child: Box<ArrayBuilderImpl>) -> Self {
        Self {
            offsets: vec![0],
            child,
            valid: BitVec::new(),
        }
    }

    pub fn child(&mut self) -> &mut ArrayBuilderImpl {
        &mut self.child
    }

    // push n element sizes
    pub fn push_n(&mut self, size: Option<usize>, repeat: usize) {
        let size = size.unwrap_or(0);
        // update the offset
        let last_offset = *self.offsets.last().unwrap();
        let to_extend = (0..repeat).scan(last_offset, |acc, _| {
            *acc += size;
            Some(*acc)
        });
        self.offsets.extend(to_extend);
    }

    pub fn push(&mut self, size: Option<usize>) {
        self.push_n(size, 1);
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> ListArray {
        let offsets = self.offsets.into();
        let child = Box::new(self.child.finish());
        let valid = self.valid;
        ListArray { offsets, child, valid }
    }
}

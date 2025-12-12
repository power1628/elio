use std::sync::Arc;

use bitvec::prelude::*;

use crate::array::datum::ListValueRef;
use crate::array::{Array, ArrayBuilderImpl, ArrayImpl, PhysicalType};

#[derive(Debug, Clone)]
pub struct ListArray {
    offsets: Arc<[usize]>,
    child: Arc<ArrayImpl>,
    valid: BitVec,
}

impl Array for ListArray {
    type RefItem<'a> = ListValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).and_then(|valid| {
            if *valid {
                let start = self.offsets[idx];
                let end = self.offsets[idx + 1];
                let child = self.child.as_ref();
                Some(ListValueRef::Index { child, start, end })
            } else {
                None
            }
        })
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn physical_type(&self) -> PhysicalType {
        PhysicalType::List(Box::new(self.child.physical_type()))
    }
}

impl ListArray {
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

    pub fn push_n(&mut self, item: Option<ListValueRef<'_>>, repeat: usize) {
        if let Some(item) = item {
            self.valid.extend(std::iter::repeat_n(true, repeat));
            for _ in 0..repeat {
                for value in item.iter() {
                    self.child.push(Some(value))
                }
                let last_offset = *self.offsets.last().unwrap();
                self.offsets.push(last_offset + item.len());
            }
        } else {
            self.valid.extend(std::iter::repeat_n(false, repeat));
            let last_offset = *self.offsets.last().unwrap();
            for _ in 0..repeat {
                self.offsets.push(last_offset);
            }
        }
    }

    pub fn push(&mut self, item: Option<ListValueRef<'_>>) {
        self.push_n(item, 1);
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> ListArray {
        let offsets = self.offsets.into();
        let child = self.child.finish().into();
        let valid = self.valid;
        ListArray { offsets, child, valid }
    }
}

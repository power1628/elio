use std::collections::HashMap;
use std::iter;
use std::sync::Arc;

use bitvec::prelude::*;
use itertools::Itertools;

use crate::array::{ArrayBuilderImpl, ArrayImpl};

pub struct StructArray {
    fields: Box<[(Arc<str>, ArrayImpl)]>,
    valid: BitVec,
}

pub struct StructArrayBuilder {
    fields: HashMap<Arc<str>, ArrayBuilderImpl>,
    valid: BitVec,
}

impl StructArrayBuilder {
    pub fn new(fields: impl Iterator<Item = (Arc<str>, ArrayBuilderImpl)>) -> Self {
        Self {
            fields: fields.collect(),
            valid: BitVec::new(),
        }
    }

    pub fn field_builder(&mut self, name: &str) -> &mut ArrayBuilderImpl {
        self.fields.get_mut(name).unwrap()
    }

    pub fn push_n(&mut self, valid: bool, repeat: usize) {
        self.valid.extend(iter::repeat(valid).take(repeat));
    }

    pub fn finish(self) -> StructArray {
        // sort by keys
        let mut fields = self
            .fields
            .into_iter()
            .map(|(k, v)| (k.clone(), v.finish()))
            .collect_vec();
        fields.sort_by_key(|(k, _)| k.clone());

        StructArray {
            fields: fields.into_boxed_slice(),
            valid: self.valid,
        }
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use bitvec::prelude::*;
use itertools::Itertools;

use crate::array::datum::StructValueRef;
use crate::array::{Array, ArrayBuilderImpl, ArrayRef, PhysicalType};

#[derive(Debug, Clone)]
pub struct StructArray {
    // We should guarantee that if parnet is null, then all the subfields must be null
    fields: Box<[(Arc<str>, ArrayRef)]>,
    valid: BitVec,
}

impl Array for StructArray {
    type RefItem<'a> = StructValueRef<'a>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.valid.get(idx).and_then(|valid| {
            if *valid {
                Some(StructValueRef::Index { array: self, idx })
            } else {
                None
            }
        })
    }

    fn len(&self) -> usize {
        self.valid.len()
    }

    fn physical_type(&self) -> PhysicalType {
        PhysicalType::Struct(
            self.fields
                .iter()
                .map(|(name, v)| (name.to_owned(), v.physical_type()))
                .collect_vec()
                .into_boxed_slice(),
        )
    }
}

impl StructArray {
    pub fn from_parts(fields: Box<[(Arc<str>, ArrayRef)]>, valid: BitVec) -> Self {
        Self { fields, valid }
    }

    pub fn fields(&self) -> &[(Arc<str>, ArrayRef)] {
        &self.fields
    }

    pub fn field_names(&self) -> impl Iterator<Item = &Arc<str>> + '_ {
        self.fields.iter().map(|(n, _)| n)
    }

    pub fn field_at_pos(&self, idx: usize) -> Option<&(Arc<str>, ArrayRef)> {
        self.fields.get(idx)
    }

    /// Return the field at given name.
    /// NOTICE: the valid map will not be joined
    pub fn field_at(&self, name: &str) -> Option<&ArrayRef> {
        self.fields.iter().find(|(n, _)| **n == *name).map(|(_, v)| v)
    }

    pub fn valid_map(&self) -> &BitVec {
        &self.valid
    }

    pub fn set_valid_map(&mut self, valid: BitVec) {
        self.valid = valid;
    }
}

#[derive(Debug)]
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

    pub fn field_at(&mut self, name: &str) -> &mut ArrayBuilderImpl {
        self.fields.get_mut(name).unwrap()
    }

    pub fn push_n(&mut self, valid: bool, repeat: usize) {
        self.valid.extend(std::iter::repeat_n(valid, repeat));
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> StructArray {
        // sort by keys
        let mut fields = self
            .fields
            .into_iter()
            .map(|(k, v)| (k.clone(), Arc::new(v.finish())))
            .collect_vec();
        fields.sort_by_key(|(k, _)| k.clone());

        StructArray {
            fields: fields.into_boxed_slice(),
            valid: self.valid,
        }
    }
}

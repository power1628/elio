use std::iter;

use bitvec::prelude::*;

use crate::array::datum::{RelValue, StructValue};
use crate::{NodeId, RelationshipId};

pub struct RelArray {
    ids: Box<[RelationshipId]>,
    reltypes: Box<[String]>,
    start_ids: Box<[NodeId]>,
    end_ids: Box<[NodeId]>,
    props: Box<[StructValue]>,
    valid: BitVec,
}

pub struct RelArrayBuilder {
    ids: Vec<RelationshipId>,
    reltypes: Vec<String>,
    start_ids: Vec<NodeId>,
    end_ids: Vec<NodeId>,
    props: Vec<StructValue>,
    valid: BitVec,
}

impl RelArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            reltypes: Vec::with_capacity(capacity),
            start_ids: Vec::with_capacity(capacity),
            end_ids: Vec::with_capacity(capacity),
            props: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, item: Option<&RelValue>, repeat: usize) {
        match item {
            Some(item) => {
                self.ids.extend(iter::repeat(item.id).take(repeat));
                self.reltypes.extend(iter::repeat(item.reltype.clone()).take(repeat));
                self.start_ids.extend(iter::repeat(item.start_id).take(repeat));
                self.end_ids.extend(iter::repeat(item.end_id).take(repeat));
                self.props.extend(iter::repeat(item.props.clone()));
                self.valid.extend(iter::repeat(true).take(repeat));
            }
            None => todo!(),
        }
    }

    pub fn finish(self) -> RelArray {
        let ids = self.ids.into_boxed_slice();
        let reltypes = self.reltypes.into_boxed_slice();
        let start_ids = self.start_ids.into_boxed_slice();
        let end_ids = self.end_ids.into_boxed_slice();
        let props = self.props.into_boxed_slice();
        let valid = self.valid;
        RelArray {
            ids,
            reltypes,
            start_ids,
            end_ids,
            props,
            valid,
        }
    }
}

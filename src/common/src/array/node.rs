use std::iter;
use std::iter::once;

use bitvec::prelude::*;

use crate::NodeId;
use crate::array::datum::{NodeValue, StructValue};

pub struct NodeArray {
    ids: Box<[NodeId]>,
    label_offsets: Box<[usize]>,
    label_values: Box<[String]>,
    props: Box<[StructValue]>,
    valid: BitVec,
}

pub struct NodeArrayBuilder {
    ids: Vec<NodeId>,
    labels: Vec<Vec<String>>,
    props: Vec<StructValue>,
    valid: BitVec,
}

impl NodeArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            labels: Vec::with_capacity(capacity),
            props: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, value: Option<&NodeValue>, repeat: usize) {
        if let Some(value) = value {
            self.ids.extend(iter::repeat(value.id).take(repeat));
            self.labels.extend(iter::repeat(value.labels.clone()).take(repeat));
            self.props.extend(iter::repeat(value.props.clone()).take(repeat));
            self.valid.extend(iter::repeat(true).take(repeat));
        } else {
            self.ids.extend(iter::repeat(NodeId::default()).take(repeat));
            self.labels.extend(iter::repeat(Vec::new()).take(repeat));
            self.props.extend(iter::repeat(StructValue::default()).take(repeat));
            self.valid.extend(iter::repeat(false).take(repeat));
        }
    }

    pub fn push(&mut self, value: Option<&NodeValue>) {
        self.push_n(value, 1);
    }

    pub fn finish(self) -> NodeArray {
        let ids = self.ids.into_boxed_slice();
        let label_offsets = once(0)
            .chain(self.labels.iter().scan(0, |acc, x| {
                *acc += x.len();
                let offset = *acc;
                Some(offset)
            }))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let label_values = self.labels.into_iter().flatten().collect::<Vec<_>>().into_boxed_slice();
        let props = self.props.into_boxed_slice();
        let valid = self.valid;
        NodeArray {
            ids,
            label_offsets,
            label_values,
            props,
            valid,
        }
    }
}

pub struct NodeIdArray {
    data: Box<[NodeId]>,
    valid: BitVec,
}

pub struct NodeIdArrayBuilder {
    data: Vec<NodeId>,
    valid: BitVec,
}

impl NodeIdArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, value: Option<&NodeId>, repeat: usize) {
        if let Some(value) = value {
            self.data.extend(iter::repeat(value).take(repeat));
            self.valid.extend(iter::repeat(true).take(repeat));
        } else {
            self.data.extend(iter::repeat(NodeId::default()).take(repeat));
            self.valid.extend(iter::repeat(false).take(repeat));
        }
    }

    pub fn push(&mut self, value: Option<&NodeId>) {
        self.push_n(value, 1);
    }

    pub fn finish(self) -> NodeIdArray {
        let data = self.data.into_boxed_slice();
        let valid = self.valid;
        NodeIdArray { data, valid }
    }
}

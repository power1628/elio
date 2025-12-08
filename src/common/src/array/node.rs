use std::iter;
use std::iter::once;
use std::sync::Arc;

use bitvec::prelude::*;

use crate::NodeId;
use crate::array::PhysicalType;
use crate::array::datum::{NodeValue, StructValue};

#[derive(Clone)]
pub struct NodeArray {
    ids: Arc<[NodeId]>,
    label_offsets: Arc<[usize]>,
    label_values: Arc<[String]>,
    props: Arc<[StructValue]>,
    valid: BitVec,
}
impl NodeArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::Node
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

    pub fn props_iter(&self) -> impl Iterator<Item = Option<&StructValue>> + '_ {
        self.valid
            .iter()
            .enumerate()
            .map(|(i, v)| if *v { Some(&self.props[i]) } else { None })
    }
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

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> NodeArray {
        let ids = self.ids.into();
        let label_offsets = once(0)
            .chain(self.labels.iter().scan(0, |acc, x| {
                *acc += x.len();
                let offset = *acc;
                Some(offset)
            }))
            .collect::<Vec<_>>()
            .into();
        let label_values = self.labels.into_iter().flatten().collect::<Vec<_>>().into();
        let props = self.props.into();
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

#[derive(Clone)]
pub struct VirtualNodeArray {
    data: Arc<[NodeId]>,
    valid: BitVec,
}

impl VirtualNodeArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::VirtualNode
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

pub struct VirtualNodeArrayBuilder {
    data: Vec<NodeId>,
    valid: BitVec,
}

impl VirtualNodeArrayBuilder {
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

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> VirtualNodeArray {
        let data = self.data.into();
        let valid = self.valid;
        VirtualNodeArray { data, valid }
    }
}

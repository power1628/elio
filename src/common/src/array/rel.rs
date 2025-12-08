use std::iter;
use std::sync::Arc;

use bitvec::prelude::*;

use crate::array::PhysicalType;
use crate::array::datum::{RelValue, StructValue, VirtualRel};
use crate::{NodeId, RelationshipId};

#[derive(Clone)]
pub struct RelArray {
    ids: Arc<[RelationshipId]>,
    reltypes: Arc<[String]>,
    start_ids: Arc<[NodeId]>,
    end_ids: Arc<[NodeId]>,
    props: Arc<[StructValue]>,
    valid: BitVec,
}

impl RelArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::Rel
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
        self.props
            .iter()
            .zip(self.valid.iter())
            .map(|(props, valid)| if *valid { Some(props) } else { None })
    }
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

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> RelArray {
        let ids = self.ids.into();
        let reltypes = self.reltypes.into();
        let start_ids = self.start_ids.into();
        let end_ids = self.end_ids.into();
        let props = self.props.into();
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

#[derive(Clone)]
pub struct VirtualRelArray {
    ids: Arc<[RelationshipId]>,
    reltypes: Arc<[String]>,
    start_ids: Arc<[NodeId]>,
    end_ids: Arc<[NodeId]>,
    valid: BitVec,
}

impl VirtualRelArray {
    pub fn physical_type(&self) -> PhysicalType {
        PhysicalType::VirtualRel
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

pub struct VirtualRelArrayBuilder {
    ids: Vec<RelationshipId>,
    reltypes: Vec<String>,
    start_ids: Vec<NodeId>,
    end_ids: Vec<NodeId>,
    valid: BitVec,
}

impl VirtualRelArrayBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            reltypes: Vec::with_capacity(capacity),
            start_ids: Vec::with_capacity(capacity),
            end_ids: Vec::with_capacity(capacity),
            valid: BitVec::with_capacity(capacity),
        }
    }

    pub fn push_n(&mut self, item: Option<&VirtualRel>, repeat: usize) {
        match item {
            Some(item) => {
                self.ids.extend(std::iter::repeat_n(item.id, repeat));
                self.reltypes.extend(std::iter::repeat_n(item.reltype.clone(), repeat));
                self.start_ids.extend(std::iter::repeat_n(item.start_id, repeat));
                self.end_ids.extend(std::iter::repeat_n(item.end_id, repeat));
                self.valid.extend(std::iter::repeat_n(true, repeat));
            }
            None => {
                self.ids.extend(std::iter::repeat_n(RelationshipId::default(), repeat));
                self.reltypes.extend(std::iter::repeat_n(String::default(), repeat));
                self.start_ids.extend(std::iter::repeat_n(NodeId::default(), repeat));
                self.end_ids.extend(std::iter::repeat_n(NodeId::default(), repeat));
                self.valid.extend(std::iter::repeat_n(false, repeat));
            }
        }
    }

    pub fn push(&mut self, item: Option<&VirtualRel>) {
        self.push_n(item, 1);
    }

    pub fn len(&self) -> usize {
        self.valid.len()
    }

    pub fn finish(self) -> VirtualRelArray {
        let ids = self.ids.into();
        let reltypes = self.reltypes.into();
        let start_ids = self.start_ids.into();
        let end_ids = self.end_ids.into();
        let valid = self.valid;
        VirtualRelArray {
            ids,
            reltypes,
            start_ids,
            end_ids,
            valid,
        }
    }
}

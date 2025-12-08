use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::{NodeId, RelationshipId};

#[derive(Debug, Clone, Default)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<String>,
    pub props: StructValue,
}

#[derive(Debug, Clone, Default)]
pub struct RelValue {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub props: StructValue,
}

#[derive(Debug, Clone, Default)]
pub struct VirtualRel {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
}

#[derive(Debug, Clone, Default)]
pub struct ListValue {
    values: Vec<Datum>,
}

#[derive(Debug, Clone, Default)]
pub struct StructValue {
    // fields must be unique and ordered
    fields: Vec<(Arc<str>, Datum)>,
}

impl StructValue {
    pub fn iter(&self) -> impl Iterator<Item = (&Arc<str>, &Datum)> {
        self.fields.iter().map(|(k, v)| (k, v))
    }

    pub fn field_at(&self, name: &str) -> Option<&Datum> {
        self.fields.iter().find(|(k, _)| **k == *name).map(|(_, v)| v)
    }
}

#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum Datum {
    // this is the place holder for null values
    #[default]
    Unknown,
    // primitives
    U16(u16),
    Integer(i64),
    Float(f64),
    String(String),
    // graph
    VirtualNode(NodeId),
    VirtualRel(VirtualRel),
    Node(Box<NodeValue>),
    Rel(Box<RelValue>),
    // nested
    List(Box<ListValue>),
    Struct(Box<StructValue>),
}

use std::sync::Arc;

use enum_as_inner::EnumAsInner;

use crate::{NodeId, RelationshipId};

//// scalar
///
#[derive(Clone, Default)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<String>,
    pub props: StructValue,
}

#[derive(Clone, Default)]
pub struct RelValue {
    pub id: RelationshipId,
    pub reltype: String,
    pub start_id: NodeId,
    pub end_id: NodeId,
    pub props: StructValue,
}

#[derive(Clone, Default)]
pub struct ListValue {
    values: Vec<Datum>,
}

#[derive(Clone, Default)]
pub struct StructValue {
    fields: Vec<(Arc<str>, Datum)>,
}

impl StructValue {
    pub fn iter(&self) -> impl Iterator<Item = (&Arc<str>, &Datum)> {
        self.fields.iter().map(|(k, v)| (k, v))
    }
}

#[derive(Clone, Default, EnumAsInner)]
pub enum Datum {
    // this is the place holder for null values
    #[default]
    Unknown,
    // primitives
    U16(u16),
    RelId(RelationshipId),
    NodeId(NodeId),
    Integer(i64),
    Float(f64),
    String(String),
    // graph
    Node(Box<NodeValue>),
    Rel(Box<RelValue>),
    // nested
    List(Box<ListValue>),
    Struct(Box<StructValue>),
}

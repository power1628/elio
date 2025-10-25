use crate::{LabelId, NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId};
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    // composite
    List(Vec<Value>),
    // map
    // structural
    Node(Box<NodeValue>),
    Relationship(Box<RelationshipValue>),
    // Path(Box<PathValue>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<LabelId>,
    pub properties: Vec<(PropertyKeyId, Value)>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RelationshipValue {
    pub id: RelationshipId,
    pub reltype: RelationshipTypeId,
    pub start: NodeId,
    pub end: NodeId,
    pub properties: Vec<(PropertyKeyId, Value)>,
}

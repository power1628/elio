use crate::{LabelId, NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId};

pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    // composite
    List(Vec<Value>),
    // map
    // structural
    Node(Box<NodeValue>),
    Relationship(Box<RelationshipValue>),
    // Path(Box<PathValue>),
}

pub struct NodeValue {
    pub id: NodeId,
    pub labels: Vec<LabelId>,
    pub properties: Vec<(PropertyKeyId, Value)>,
}

pub struct RelationshipValue {
    pub id: RelationshipId,
    pub reltype: RelationshipTypeId,
    pub start: NodeId,
    pub end: NodeId,
    pub properties: Vec<(PropertyKeyId, Value)>,
}

pub type NodeId = u64;
pub type RelationshipId = u64;

pub type LabelId = u32;
pub type RelationshipTypeId = u32;
pub type PropertyId = u32;

pub type PropertyKey = String;
pub type Label = String;
pub type RelationshipType = String;

pub enum PropertyValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

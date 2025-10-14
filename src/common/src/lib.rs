pub mod data_type;
pub mod store_types;
pub mod value;

pub type NodeId = u64;
pub type RelationshipId = u64;

pub type LabelId = u16;
pub type RelationshipTypeId = u16;
pub type PropertyKeyId = u16;

pub type PropertyKey = String;
pub type Label = String;
pub type RelationshipType = String;

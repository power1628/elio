pub mod data_type;
pub mod order;
pub mod schema;
pub mod store_types;
pub mod value;
pub mod variable;

pub type NodeId = u64;
pub type RelationshipId = u64;

pub type TokenId = u16;
pub type LabelId = TokenId;
pub type RelationshipTypeId = TokenId;
pub type PropertyKeyId = TokenId;

pub type PropertyKey = String;
pub type Label = String;
pub type RelationshipType = String;

pub enum EntityKind {
    Node,
    Rel,
}

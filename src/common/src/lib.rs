use enum_as_inner::EnumAsInner;

pub mod array;
pub mod data_type;
mod macros;
pub mod mapb;
pub mod order;
pub mod scalar;
pub mod schema;
pub mod store_types;
pub mod value;
pub mod variable;

// pub type NodeId = u64;
// pub type RelationshipId = u64;

pub type TokenId = u16;
pub type LabelId = TokenId;
pub type RelationshipTypeId = TokenId;
pub type PropertyKeyId = TokenId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumAsInner, derive_more::Display)]
pub enum IrToken {
    Resolved(TokenId),
    Unresolved(String),
}

impl From<TokenId> for IrToken {
    fn from(token: TokenId) -> Self {
        Self::Resolved(token)
    }
}

#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq)]
pub enum TokenKind {
    Label,
    RelationshipType,
    PropertyKey,
}

pub type PropertyKey = String;
pub type Label = String;
pub type RelationshipType = String;

pub enum EntityKind {
    Node,
    Rel,
}

#[derive(
    derive_more::Display,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(transparent)]
pub struct NodeId(pub u64);

impl NodeId {
    pub fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }
}

#[derive(
    derive_more::Display,
    derive_more::Deref,
    derive_more::From,
    derive_more::Into,
    Debug,
    Copy,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(transparent)]
pub struct RelationshipId(pub u64);

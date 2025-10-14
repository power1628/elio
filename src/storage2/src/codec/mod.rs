pub mod token;
pub use token::*;
pub mod node;
pub use node::*;
pub mod relationship;
pub use relationship::*;
pub mod property;
pub use property::*;
pub mod types;
pub use types::*;

pub mod topology;
pub use topology::*;

/// node and relationship properties are stored in the same column family.
/// and use different prefix to distinguish node and relationship.
pub const NODE_KEY_PREFIX: u8 = 0x01;
pub const REL_KEY_PREFIX: u8 = 0x02;

/// topology are stored in separated column family.
/// and prefixed by TOPO_KEY_PREFIX
pub const TOPO_KEY_PREFIX: u8 = 0x01;

pub mod codec;
pub mod dict;
pub mod error;
pub mod graph;
pub mod token;
pub mod transaction;

pub(crate) mod cf_meta {
    pub const CF_NAME: &str = "cf_meta";
    // token -> token_id
    pub(crate) const LABEL_KEY_PREFIX: u8 = 0x01;
    pub(crate) const RELTYPE_KEY_PREFIX: u8 = 0x02;
    pub(crate) const PROPERTY_KEY_PREFIX: u8 = 0x03;
    // id allocation
    pub(crate) const MAX_NODE_ID_KEY: &[u8; 1] = &[0x04];
    pub(crate) const MAX_REL_ID_KEY: &[u8; 1] = &[0x05];
}

pub(crate) mod cf_topology {
    pub const CF_NAME: &str = "cf_topology";
}

pub(crate) mod cf_property {
    pub const CF_NAME: &str = "cf_property";
    pub const NODE_KEY_PREFIX: u8 = 0x01;
}

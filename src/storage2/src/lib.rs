use crate::{graph::GraphStore, meta::MetaStore};

pub mod codec;
pub mod error;
pub mod graph;
pub mod meta;
pub mod transaction;

pub const CF_META: &str = "cf_meta";
pub const CF_TOPOLOGY: &str = "cf_topology";
pub const CF_PROPERTY: &str = "cf_property";

pub struct Store {
    graph: GraphStore,
    meta: MetaStore,
}

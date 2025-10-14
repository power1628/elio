use std::sync::Arc;

use rocksdb::{self};

pub mod graph_cf {
    pub const CF_META: &str = "cf_meta";
    pub const CF_TOPOLOGY: &str = "cf_topology";
    pub const CF_PROPERTY: &str = "cf_property";
}

pub struct GraphStore {
    // db: rocksdb::DBWithThreadMode<MultiThreaded>,
    db: Arc<rocksdb::TransactionDB>,
}

impl GraphStore {
    // TODO(power): return results
    pub fn open(path: &str) -> Self {
        let db = rocksdb::TransactionDB::open_default(path).unwrap();
        Self { db: Arc::new(db) }
    }
}

use std::sync::Arc;

use mojito_common::NodeId;
use rocksdb::{self};

use crate::transaction::Transaction;

pub mod graph_cf {
    pub const CF_META: &str = "cf_meta";
    pub const CF_TOPOLOGY: &str = "cf_topology";
    pub const CF_PROPERTY: &str = "cf_property";
}

pub struct GraphStore {
    db: Arc<rocksdb::TransactionDB>,
}

impl GraphStore {
    // TODO(power): return results
    pub fn open(path: &str) -> Self {
        let db = rocksdb::TransactionDB::open_default(path).unwrap();
        Self { db: Arc::new(db) }
    }

    pub fn transaction(&self) -> Transaction {
        Transaction::new(self.db.clone())
    }
}

pub struct PropertyStore {
    _db: Arc<rocksdb::TransactionDB>,
}

pub struct TopologyStore {
    _db: Arc<rocksdb::TransactionDB>,
}

pub struct DictStore {
    _db: Arc<rocksdb::TransactionDB>,
    // latest node id
    // latest rel id
    // cache
}

impl DictStore {
    pub fn alloc_node_id(&mut self) -> NodeId {
        // atomic add and put
    }
}

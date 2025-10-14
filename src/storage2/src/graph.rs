use std::sync::Arc;

use rocksdb::{self};

use crate::{dict::DictStore, meta::MetaStore, transaction::Transaction};

pub mod graph_cf {
    pub const CF_META: &str = "cf_meta";
    pub const CF_TOPOLOGY: &str = "cf_topology";
    pub const CF_PROPERTY: &str = "cf_property";
}

pub struct GraphStore {
    db: Arc<rocksdb::TransactionDB>,
    dict: Arc<DictStore>,
    meta: Arc<MetaStore>,
}

impl GraphStore {
    // TODO(power): return results
    pub fn open(path: &str) -> Self {
        let db: rocksdb::TransactionDB<rocksdb::MultiThreaded> = rocksdb::TransactionDB::open_default(path).unwrap();
        todo!()
        // Self { db: Arc::new(db) }
    }

    pub fn transaction(&self) -> Transaction {
        Transaction::new(self.db.clone(), self.dict.clone())
    }
}

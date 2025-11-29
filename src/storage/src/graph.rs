use std::sync::Arc;

use rocksdb::{self};

use crate::dict::IdStore;
use crate::token::TokenStore;
use crate::transaction::{Transaction, TransactionImpl};

// metadata store
pub const CF_META: &str = "cf_meta";
// relationships store
pub const CF_TOPOLOGY: &str = "cf_topology";
// node store
pub const CF_PROPERTY: &str = "cf_property";

pub struct GraphStore {
    db: Arc<rocksdb::TransactionDB>,
    dict: Arc<IdStore>,
    tokens: Arc<TokenStore>,
}

pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}

impl GraphStore {
    pub fn open(path: &str) -> Self {
        let _db: rocksdb::TransactionDB<rocksdb::MultiThreaded> = rocksdb::TransactionDB::open_default(path).unwrap();
        todo!()
        // initialization:
        // create column families
        // Self { db: Arc::new(db) }
    }

    pub fn transaction(&self) -> Arc<dyn Transaction> {
        Arc::new(TransactionImpl::new(self.db.clone(), self.dict.clone()))
    }
}

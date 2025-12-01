use std::sync::Arc;

use rocksdb::{self};

use crate::dict::IdStore;
use crate::error::GraphStoreError;
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
    token: Arc<TokenStore>,
}

pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}

impl GraphStore {
    pub fn open(path: &str) -> Result<Self, GraphStoreError> {
        let db: rocksdb::TransactionDB<rocksdb::MultiThreaded> = rocksdb::TransactionDB::open_default(path).unwrap();
        // create column families
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        db.create_cf(CF_META, &opts)?;
        db.create_cf(CF_TOPOLOGY, &opts)?;
        db.create_cf(CF_PROPERTY, &opts)?;
        let db = Arc::new(db);

        let dict = Arc::new(IdStore::new(db.clone())?);
        let token = Arc::new(TokenStore::new(db.clone())?);

        Ok(Self { db, dict, token })
    }

    pub fn token_store(&self) -> &Arc<TokenStore> {
        &self.token
    }

    pub fn transaction(&self) -> Arc<dyn Transaction> {
        Arc::new(TransactionImpl::new(self.db.clone(), self.dict.clone()))
    }
}

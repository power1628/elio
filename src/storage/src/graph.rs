use std::sync::Arc;

use rocksdb;
use rocksdb::{ColumnFamilyDescriptor, Options};

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
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(CF_META, Options::default()),
            ColumnFamilyDescriptor::new(CF_TOPOLOGY, Options::default()),
            ColumnFamilyDescriptor::new(CF_PROPERTY, Options::default()),
        ];
        let tx_db_opts = rocksdb::TransactionDBOptions::default();

        // create db and create cf if not exist
        let db = match rocksdb::TransactionDB::open_cf_descriptors(&opts, &tx_db_opts, path, cf_descriptors) {
            Ok(db) => db,
            Err(_) => {
                // if db not exists, create one
                let db = rocksdb::TransactionDB::open_default(path)?;

                // create cf
                let cf_opts = Options::default();
                db.create_cf(CF_META, &cf_opts)?;
                db.create_cf(CF_TOPOLOGY, &cf_opts)?;
                db.create_cf(CF_PROPERTY, &cf_opts)?;

                db
            }
        };

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

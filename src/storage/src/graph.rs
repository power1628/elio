use std::sync::Arc;

use rocksdb;
use rocksdb::{ColumnFamilyDescriptor, Options};

use crate::dict::IdStore;
use crate::error::GraphStoreError;
use crate::token::TokenStore;
use crate::transaction::TransactionImpl;
use crate::{cf_meta, cf_property, cf_topology};

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
            ColumnFamilyDescriptor::new(cf_meta::CF_NAME, Options::default()),
            ColumnFamilyDescriptor::new(cf_topology::CF_NAME, Options::default()),
            ColumnFamilyDescriptor::new(cf_property::CF_NAME, Options::default()),
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
                db.create_cf(cf_meta::CF_NAME, &cf_opts)?;
                db.create_cf(cf_topology::CF_NAME, &cf_opts)?;
                db.create_cf(cf_property::CF_NAME, &cf_opts)?;

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

    pub fn transaction(&self) -> Arc<TransactionImpl> {
        Arc::new(TransactionImpl::new(
            self.db.clone(),
            self.dict.clone(),
            self.token.clone(),
        ))
    }
}

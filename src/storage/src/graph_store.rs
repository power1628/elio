use std::sync::Arc;

use redb::{self, TableDefinition};

use crate::{error::GraphStoreError, transaction::GraphWrite};

pub const SCHEME_KEY_PREFIX: u8 = 0x00;

pub const NODEID_KEY: u8 = 0x04;
pub const RELID_KEY: u8 = 0x05;

// all data are stored in single key value store table.
pub const KVSTORE_TABLE_DEFINITION: TableDefinition<&'static [u8], &'static [u8]> = TableDefinition::new("default");

pub struct GraphStoreConfig {
    pub path: String,
}

pub struct GraphStore {
    db: Arc<redb::Database>,
}

impl GraphStore {
    pub fn open(config: &GraphStoreConfig) -> Result<Self, GraphStoreError> {
        let db = redb::Database::builder()
            .create(config.path.as_str())
            .map_err(Box::new)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn begin_write(&self) -> Result<GraphWrite, GraphStoreError> {
        GraphWrite::new(&self.db)
    }
}

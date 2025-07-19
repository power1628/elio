use std::path::Path;

use redb::{self, TableDefinition};

use crate::{error::GraphStoreError, transaction::GraphWriteTransaction};

pub const SCHEME_KEY_PREFIX: u8 = 0x00;

pub const TOKEN_LABEL_PREFIX: u8 = 0x01;
pub const TOKEN_RELTYPE_PREFIX: u8 = 0x02;
pub const TOKEN_PROPERTY_KEY_PREFIX: u8 = 0x03;

pub const NODEID_KEY: u8 = 0x04;
pub const RELID_KEY: u8 = 0x05;

pub const NODE_KEY_PREFIX: u8 = 0x06;

// all data are stored in single key value store table.
pub const KVSTORE_TABLE_DEFINITION: TableDefinition<&'static [u8], &'static [u8]> = TableDefinition::new("default");

pub struct GraphStore {
    db: redb::Database,
}

impl GraphStore {
    pub fn open(path: impl AsRef<Path>) -> Self {
        let db = redb::Database::open(path).unwrap();
        Self { db }
    }
}

use std::path::Path;

use redb;

use crate::{
    error::Error,
    transaction::{GraphReadTransaction, GraphWriteTransaction},
};

/// id allocator
pub const NODE_ID_TABLE_NAME: &str = "node_id";
pub const REL_ID_TABLE_NAME: &str = "rel_id";

/// token
pub const LABEL_TOKEN_TABLE_NAME: &str = "label_token";
pub const RELTYPE_TOKEN_TABLE_NAME: &str = "reltype_token";
pub const PROPERTY_KEY_TOKEN_TABLE_NAME: &str = "property_key_token";

/// graph data
pub const NODE_TABLE_NAME: &str = "node";

pub struct GraphStore {
    db: redb::Database,
}

impl GraphStore {
    pub fn open(path: impl AsRef<Path>) -> Self {
        let db = redb::Database::open(path).unwrap();
        Self { db }
    }

    pub fn read_transaction(&self) -> Result<GraphReadTransaction, Error> {
        let kv_tx = self
            .db
            .begin_read()
            .map_err(|e| Error::RedbTransactionError(Box::new(e)))?;
        Ok(GraphReadTransaction::new(kv_tx))
    }

    pub fn write_transaction(&self) -> Result<GraphWriteTransaction, Error> {
        let kv_tx = self
            .db
            .begin_write()
            .map_err(|e| Error::RedbTransactionError(Box::new(e)))?;
        Ok(GraphWriteTransaction::new(kv_tx))
    }
}

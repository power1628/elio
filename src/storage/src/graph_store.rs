use std::path::Path;

use redb;

use crate::{error::Error, transaction::GraphReadTransaction};

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
}

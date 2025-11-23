use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphStoreError {
    #[error("rocksdb error: {0}")]
    Rocksdb(#[from] rocksdb::Error),
    #[error("transaction commit error: {0}")]
    TxCommit(rocksdb::Error),
    #[error("transaction rollback error: {0}")]
    TxRollback(rocksdb::Error),
    #[error("internal error: {0}")]
    Internal(String),
}

impl GraphStoreError {
    pub fn tx_commit(e: rocksdb::Error) -> Self {
        Self::TxCommit(e)
    }

    pub fn internal<T: Display>(msg: T) -> Self {
        Self::Internal(msg.to_string())
    }
}

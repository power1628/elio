use rocksdb;
use std::{pin::Pin, sync::Arc};

use crate::{dict::DictStore, error::GraphStoreError};

mod node;
mod relationship;


pub struct Transaction {
    // NOTE: _db should be dropped at last
    _db: Arc<rocksdb::TransactionDB>,
    dict: Arc<DictStore>,
    inner: Pin<Box<rocksdb::Transaction<'static, rocksdb::TransactionDB>>>,
}

impl Transaction {
    pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<DictStore>) -> Self {
        let inner = db.transaction();
        // SAFETY: We extend the lifetime to 'static because we hold a reference to the DB
        // The DB will outlive the transaction due to the drop order
        let inner: rocksdb::Transaction<'static, rocksdb::TransactionDB> = unsafe { std::mem::transmute(inner) };
        Self {
            _db: db,
            dict,
            inner: Box::pin(inner),
        }
    }

    pub fn commit(self) -> Result<(), GraphStoreError> {
        let tx = unsafe { Pin::into_inner_unchecked(self.inner) };
        tx.commit().map_err(GraphStoreError::TxCommit)
    }

    pub fn rollback(self) -> Result<(), GraphStoreError> {
        let tx = unsafe { Pin::into_inner_unchecked(self.inner) };
        tx.rollback().map_err(GraphStoreError::TxCommit)
    }
}

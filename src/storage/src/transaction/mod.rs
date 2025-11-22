use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use mojito_common::array::chunk::DataChunk;
use rocksdb::{self};

use crate::dict::IdStore;
use crate::error::GraphStoreError;

// mod node;
// mod relationship;

pub struct RelScanOptions {}
pub struct NodeScanOptions {}

pub trait DataChunkIterator: Send {
    fn next_batch(&mut self) -> Result<Option<DataChunk>, GraphStoreError>;
}

pub trait Transaction {
    // readonly
    fn rel_scan(&self, opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError>;
    fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError>;
    // read-write
    fn node_create(&self, node: &DataChunk) -> Result<(), GraphStoreError>;
    fn relationship_create(&self, rel: &DataChunk) -> Result<(), GraphStoreError>;
    fn node_delete(&self, node: &DataChunk) -> Result<(), GraphStoreError>;
    fn relationship_delete(&self, rel: &DataChunk) -> Result<(), GraphStoreError>;
    // commit
    fn commit(self) -> Result<(), GraphStoreError>;
    fn abort(self) -> Result<(), GraphStoreError>;
}

/// impl Transaction for RoTransaction
pub struct RoTransaction {
    // snapshot
    inner: OwnedSnapshot,
    dict: Arc<IdStore>,
}

impl RoTransaction {
    pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>) -> Self {
        Self {
            inner: OwnedSnapshot::new(db),
            dict,
        }
    }
}

impl Transaction for RoTransaction {
    fn rel_scan(&self, opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_create(&self, node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn relationship_create(&self, rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn node_delete(&self, node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn relationship_delete(&self, rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn commit(self) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn abort(self) -> Result<(), GraphStoreError> {
        todo!()
    }
}

pub struct RwTransaction {
    // rocksdb transaction
    inner: OwnedTransaction,
    dict: Arc<IdStore>,
}

impl RwTransaction {
    pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>) -> Self {
        Self {
            inner: OwnedTransaction::new(db),
            dict,
        }
    }
}

impl Transaction for RwTransaction {
    fn rel_scan(&self, opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_create(&self, node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn relationship_create(&self, rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn node_delete(&self, node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn relationship_delete(&self, rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn commit(self) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn abort(self) -> Result<(), GraphStoreError> {
        todo!()
    }
}

struct OwnedTransaction {
    _db: Arc<rocksdb::TransactionDB>,
    tx: rocksdb::Transaction<'static, rocksdb::TransactionDB>,
}

impl OwnedTransaction {
    pub fn new(db: Arc<rocksdb::TransactionDB>) -> Self {
        unsafe {
            let tx = db.transaction();
            let static_tx: rocksdb::Transaction<'static, rocksdb::TransactionDB> = std::mem::transmute(tx);
            Self { _db: db, tx: static_tx }
        }
    }
}

impl Deref for OwnedTransaction {
    type Target = rocksdb::Transaction<'static, rocksdb::TransactionDB>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

impl DerefMut for OwnedTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tx
    }
}

unsafe impl Send for OwnedTransaction {}

struct OwnedSnapshot {
    _db: Arc<rocksdb::TransactionDB>,
    snapshot: rocksdb::Snapshot<'static>,
}

impl OwnedSnapshot {
    pub fn new(db: Arc<rocksdb::TransactionDB>) -> Self {
        unsafe {
            let snapshot = db.snapshot();
            let static_snapshot: rocksdb::Snapshot<'static> = std::mem::transmute(snapshot);
            Self {
                _db: db,
                snapshot: static_snapshot,
            }
        }
    }
}

impl Deref for OwnedSnapshot {
    type Target = rocksdb::Snapshot<'static>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

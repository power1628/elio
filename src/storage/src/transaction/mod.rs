use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use mojito_common::array::NodeIdArray;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::list::ListArray;
use mojito_common::array::prop_map::PropertyMapArray;

use crate::cf_property;
use crate::dict::IdStore;
use crate::error::GraphStoreError;

mod node;
// mod relationship;

pub struct RelScanOptions {}
pub struct NodeScanOptions {}

#[async_trait]
pub trait DataChunkIterator: Send {
    fn next_batch(&mut self) -> Result<Option<DataChunk>, GraphStoreError>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    // readonly
    fn rel_scan(&self, opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError>;
    fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError>;
    // read-write
    fn node_create(&self, label: &ListArray, prop: &PropertyMapArray) -> Result<NodeIdArray, GraphStoreError>;
    fn relationship_create(&self, rel: &DataChunk) -> Result<DataChunk, GraphStoreError>;
    fn node_delete(&self, node: &DataChunk) -> Result<(), GraphStoreError>;
    fn relationship_delete(&self, rel: &DataChunk) -> Result<(), GraphStoreError>;
    // commit
    fn commit(self) -> Result<(), GraphStoreError>;
    fn abort(self) -> Result<(), GraphStoreError>;
}

// Simple transaction implementation with snapshot and write batch buffer
pub struct TransactionImpl {
    inner: OwnedSnapshot,
    dict: Arc<IdStore>,
    // write buffer
    write_state: Mutex<WriteState>,
}

#[derive(Default)]
pub struct WriteState {
    pub(crate) batch: rocksdb::WriteBatch,
    // TODO(pgao): local buffer
    // local_cache: HashMap<Vec<u8>, Option<Vec<u8>>>,
}

impl TransactionImpl {
    pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>) -> Self {
        Self {
            inner: OwnedSnapshot::new(db),
            dict,
            write_state: WriteState::default().into(),
        }
    }
}

impl Transaction for TransactionImpl {
    fn rel_scan(&self, _opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_scan(&self, _opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_create(&self, _label: &ListArray, _prop: &PropertyMapArray) -> Result<NodeIdArray, GraphStoreError> {
        todo!()
    }

    fn relationship_create(&self, _rel: &DataChunk) -> Result<DataChunk, GraphStoreError> {
        todo!()
    }

    fn node_delete(&self, _node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn relationship_delete(&self, _rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn commit(self) -> Result<(), GraphStoreError> {
        todo!()
    }

    fn abort(self) -> Result<(), GraphStoreError> {
        todo!()
    }
}

// /// impl Transaction for RoTransaction
// pub struct RoTransaction {
//     // snapshot
//     inner: OwnedSnapshot,
//     dict: Arc<IdStore>,
// }

// impl RoTransaction {
//     pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>) -> Self {
//         Self {
//             inner: OwnedSnapshot::new(db),
//             dict,
//         }
//     }
// }

// impl Transaction for RoTransaction {
//     fn rel_scan(&self, _opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
//         todo!()
//     }

//     fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
//         batch_node_scan(&self.inner, opts)
//     }

//     fn node_create(&self, _label: &ListArray, _prop: &PropertyMapArray) -> Result<NodeIdArray, GraphStoreError> {
//         // readonly transaction, not allowed to create node
//         Err(GraphStoreError::internal(
//             "readonly transaction, not allowed to create node",
//         ))
//     }

//     fn relationship_create(&self, _rel: &DataChunk) -> Result<DataChunk, GraphStoreError> {
//         // readonly transaction, not allowed to create relationship
//         Err(GraphStoreError::internal(
//             "readonly transaction, not allowed to create relationship",
//         ))
//     }

//     fn node_delete(&self, _node: &DataChunk) -> Result<(), GraphStoreError> {
//         // readonly transaction, not allowed to delete node
//         Err(GraphStoreError::internal(
//             "readonly transaction, not allowed to delete node",
//         ))
//     }

//     fn relationship_delete(&self, _rel: &DataChunk) -> Result<(), GraphStoreError> {
//         // readonly transaction, not allowed to delete relationship
//         Err(GraphStoreError::internal(
//             "readonly transaction, not allowed to delete relationship",
//         ))
//     }

//     fn commit(self) -> Result<(), GraphStoreError> {
//         todo!()
//     }

//     fn abort(self) -> Result<(), GraphStoreError> {
//         todo!()
//     }
// }

// pub struct RwTransaction {
//     // rocksdb transaction
//     pub(crate) inner: OwnedTransaction,
//     dict: Arc<IdStore>,
// }

// impl RwTransaction {
//     pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>) -> Self {
//         Self {
//             inner: OwnedTransaction::new(db),
//             dict,
//         }
//     }
// }

// impl Transaction for RwTransaction {
//     fn rel_scan(&self, _opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
//         todo!()
//     }

//     fn node_scan(&self, opts: &NodeScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
//         batch_node_scan(&self.inner, opts)
//     }

//     fn node_create(&self, label: &ListArray, prop: &PropertyMapArray) -> Result<NodeIdArray, GraphStoreError> {
//         batch_node_create(self, label, prop)
//     }

//     fn relationship_create(&self, _rel: &DataChunk) -> Result<DataChunk, GraphStoreError> {
//         todo!()
//     }

//     fn node_delete(&self, _node: &DataChunk) -> Result<(), GraphStoreError> {
//         todo!()
//     }

//     fn relationship_delete(&self, _rel: &DataChunk) -> Result<(), GraphStoreError> {
//         todo!()
//     }

//     fn commit(self) -> Result<(), GraphStoreError> {
//         todo!()
//     }

//     fn abort(self) -> Result<(), GraphStoreError> {
//         todo!()
//     }
// }

pub(crate) struct OwnedTransaction {
    pub(crate) _db: Arc<rocksdb::TransactionDB>,
    pub(crate) tx: rocksdb::Transaction<'static, rocksdb::TransactionDB>,
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

pub trait TxRead {
    type DBAccess: rocksdb::DBAccess;
    /// full data scan, without seek
    fn full_iter(&self) -> rocksdb::DBIteratorWithThreadMode<'_, Self::DBAccess>;
}

impl TxRead for OwnedSnapshot {
    type DBAccess = rocksdb::DBWithThreadMode<rocksdb::MultiThreaded>;

    fn full_iter(&self) -> rocksdb::DBIteratorWithThreadMode<'_, Self::DBAccess> {
        self.iter()
    }
}

impl TxRead for OwnedTransaction {
    type DBAccess = rocksdb::Transaction<'static, rocksdb::TransactionDB>;

    fn full_iter(&self) -> rocksdb::DBIteratorWithThreadMode<'_, Self::DBAccess> {
        self.iter()
    }
}

impl OwnedSnapshot {
    pub fn iter(&self) -> rocksdb::DBIteratorWithThreadMode<'_, rocksdb::DB> {
        let cf = self._db.cf_handle(cf_property::CF_NAME).unwrap();
        let readopts = rocksdb::ReadOptions::default();
        let mode = rocksdb::IteratorMode::Start;
        self.snapshot.iterator_cf_opt(&cf, readopts, mode)
    }
}

impl OwnedTransaction {
    pub fn iter(&self) -> rocksdb::DBIteratorWithThreadMode<'_, rocksdb::Transaction<'static, rocksdb::TransactionDB>> {
        let cf = self._db.cf_handle(cf_property::CF_NAME).unwrap();
        let mode = rocksdb::IteratorMode::Start;
        self.tx.iterator_cf(&cf, mode)
    }
}

use std::ops::Deref;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayImpl, NodeArray, VirtualNodeArray};

use crate::dict::IdStore;
use crate::error::GraphStoreError;
use crate::token::TokenStore;
use crate::transaction::node::{batch_materialize_node, batch_node_create, batch_node_scan};

mod node;
// mod relationship;

pub struct RelScanOptions {}
pub struct NodeScanOptions {
    pub batch_size: usize,
}

#[async_trait]
pub trait DataChunkIterator: Send {
    fn next_batch(&mut self) -> Result<Option<DataChunk>, GraphStoreError>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    // readonly
    fn rel_scan(&self, opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError>;
    fn node_scan(&self, opts: NodeScanOptions) -> Result<Box<dyn DataChunkIterator + '_>, GraphStoreError>;
    fn materialize_node(&self, node: &VirtualNodeArray) -> Result<NodeArray, GraphStoreError>;
    // read-write
    fn node_create(&self, label: &[String], prop: &ArrayImpl) -> Result<NodeArray, GraphStoreError>;
    fn relationship_create(&self, rel: &DataChunk) -> Result<DataChunk, GraphStoreError>;
    fn node_delete(&self, node: &DataChunk) -> Result<(), GraphStoreError>;
    fn relationship_delete(&self, rel: &DataChunk) -> Result<(), GraphStoreError>;
    // commit
    fn commit(&self) -> Result<(), GraphStoreError>;
    fn abort(&self) -> Result<(), GraphStoreError>;
}

// Simple transaction implementation with snapshot and write batch buffer
pub struct TransactionImpl {
    pub(crate) inner: OwnedSnapshot,
    dict: Arc<IdStore>,
    token: Arc<TokenStore>,
    // write buffer
    write_state: Mutex<WriteState>,
}

#[derive(Default)]
pub struct WriteState {
    // TODO(pgao): should we use transaction db?
    pub(crate) batch: rocksdb::WriteBatchWithTransaction<true>,
    // TODO(pgao): local buffer
    // local_cache: HashMap<Vec<u8>, Option<Vec<u8>>>,
}

impl TransactionImpl {
    pub fn new(db: Arc<rocksdb::TransactionDB>, dict: Arc<IdStore>, token: Arc<TokenStore>) -> Self {
        Self {
            inner: OwnedSnapshot::new(db),
            dict,
            token,
            write_state: WriteState::default().into(),
        }
    }
}

impl Transaction for TransactionImpl {
    fn rel_scan(&self, _opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    fn node_scan(&self, opts: NodeScanOptions) -> Result<Box<dyn DataChunkIterator + '_>, GraphStoreError> {
        batch_node_scan(self, opts)
    }

    fn materialize_node(&self, node_ids: &VirtualNodeArray) -> Result<NodeArray, GraphStoreError> {
        batch_materialize_node(self, node_ids)
    }

    fn node_create(&self, label: &[String], prop: &ArrayImpl) -> Result<NodeArray, GraphStoreError> {
        batch_node_create(self, label, prop)
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

    fn commit(&self) -> Result<(), GraphStoreError> {
        let mut state = self.write_state.lock().unwrap();
        let batch = std::mem::take(&mut state.batch);
        self.inner._db.write(batch)?;
        Ok(())
    }

    fn abort(&self) -> Result<(), GraphStoreError> {
        let mut state = self.write_state.lock().unwrap();
        state.batch.clear();
        Ok(())
    }
}

struct OwnedSnapshot {
    pub(crate) _db: Arc<rocksdb::TransactionDB>,
    pub(crate) snapshot: rocksdb::Snapshot<'static>,
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

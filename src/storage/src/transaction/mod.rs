use std::ops::Deref;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayImpl, NodeArray, RelArray, StructArray, VirtualNodeArray};
use mojito_common::{NodeId, SemanticDirection, TokenId};

use crate::dict::IdStore;
use crate::error::GraphStoreError;
use crate::token::TokenStore;
use crate::transaction::node::{batch_materialize_node, batch_node_create, batch_node_scan};
use crate::transaction::relationship::{NodeIdContainer, RelIterForNode, batch_rel_create, rel_iter_for_node};

mod node;
mod relationship;

pub struct RelScanOptions {}
pub struct NodeScanOptions {
    pub batch_size: usize,
}

#[async_trait]
pub trait DataChunkIterator: Send {
    fn next_batch(&mut self) -> Result<Option<DataChunk>, GraphStoreError>;
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

// impl Transaction for TransactionImpl {
impl TransactionImpl {
    pub fn rel_scan(&self, _opts: &RelScanOptions) -> Result<Box<dyn DataChunkIterator>, GraphStoreError> {
        todo!()
    }

    pub fn node_scan(&self, opts: NodeScanOptions) -> Result<Box<dyn DataChunkIterator + '_>, GraphStoreError> {
        batch_node_scan(self, opts)
    }

    pub fn materialize_node(&self, node_ids: &VirtualNodeArray) -> Result<NodeArray, GraphStoreError> {
        batch_materialize_node(self, node_ids)
    }

    pub fn node_create(&self, label: &[Arc<str>], prop: &ArrayImpl) -> Result<NodeArray, GraphStoreError> {
        batch_node_create(self, label, prop)
    }

    pub fn relationship_create<A, B>(
        &self,
        rtype: &Arc<str>,
        start: &A,          // VirtualNode/Node
        end: &B,            // VirtualNode/Node
        prop: &StructArray, // Struct or Any
    ) -> Result<RelArray, GraphStoreError>
    where
        A: NodeIdContainer,
        B: NodeIdContainer,
    {
        batch_rel_create(self, rtype, start, end, prop)
    }

    pub fn node_delete(&self, _node: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn relationship_delete(&self, _rel: &DataChunk) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn rel_iter_for_node(
        &self,
        node_id: NodeId,
        dir: SemanticDirection,
        rtypes: &[TokenId],
    ) -> Result<RelIterForNode<'_>, GraphStoreError> {
        rel_iter_for_node(self, node_id, dir, rtypes)
    }

    pub fn commit(&self) -> Result<(), GraphStoreError> {
        let mut state = self.write_state.lock().unwrap();
        let batch = std::mem::take(&mut state.batch);
        self.inner._db.write(batch)?;
        Ok(())
    }

    pub fn abort(&self) -> Result<(), GraphStoreError> {
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

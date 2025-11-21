use std::sync::Arc;

use mojito_common::{NodeId, RelationshipId, RelationshipTypeId};

pub struct DictStore {
    _db: Arc<rocksdb::TransactionDB>,
    // latest node id
    // latest rel id
    // cache
}

impl DictStore {
    pub fn new(db: Arc<rocksdb::TransactionDB>) -> Self {
        Self { _db: db }
    }
    pub fn alloc_node_id(&self) -> NodeId {
        // atomic add and put
        todo!()
    }
    pub fn alloc_rel_id(&self, _reltype: RelationshipTypeId) -> RelationshipId {
        todo!()
    }
}

impl DictStore {
    fn load_from_db(&self) {
        todo!()
    }
}

use core::todo;

use crate::{
    error::Error,
    types::{Label, NodeId, PropertyKey, PropertyValue},
};

pub struct GraphReadTransaction {
    kv_tx: redb::ReadTransaction,
}

impl GraphReadTransaction {
    pub fn new(kv_tx: redb::ReadTransaction) -> Self {
        Self { kv_tx }
    }
}

pub struct GraphWriteTransaction {
    kv_tx: redb::WriteTransaction,
}

impl GraphWriteTransaction {
    pub fn new(kv_tx: redb::WriteTransaction) -> Self {
        Self { kv_tx }
    }
}

impl GraphWriteTransaction {
    pub fn create_node(
        &mut self,
        labels: Vec<Label>,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<NodeId, Error> {
        // TODO(pgao): impl
        todo!()
    }
}

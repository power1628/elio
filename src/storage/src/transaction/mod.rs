use std::{mem::ManuallyDrop, pin::Pin, sync::Arc};

use redb::Table;

use mojito_common::{Label, NodeId, PropertyKey, RelationshipId, RelationshipType};

mod id;
mod node;
mod relationship;
mod token;


use crate::{error::GraphStoreError, graph_store::KVSTORE_TABLE_DEFINITION, types::PropertyValue};

pub struct GraphWriteTransaction {
    kv_tx: redb::WriteTransaction,
}

impl GraphWriteTransaction {
    pub fn new(kv_tx: redb::WriteTransaction) -> Self {
        Self { kv_tx }
    }
}

impl GraphWriteTransaction {
    /// create node with given labels and properties,
    /// TODO(pgao): constraint checking
    pub fn node_create(
        &mut self,
        labels: Vec<Label>,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<NodeId, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    /// Delete the node with given id, return true if node exists.
    pub fn node_delete(&mut self, node_id: NodeId) -> Result<bool, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    /// Delete node and all associated relationships
    pub fn node_detach_delete(&mut self, node_id: NodeId) -> Result<bool, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_set_label(&mut self, node_id: NodeId, labels: Vec<Label>) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_remove_label(&mut self, node_id: NodeId, labels: Vec<Label>) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_set_property(
        &mut self,
        node_id: NodeId,
        key: PropertyKey,
        value: PropertyValue,
    ) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_remove_property(&mut self, node_id: NodeId, key: PropertyKey) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    /// create relationship between two nodes
    pub fn relationship_create(
        &mut self,
        src: NodeId,
        dst: NodeId,
        rel_type: RelationshipType,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<RelationshipId, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    /// delete a relationship
    pub fn relationship_delete(&mut self, relationship_id: RelationshipId) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn relationship_set_property(
        &mut self,
        relationship_id: RelationshipId,
        key: PropertyKey,
        value: PropertyValue,
    ) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn relationship_remove_property(
        &mut self,
        relationship_id: RelationshipId,
        key: PropertyKey,
    ) -> Result<(), GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }
}

pub struct GraphWrite {
    // pin the transaction to ensure stable memory address
    kv_tx: Pin<Box<redb::WriteTransaction>>,
    // SAFETY: table will be drop before kv_tx
    table: ManuallyDrop<Option<Table<'static, &'static [u8], &'static [u8]>>>,
}

impl GraphWrite {
    pub fn new(db: &Arc<redb::Database>) -> Result<Self, GraphStoreError> {
        let kv_tx = db.begin_write().map_err(Box::new)?;
        let mut container = Self {
            kv_tx: Box::pin(kv_tx),
            table: ManuallyDrop::new(None),
        };
        let tx_ref: &'static redb::WriteTransaction =
            unsafe { std::mem::transmute(container.kv_tx.as_ref().get_ref()) };
        let table = tx_ref.open_table(KVSTORE_TABLE_DEFINITION).map_err(Box::new)?;
        *container.table = Some(table);
        Ok(container)
    }

    pub fn table(&self) -> &Table<&'static [u8], &'static [u8]> {
        // safety: with new, table must be initialized
        self.table.as_ref().unwrap()
    }

    pub fn table_mut(&mut self) -> &mut Table<'static, &'static [u8], &'static [u8]> {
        // safety: with new, table must be initialized
        self.table.as_mut().unwrap()
    }
}

impl Drop for GraphWrite {
    fn drop(&mut self) {
        // manually drop table first
        unsafe {
            ManuallyDrop::drop(&mut self.table);
        }
        // kv_tx will be dropped after
    }
}

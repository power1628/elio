use mojito_common::{LabelId, NodeId, PropertyKeyId, store_types::PropertyValue, value::Value};

use crate::{error::GraphStoreError, transaction::Transaction};

impl Transaction {
    pub fn node_create(
        &self,
        labels: Vec<LabelId>,
        props: Vec<(PropertyKeyId, Value)>,
    ) -> Result<NodeId, GraphStoreError> {
        todo!()
    }

    pub fn node_delete(&self, node_id: NodeId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    /// return number of deleted relationships
    pub fn node_detach_delete(&self, node_id: NodeId) -> Result<u64, GraphStoreError> {
        todo!()
    }

    /// return true if label added
    pub fn node_add_label(&self, node_id: NodeId, label: LabelId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn node_remove_label(&self, node_id: NodeId, label: LabelId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn node_set_property(&self, key: PropertyKeyId, value: PropertyValue) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn node_remove_property(&self, key: PropertyKeyId) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn node_apply_changes(
        &self,
        added_label: Vec<LabelId>,
        removed_label: Vec<LabelId>,
        properties: Vec<(PropertyKeyId, PropertyValue)>,
    ) -> Result<(), GraphStoreError> {
        todo!()
    }
}

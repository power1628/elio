use mojito_common::{LabelId, NodeId, PropertyKeyId, store_types::PropertyValue, value::Value};

use crate::{error::GraphStoreError, transaction::Transaction};

impl Transaction {
    pub fn node_create(
        &self,
        _labels: Vec<LabelId>,
        _props: Vec<(PropertyKeyId, Value)>,
    ) -> Result<NodeId, GraphStoreError> {
        todo!()
    }

    pub fn node_delete(&self, _node_id: NodeId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    /// return number of deleted relationships
    pub fn node_detach_delete(&self, _node_id: NodeId) -> Result<u64, GraphStoreError> {
        todo!()
    }

    /// return true if label added
    pub fn node_add_label(&self, _node_id: NodeId, _label: LabelId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn node_remove_label(&self, _node_id: NodeId, _label: LabelId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn node_set_property(&self, _key: PropertyKeyId, _value: PropertyValue) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn node_remove_property(&self, _key: PropertyKeyId) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn node_apply_changes(
        &self,
        _added_label: Vec<LabelId>,
        _removed_label: Vec<LabelId>,
        _properties: Vec<(PropertyKeyId, PropertyValue)>,
    ) -> Result<(), GraphStoreError> {
        todo!()
    }
}

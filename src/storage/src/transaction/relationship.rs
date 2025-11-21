use mojito_common::{
    NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId, store_types::PropertyValue, value::Value,
};

use crate::{error::GraphStoreError, transaction::Transaction};

impl Transaction {
    pub fn relationship_create(
        &self,
        _rel_type: RelationshipTypeId,
        _start_node_id: NodeId,
        _end_node_id: NodeId,
        _props: Vec<(PropertyKeyId, Value)>,
    ) -> Result<RelationshipId, GraphStoreError> {
        todo!()
    }

    /// return true on relationship deleted
    pub fn relationship_delete(&self, _rel_id: RelationshipId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn relationship_set_property(&self, _key: PropertyKeyId, _value: PropertyValue) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn relationship_remove_property(&self, _key: PropertyKeyId) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn relationship_apply_changes(
        &self,
        _rel_id: RelationshipId,
        _props: Vec<(PropertyKeyId, PropertyValue)>,
    ) -> Result<(), GraphStoreError> {
        todo!()
    }
}

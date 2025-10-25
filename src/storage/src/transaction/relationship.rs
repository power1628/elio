use mojito_common::{
    NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId, store_types::PropertyValue, value::Value,
};

use crate::{error::GraphStoreError, transaction::Transaction};

impl Transaction {
    pub fn relationship_create(
        &self,
        rel_type: RelationshipTypeId,
        start_node_id: NodeId,
        end_node_id: NodeId,
        props: Vec<(PropertyKeyId, Value)>,
    ) -> Result<RelationshipId, GraphStoreError> {
        todo!()
    }

    /// return true on relationship deleted
    pub fn relationship_delete(&self, rel_id: RelationshipId) -> Result<bool, GraphStoreError> {
        todo!()
    }

    pub fn relationship_set_property(&self, key: PropertyKeyId, value: PropertyValue) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn relationship_remove_property(&self, key: PropertyKeyId) -> Result<(), GraphStoreError> {
        todo!()
    }

    pub fn relationship_apply_changes(
        &self,
        rel_id: RelationshipId,
        props: Vec<(PropertyKeyId, PropertyValue)>,
    ) -> Result<(), GraphStoreError> {
        todo!()
    }
}

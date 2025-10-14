use mojito_common::{NodeId, PropertyKeyId, RelationshipId, RelationshipTypeId, value::Value};

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
}

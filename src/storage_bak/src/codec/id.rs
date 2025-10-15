use crate::error::GraphStoreError;
use mojito_common::{NodeId, RelationshipId};

pub struct IdReader;

impl IdReader {
    pub fn as_node_id(data: &[u8]) -> Result<NodeId, GraphStoreError> {
        let value = data[..size_of::<NodeId>()]
            .try_into()
            .map_err(|_| GraphStoreError::ill_formatted_data("invalid node id", data.into()))?;
        Ok(NodeId::from_le_bytes(value))
    }

    pub fn as_relationship_id(data: &[u8]) -> Result<RelationshipId, GraphStoreError> {
        let value = data[..size_of::<RelationshipId>()]
            .try_into()
            .map_err(|_| GraphStoreError::ill_formatted_data("invalid relationship id", data.into()))?;
        Ok(RelationshipId::from_le_bytes(value))
    }
}

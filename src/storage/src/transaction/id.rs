use redb::ReadableTable;

use crate::{
    codec::IdReader,
    error::GraphStoreError,
    graph_store::{NODEID_KEY, RELID_KEY},
    transaction::GraphWrite,
    types::{NodeId, RelationshipId},
};

impl GraphWrite {
    pub fn alloc_node_id(&mut self) -> Result<NodeId, GraphStoreError> {
        alloc_node_id(self)
    }

    pub fn alloc_relationship_id(&mut self) -> Result<RelationshipId, GraphStoreError> {
        alloc_relationship_id(self)
    }
}

fn alloc_node_id(tx: &mut GraphWrite) -> Result<NodeId, GraphStoreError> {
    let key = [NODEID_KEY];
    let table = tx.table_mut();
    let old_value = {
        match table.get(key.as_ref()).map_err(Box::new)? {
            Some(old_value) => IdReader::as_node_id(old_value.value())?,
            None => 0,
        }
    };
    let new_value = old_value + 1;
    let new_value = new_value.to_le_bytes();
    table.insert(key.as_ref(), new_value.as_ref()).map_err(Box::new)?;
    Ok(old_value)
}

fn alloc_relationship_id(tx: &mut GraphWrite) -> Result<RelationshipId, GraphStoreError> {
    let key = [RELID_KEY];
    let table = tx.table_mut();
    let old_value = {
        match table.get(key.as_ref()).map_err(Box::new)? {
            Some(old_value) => IdReader::as_relationship_id(old_value.value())?,
            None => 0,
        }
    };
    let new_value = old_value + 1;
    let new_value = new_value.to_le_bytes();
    table.insert(key.as_ref(), new_value.as_ref()).map_err(Box::new)?;
    Ok(old_value)
}

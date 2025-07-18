use core::todo;

use crate::{
    error::Error,
    types::{Label, NodeId, PropertyKey, PropertyValue, RelationshipId, RelationshipType, RelationshipTypeId},
};

pub trait GraphRead {}

pub trait GraphWrite: GraphRead {}

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
    /// create node with given labels and properties,
    /// TODO(pgao): constraint checking
    pub fn node_create(
        &mut self,
        labels: Vec<Label>,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<NodeId, Error> {
        // TODO(pgao): impl
        todo!()
    }

    /// Delete the node with given id, return true if node exists.
    pub fn node_delete(&mut self, node_id: NodeId) -> Result<bool, Error> {
        // TODO(pgao): impl
        todo!()
    }

    /// Delete node and all associated relationships
    pub fn node_detach_delete(&mut self, node_id: NodeId) -> Result<bool, Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_set_label(&mut self, node_id: NodeId, labels: Vec<Label>) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_remove_label(&mut self, node_id: NodeId, labels: Vec<Label>) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_set_property(&mut self, node_id: NodeId, key: PropertyKey, value: PropertyValue) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn node_remove_property(&mut self, node_id: NodeId, key: PropertyKey) -> Result<(), Error> {
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
    ) -> Result<RelationshipId, Error> {
        // TODO(pgao): impl
        todo!()
    }

    /// delete a relationship
    pub fn relationship_delete(&mut self, relationship_id: RelationshipId) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn relationship_set_property(
        &mut self,
        relationship_id: RelationshipId,
        key: PropertyKey,
        value: PropertyValue,
    ) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn relationship_remove_property(
        &mut self,
        relationship_id: RelationshipId,
        key: PropertyKey,
    ) -> Result<(), Error> {
        // TODO(pgao): impl
        todo!()
    }
}

/// Id generator
impl GraphWriteTransaction {
    fn node_id_alloc(&mut self) -> Result<NodeId, Error> {
        // TODO(pgao): impl
        todo!()
    }

    fn relationship_id_alloc(&mut self) -> Result<RelationshipId, Error> {
        // TODO(pgao): impl
        todo!()
    }
}

/// Token
impl GraphWriteTransaction {
    fn 
}



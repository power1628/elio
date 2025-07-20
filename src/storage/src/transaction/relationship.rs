//! Relationships
//! Incoming and outgoing.
//!
//! Relationship contains the following info
//!  - owned node id
//!  - other node id
//!  - reltype id
//!  - properties
//!
//! And Incoming/Outgoign relationships forms an array.
//! they share the same owned node id.
//!
//! When insert releationship, it will insert to both end.
//!
//! When delete relationship, it will delete from both end.
//!
//! For incoming relationship. the direction will be other node id -> owned node id
//! When relationships becomes too large, we should be able to split into smaller chunks.

use mojito_common::{NodeId, PropertyKey, RelationshipId, RelationshipType};

use crate::{error::GraphStoreError, transaction::GraphWrite, types::PropertyValue};

impl GraphWrite {
    pub fn relationship_create(
        &mut self,
        src: NodeId,
        dst: NodeId,
        reltype: &RelationshipType,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<RelationshipId, GraphStoreError> {
        todo!()
    }
}

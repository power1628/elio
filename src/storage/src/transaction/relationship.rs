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

use bytes::BytesMut;
use mojito_common::{NodeId, PropertyKey, RelationshipId, RelationshipType};
use redb::ReadableTable;

use crate::{
    codec::{NodeUpdater, PropertyWriter, RelationshipFormatWriter},
    error::GraphStoreError,
    model::node_key,
    transaction::GraphWrite,
    types::{PropertyValue, RelationshipDirection},
};

impl GraphWrite {
    // SAFETY: src and dst must be valid node id
    pub fn relationship_create(
        &mut self,
        src: NodeId,
        dst: NodeId,
        reltype: &RelationshipType,
        properties: Vec<(PropertyKey, PropertyValue)>,
    ) -> Result<RelationshipId, GraphStoreError> {
        let reltype_id = self.register_reltype(reltype)?;
        let relationship_id = self.alloc_relationship_id()?;

        // serialize properties
        let prop_buf = {
            let mut keys = vec![];
            let mut vals = vec![];
            for (key, val) in properties {
                keys.push(self.register_property_key(&key)?);
                vals.push(val);
            }
            let mut buf = BytesMut::new();
            let mut writer = PropertyWriter::new(&mut buf, keys.len());
            for (k, v) in keys.into_iter().zip(vals) {
                writer.add_property(&k, &v);
            }
            writer.finish();
            buf
        };

        // add src -> dst
        let outgoing = {
            let mut buf = BytesMut::new();
            let mut writer = RelationshipFormatWriter::new(&mut buf);
            writer.write(
                reltype_id,
                dst,
                relationship_id,
                RelationshipDirection::Outgoing,
                &prop_buf,
            );
            buf
        };
        self.add_relationship(src, &outgoing)?;

        // add dst <- src
        let incomming = {
            let mut buf = BytesMut::new();
            let mut writer = RelationshipFormatWriter::new(&mut buf);
            writer.write(
                reltype_id,
                src,
                relationship_id,
                RelationshipDirection::Incoming,
                &prop_buf,
            );
            buf
        };
        self.add_relationship(dst, &incomming)?;
        Ok(relationship_id)
    }
}

impl GraphWrite {
    fn add_relationship(&mut self, owned_id: NodeId, rel_buf: &[u8]) -> Result<(), GraphStoreError> {
        let table = self.table_mut();
        let key = node_key(owned_id);

        // SAFETY: owned_id exists
        let val = table.get(key.as_slice()).map_err(Box::new)?.unwrap();

        // TODO(pgao): double check the memory here
        // redb value() function will return an 'static [u8], and BytesMut will deep copy it.
        let mut node_updater = NodeUpdater::new(BytesMut::from(val.value()));
        node_updater.add_relationship(rel_buf);
        let new_node_value = node_updater.finish();
        drop(val);

        table
            .insert(key.as_slice(), new_node_value.as_ref())
            .map_err(Box::new)?;
        Ok(())
    }
}

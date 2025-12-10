//! Relationship stored in key-value database.
//!
//! RelationshipKey ::= <cf_topology::REL_KEY_PREFIX> <src_node_id> <DIRECTION> <reltype_id> <dst_node_id> <rel_id>
//! DIRECTION ::= <cf_topology::DIR_OUT> | <cf_topology::DIR_IN>
//!
//! RelationshipValue ::= <PropertyBlock>

use bytes::{BufMut, Bytes, BytesMut};
use mojito_common::array::datum::StructValueRef;
use mojito_common::mapb::{PropertyMapMut, PropertyMapRef};
use mojito_common::store_types::RelDirection;
use mojito_common::{NodeId, RelationshipId, TokenId};

use crate::cf_topology;

pub struct RelFormat;

impl RelFormat {
    pub fn encode_key(
        src_node_id: NodeId,
        direction: RelDirection,
        reltype: TokenId,
        dst_node_id: NodeId,
        rel_id: RelationshipId,
    ) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(cf_topology::REL_KEY_PREFIX);
        bytes.put_u64(*src_node_id);
        bytes.put_u8(direction as u8);
        bytes.put_u16(reltype);
        bytes.put_u64(*dst_node_id);
        bytes.put_u64(*rel_id);
        bytes.freeze()
    }

    pub fn decode_key(buf: &[u8]) -> (NodeId, RelDirection, TokenId, NodeId, RelationshipId) {
        assert_eq!(buf.len(), 25);
        let src_node_id = NodeId::from_be_bytes(buf[1..9].try_into().unwrap());
        let direction = RelDirection::from(buf[9]);
        let reltype = TokenId::from_be_bytes(buf[10..12].try_into().unwrap());
        let dst_node_id = NodeId::from_be_bytes(buf[12..20].try_into().unwrap());
        let rel_id = RelationshipId::from_be_bytes(buf[20..28].try_into().unwrap());
        (src_node_id, direction, reltype, dst_node_id, rel_id)
    }

    pub fn encode_value(key_ids: &[TokenId], property_map: StructValueRef<'_>) -> Result<Bytes, String> {
        let mut buf = BytesMut::new();
        let mut mapb_mut = PropertyMapMut::with_capacity(key_ids.len());

        assert_eq!(key_ids.len(), property_map.len());

        // put properties
        for (key_id, (_, prop)) in key_ids.iter().zip(property_map.iter()) {
            mapb_mut.insert(*key_id, Some(&prop))?;
        }
        mapb_mut.freeze().write(&mut buf);
        Ok(buf.freeze())
    }

    pub fn decode_value(buf: &[u8]) -> Result<PropertyMapRef<'_>, String> {
        Ok(PropertyMapRef::new(buf))
    }
}

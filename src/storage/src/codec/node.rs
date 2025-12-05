//！NodeKey ::= <node_id>
//! NodeValue ::= <PropertyBlock>

use bytes::{BufMut, Bytes, BytesMut};
use mojito_common::scalar::PropertyMapValueRef;
use mojito_common::{LabelId, NodeId};

pub struct NodeFormat;

impl NodeFormat {
    pub fn encode_node_key(node_id: NodeId) -> Bytes {
        let mut key = BytesMut::new();
        key.put_slice(crate::cf_property::NODE_KEY_PREFIX);
        key.put_u64_le(*node_id);
        key.freeze()
    }

    pub fn decode_node_key(buf: &[u8]) -> NodeId {
        assert_eq!(buf.len(), 9);
        NodeId::from_le_bytes(buf[1..9].try_into().unwrap())
    }

    pub fn encode_node_value(labels: &[LabelId], property_map: PropertyMapValueRef<'_>) -> Bytes {
        let mut buf = BytesMut::new();
        // put header
        buf.put_u16_le(labels.len() as u16);
        buf.put_u32_le(property_map.bytes() as u32);
        // put labels
        for label_id in labels {
            buf.put_u16_le(*label_id);
        }
        // put properties
        (*property_map).to_owned().write(&mut buf);
        buf.freeze()
    }
}

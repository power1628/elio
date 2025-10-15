//! Relationship
//!
//! RelationshipKey ::= <relationship_id>
//!
//! RelationshipValue ::= <PropertyBlock>

use bytes::{BufMut, Bytes, BytesMut};
use mojito_common::{PropertyKeyId, RelationshipId, store_types::PropertyValue};

use crate::codec::{PropertyWriter, REL_KEY_PREFIX};

pub struct RelationshipFormat;

impl RelationshipFormat {
    pub fn encode_relationship_key(relid: RelationshipId) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(REL_KEY_PREFIX);
        bytes.put_u64_le(relid);
        bytes.freeze()
    }

    pub fn decode_relationship_key(buf: &[u8]) -> RelationshipId {
        assert_eq!(buf.len(), 9);
        u64::from_le_bytes(buf[1..9].try_into().unwrap())
    }
}

pub struct RelationshipFormatWriter<'a> {
    buf: &'a mut BytesMut,
}

impl<'a> RelationshipFormatWriter<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        Self { buf }
    }

    pub fn write_properties(&mut self, keys: &[PropertyKeyId], values: &[PropertyValue]) {
        let mut writer = PropertyWriter::new(self.buf, keys.len());

        for (key, value) in keys.iter().zip(values) {
            writer.add_property(key, value);
        }
    }
}

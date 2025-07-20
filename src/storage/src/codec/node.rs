//！NodeKey ::= <node_id>
//!
//! NodeValue ::= <NodeHeader> <LabelBlocks> <PropertyBlock> <RelationshipBlock>
//!
//! Nodeheader ::= <NumLabels> <PropertySize> <RelationshipSize>
//!
//! NumLabels ::= u16
//!
//! LabelBlock ::= <LabelId>{NumLabels}
//!
//! PropertySize ::= u32 // size of property block
//!
//! RelationshipSize ::= u32 // size of relationship block
//!

use bytes::{BufMut, BytesMut};
use mojito_common::{LabelId, PropertyKeyId};

use crate::{codec::PropertyWriter, types::PropertyValue};

const NUM_LABELS_SIZE: usize = 2;
const PROPERTY_SIZE_SIZE: usize = 4;
const RELATIONSHIP_SIZE_SIZE: usize = 4;
const NODE_HEADER_SIZE: usize = NUM_LABELS_SIZE + PROPERTY_SIZE_SIZE + RELATIONSHIP_SIZE_SIZE;

pub struct NodeFormat;

#[repr(C, packed(1))]
pub struct NodeFormatHeader {
    num_labels: u16,
    property_size: u32,
    relationship_size: u32,
}

pub struct NodeWriter<'a> {
    buf: &'a mut BytesMut,
    offset: usize,
}
impl<'a> NodeWriter<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        let offset = buf.len();
        buf.reserve(NODE_HEADER_SIZE);
        // init header
        buf.put_bytes(0, NODE_HEADER_SIZE);
        Self { buf, offset }
    }

    fn set_num_labels(&mut self, num_labels: u16) {
        let header = unsafe { &mut *(self.buf.as_mut_ptr().add(self.offset) as *mut NodeFormatHeader) };
        header.num_labels = num_labels;
    }

    fn set_property_size(&mut self, property_size: u32) {
        let header = unsafe { &mut *(self.buf.as_mut_ptr().add(self.offset) as *mut NodeFormatHeader) };
        header.property_size = property_size;
    }

    // append labels and update header
    pub fn write_labels(&mut self, label_ids: &[LabelId]) {
        self.buf.reserve(label_ids.len() * 4);
        for label_id in label_ids {
            self.buf.put_u16_le(*label_id);
        }
        self.set_num_labels(label_ids.len() as u16);
    }

    // write properties
    pub fn write_properties(&mut self, keys: &[PropertyKeyId], values: &[PropertyValue]) {
        let mut writer = PropertyWriter::new(self.buf, keys.len());

        for (key, value) in keys.iter().zip(values) {
            writer.add_property(key, value);
        }

        let property_size = writer.finish();
        self.set_property_size(property_size as u32);
    }

    pub fn finish(self) -> usize {
        self.buf.len() - self.offset
    }
}

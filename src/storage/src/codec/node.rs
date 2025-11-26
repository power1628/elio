//！NodeKey ::= <node_id>
//! NodeValue ::= <NodeHeader> <LabelBlocks> <PropertyBlock>
//!
//! Nodeheader ::= <NumLabels> <PropertySize>
//!
//! NumLabels ::= u16
//!
//! LabelBlock ::= <LabelId>{NumLabels}
//!
//! PropertySize ::= u32 // size of property block

use bytes::{BufMut, Bytes, BytesMut};
use mojito_common::scalar::PropertyMapValue;
use mojito_common::store_types::PropertyValue;
use mojito_common::{LabelId, NodeId, PropertyKeyId, TokenId};

const NUM_LABELS_SIZE: usize = 2;
const PROPERTY_SIZE_SIZE: usize = 4;
const NODE_HEADER_SIZE: usize = NUM_LABELS_SIZE + PROPERTY_SIZE_SIZE;

pub struct NodeFormat;

impl NodeFormat {
    pub fn encode_node_key(node_id: NodeId) -> Bytes {
        let mut key = BytesMut::new();
        key.put_u8(crate::cf_property::NODE_KEY_PREFIX);
        key.put_u64_le(*node_id);
        key.freeze()
    }

    pub fn decode_node_key(buf: &[u8]) -> NodeId {
        assert_eq!(buf.len(), 9);
        NodeId::from_le_bytes(buf[1..9].try_into().unwrap())
    }
}

pub fn encode_node_value(labels: &[LabelId], property_map: PropertyMapValue) -> Bytes {
    let mut buf = BytesMut::new();
    // put header
    buf.put_u16_le(labels.len() as u16);
    buf.put_u32_le(property_map.size() as u32);
    // put labels
    for label_id in labels {
        buf.put_u16_le(*label_id);
    }
    // put properties
    buf.put(property_map.buffer);
    buf.freeze()
}

#[repr(C, packed(1))]
pub struct NodeFormatHeader {
    num_labels: u16,    // number of labels
    property_size: u32, // property size in bytes
}

impl NodeFormatHeader {
    pub fn set_property_size(&mut self, property_size: u32) {
        self.property_size = property_size;
    }

    pub fn set_num_labels(&mut self, num_labels: u16) {
        self.num_labels = num_labels;
    }
}

/// Node value writer
pub struct NodeFormatWriter<'a> {
    buf: &'a mut BytesMut,
    offset: usize,
}

impl<'a> NodeFormatWriter<'a> {
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

// pub struct NodeUpdater {
//     buf: BytesMut, // node value buffer
// }

// impl NodeUpdater {
//     pub fn new(buf: BytesMut) -> Self {
//         assert!(buf.len() >= NODE_HEADER_SIZE);
//         Self { buf }
//     }

//     // pub fn add_relationship(&mut self, rel: &[u8]) {
//     //     let header = self.header_mut();
//     //     let new_rel_size = header.relationship_size + (rel.len() as u32);
//     //     header.set_relationship_size(new_rel_size);
//     //     self.buf.extend_from_slice(rel);
//     // }

//     pub fn add_property(&mut self, key: PropertyKeyId, value: PropertyValue) {
//         let header = self.header_mut();
//         let property_size = header.property_size;
//         let property_writer = PropertyWriter::new(&mut self.buf, property_size as usize);
//         property_writer.add_property(key, value);
//         header.set_property_size(property_size + 1u32);
//     }

//     fn header_mut(&mut self) -> &mut NodeFormatHeader {
//         unsafe {
//             let ptr = self.buf.as_mut_ptr() as *mut NodeFormatHeader;
//             &mut *ptr
//         }
//     }

//     #[allow(unused)]
//     fn header(&self) -> &NodeFormatHeader {
//         unsafe {
//             let ptr = self.buf.as_ptr() as *const NodeFormatHeader;
//             &*ptr
//         }
//     }

//     pub fn finish(self) -> Bytes {
//         self.buf.freeze()
//     }
// }

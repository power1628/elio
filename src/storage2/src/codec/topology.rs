//! TopologyKey ::= TOPO_KEY_PREFIX NodeId Direction
//! TopologyValue ::= <Num> <EdgeBlock>
//!
//! Num ::= <Num>(4B)
//! EdgeBlock ::= <TopologyElement>{num}
//!
//! TopologyElement ::= <RelationshipTypeId>(8B) <OtherNodeId>(8B)

use bytemuck::{Pod, Zeroable};
use bytes::{BufMut, Bytes, BytesMut};
use mojito_common::{NodeId, RelationshipId, store_types::RelationshipDirection};

use crate::codec::TOPO_KEY_PREFIX;

const INCOMING_SUFFIX: u8 = 0x01;
const OUTGOING_SUFFIX: u8 = 0x02;

pub struct TopoFormat;

impl TopoFormat {
    pub fn encode_topo_key(node_id: NodeId, dir: RelationshipDirection) -> Bytes {
        let mut bytes = BytesMut::new();
        bytes.put_u8(TOPO_KEY_PREFIX);
        bytes.put_u64_le(node_id);
        match dir {
            RelationshipDirection::Incoming => bytes.put_u8(INCOMING_SUFFIX),
            RelationshipDirection::Outgoing => bytes.put_u8(OUTGOING_SUFFIX),
        }
        bytes.freeze()
    }

    pub fn decode_topo_key(buf: &[u8]) -> (NodeId, RelationshipDirection) {
        assert_eq!(buf.len(), 10);
        let node_id = u64::from_le_bytes(buf[1..9].try_into().unwrap());
        let dir = match buf[9] {
            INCOMING_SUFFIX => RelationshipDirection::Incoming,
            OUTGOING_SUFFIX => RelationshipDirection::Outgoing,
            _ => panic!("invalid direction suffix"),
        };
        (node_id, dir)
    }
}

#[repr(C, packed(1))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TopoHeader {
    pub num: u32,
}

pub struct TopoFormatWriter<'a> {
    buf: &'a mut BytesMut,
    header: &'a mut TopoHeader,
}

impl<'a> TopoFormatWriter<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        if buf.is_empty() {
            // initialize header
            let topo = TopoHeader::zeroed();
            buf.put_slice(bytemuck::bytes_of(&topo));
        }
        if buf.len() != size_of::<TopoHeader>() {
            panic!("topology value must be {} bytes", size_of::<TopoHeader>());
        }
        let header_buf = buf.as_mut_ptr() as *mut TopoHeader;
        let header = unsafe { &mut *header_buf };
        Self { buf, header }
    }

    pub fn add_relationship(&mut self, rel_id: RelationshipId, other_node_id: NodeId) {
        self.buf.put_u64_le(rel_id);
        self.buf.put_u64_le(other_node_id);
        self.header.num += 1;
    }

    pub fn delete_relationship(&mut self, rel_id: RelationshipId) {
        // TODO: implement delete relationship
    }
}

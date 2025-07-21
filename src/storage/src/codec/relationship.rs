//! Relationship
//!
//! ```text
//! +-----------+-------------+--------------------+-----------------+--------------------+
//! |reltpye(2B)|meta_data(1B)| relationship_id(5B)| other_node_id(8B| properties(varsize)|
//! +-----------+-------------+--------------------+-----------------+--------------------+
//!        +----+             +--+                                    
//!        v                     v                                    
//!        b7 b6 b5 b4 b3 b2 b1 b0                                    
//!                          |  |                                     
//!                          |  +-> direction: 1 outgoing, 0 incomming
//!                          +----> tombstone                         
//! ```

use bytes::{BufMut, BytesMut};
use mojito_common::{NodeId, RelationshipId, RelationshipTypeId};

use crate::types::RelationshipDirection;

const RELTIONSHIP_HEADER_SIZE: usize = 2 + 1 + 8;

// const DIRECTION_MASK: u8 = 0x01;
const REL_TYPE_MASK: u64 = 0xFFFFFFFFFF_FF_0000;
// const DIRECTION_MASK: u64 = 0x0000000000_10_0000;
const REL_ID_MASK: u64 = 0xFFFFFFFFFF_00_0000;

const DIRECTION_SHIFT: usize = 16;
const REL_ID_SHIFT: usize = 24;

pub struct RelationshipFormat;

pub struct RelationshipFormatWriter<'a> {
    buf: &'a mut BytesMut,
    offset: usize,
}

#[repr(C, packed(1))]
struct Header {
    // rel_type: u16,
    // meta_data: u8,
    // rel_id: [u8; 5],
    type_and_meta_and_id: u64,
    other_node_id: u64,
}

impl Header {
    pub fn set_reltype(&mut self, reltype: RelationshipTypeId) -> &mut Self {
        let cleared = self.type_and_meta_and_id & REL_TYPE_MASK;
        self.type_and_meta_and_id = cleared | (reltype as u64);
        self
    }

    pub fn set_direction(&mut self, dir: RelationshipDirection) -> &mut Self {
        self.type_and_meta_and_id = match dir {
            RelationshipDirection::Incoming => self.type_and_meta_and_id & !(1u64 << DIRECTION_SHIFT),
            RelationshipDirection::Outgoing => self.type_and_meta_and_id | (1u64 << DIRECTION_SHIFT),
        };
        self
    }

    pub fn set_relationship_id(&mut self, relid: RelationshipId) -> &mut Self {
        let cleared = self.type_and_meta_and_id & REL_ID_MASK;
        self.type_and_meta_and_id = cleared | (relid << REL_ID_SHIFT);
        self
    }

    pub fn set_other_node(&mut self, other_node_id: NodeId) -> &mut Self {
        self.other_node_id = other_node_id;
        self
    }
}

impl<'a> RelationshipFormatWriter<'a> {
    pub fn new(buf: &'a mut BytesMut) -> Self {
        let offset = buf.len();
        // init header
        buf.put_bytes(0, RELTIONSHIP_HEADER_SIZE);
        Self { buf, offset }
    }

    pub fn write(
        &mut self,
        reltype_id: RelationshipTypeId,
        other_node_id: NodeId,
        relationship_id: RelationshipId,
        dir: RelationshipDirection,
        properties: &[u8],
    ) -> usize {
        let header = unsafe {
            let ptr = self.buf[self.offset..].as_mut_ptr() as *mut Header;
            &mut *ptr
        };
        header
            .set_reltype(reltype_id)
            .set_direction(dir)
            .set_relationship_id(relationship_id)
            .set_other_node(other_node_id);

        self.buf.extend_from_slice(properties);
        self.buf.len() - self.offset
    }
}

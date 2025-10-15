use mojito_common::NodeId;

pub const NODE_KEY_PREFIX: u8 = 0x06;

pub fn node_key(node_id: NodeId) -> Vec<u8> {
    let mut key = vec![NODE_KEY_PREFIX];
    key.extend_from_slice(&node_id.to_le_bytes());
    key
}

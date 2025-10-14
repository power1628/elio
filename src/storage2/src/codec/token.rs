const LABEL_KEY_PREFIX: u8 = 0x01;
const RELTYPE_KEY_PREFIX: u8 = 0x02;
const PROPERTY_KEY_PREFIX: u8 = 0x03;

const NEXT_LABEL_ID_PREFIX: u8 = 0x04;
const NEXT_RELTYPE_ID_PREFIX: u8 = 0x05;
const NEXT_PROPERTY_KEY_ID_PREFIX: u8 = 0x06;

/// Storage
///   - key   := <LABEL_PREFIX> <label>
///   - value := <label_id>
///
/// NextId
///   - key   := <LABEL_PREFIX> <TOKEN_NEXT_ID>
///   - value := <next_id>
pub struct TokenFormat;

#[derive(Copy, Clone)]
pub enum TokenKind {
    Label,
    RelationshipType,
    PropertyKey,
}
impl TokenFormat {
    pub fn next_id_key(kind: &TokenKind) -> [u8; 1] {
        match kind {
            TokenKind::Label => [NEXT_LABEL_ID_PREFIX],
            TokenKind::RelationshipType => [NEXT_RELTYPE_ID_PREFIX],
            TokenKind::PropertyKey => [NEXT_PROPERTY_KEY_ID_PREFIX],
        }
    }

    #[inline]
    pub fn next_id_value(next_id: u16) -> [u8; 2] {
        next_id.to_le_bytes()
    }

    pub fn decode_next_id(buf: &[u8]) -> u16 {
        u16::from_le_bytes([buf[0], buf[1]])
    }

    pub fn data_key(kind: &TokenKind, token: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(1 + token.len());
        match kind {
            TokenKind::Label => {
                key.extend_from_slice(&[LABEL_KEY_PREFIX]);
            }
            TokenKind::RelationshipType => {
                key.extend_from_slice(&[RELTYPE_KEY_PREFIX]);
            }
            TokenKind::PropertyKey => {
                key.extend_from_slice(&[PROPERTY_KEY_PREFIX]);
            }
        };
        key.extend_from_slice(token.as_bytes());
        key
    }

    /// Used for prefix scan
    pub fn data_key_prefix(kind: &TokenKind) -> Vec<u8> {
        match kind {
            TokenKind::Label => vec![LABEL_KEY_PREFIX],
            TokenKind::RelationshipType => vec![RELTYPE_KEY_PREFIX],
            TokenKind::PropertyKey => vec![PROPERTY_KEY_PREFIX],
        }
    }

    pub fn decode_data_key(key: &[u8]) -> (TokenKind, String) {
        let kind = match key[0] {
            LABEL_KEY_PREFIX => TokenKind::Label,
            RELTYPE_KEY_PREFIX => TokenKind::RelationshipType,
            PROPERTY_KEY_PREFIX => TokenKind::PropertyKey,
            _ => panic!("invalid token kind"),
        };
        let token = String::from_utf8_lossy(&key[1..]).to_string();
        (kind, token)
    }

    pub fn decode_data_value(val: &[u8]) -> u16 {
        u16::from_le_bytes([val[0], val[1]])
    }
}

impl TokenFormat {
    #[inline]
    pub fn read_token(buf: &[u8]) -> u16 {
        TokenFormat::decode(buf)
    }

    #[inline]
    pub fn encode_data_value(id: u16) -> [u8; 2] {
        id.to_le_bytes()
    }

    #[inline]
    pub fn decode(buffer: &[u8]) -> u16 {
        u16::from_le_bytes([buffer[0], buffer[1]])
    }
}

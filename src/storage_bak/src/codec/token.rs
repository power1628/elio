pub const TOKEN_LABEL_PREFIX: u8 = 0x01;
pub const TOKEN_RELTYPE_PREFIX: u8 = 0x02;
pub const TOKEN_PROPERTY_KEY_PREFIX: u8 = 0x03;

pub const TOKEN_NEXT_ID_PREFIX: u8 = 0x00;
pub const TOKEN_DATA_PREFIX: u8 = 0x01;

pub const TOKEN_KIND_BYTES: usize = 2; // all the tokens have u16 type

/// Storage
///   - key   := <LABEL_PREFIX> <label>
///   - value := <label_id>
///
/// NextId
///   - key   := <LABEL_PREFIX> <TOKEN_NEXT_ID>
///   - value := <next_id>
pub struct TokenFormat;

pub enum TokenKind {
    Label,
    RelationshipType,
    PropertyKey,
}
impl TokenFormat {
    pub fn next_id_key(kind: &TokenKind) -> [u8; 2] {
        match kind {
            TokenKind::Label => [TOKEN_LABEL_PREFIX, TOKEN_NEXT_ID_PREFIX],
            TokenKind::RelationshipType => [TOKEN_RELTYPE_PREFIX, TOKEN_NEXT_ID_PREFIX],
            TokenKind::PropertyKey => [TOKEN_PROPERTY_KEY_PREFIX, TOKEN_NEXT_ID_PREFIX],
        }
    }

    pub fn data_key(kind: &TokenKind, token: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(TOKEN_KIND_BYTES + token.len());
        match kind {
            TokenKind::Label => {
                key.extend_from_slice(&[TOKEN_LABEL_PREFIX, TOKEN_DATA_PREFIX]);
            }
            TokenKind::RelationshipType => {
                key.extend_from_slice(&[TOKEN_RELTYPE_PREFIX, TOKEN_DATA_PREFIX]);
            }
            TokenKind::PropertyKey => {
                key.extend_from_slice(&[TOKEN_PROPERTY_KEY_PREFIX, TOKEN_DATA_PREFIX]);
            }
        };
        key.extend_from_slice(token.as_bytes());
        key
    }
}

impl TokenFormat {
    #[inline]
    pub fn read_next_id(buf: &[u8]) -> u16 {
        u16::from_le_bytes([buf[0], buf[1]])
    }

    #[inline]
    pub fn read_token(buf: &[u8]) -> u16 {
        TokenFormat::decode(buf)
    }

    #[inline]
    pub fn encode_next_id(next_id: u16) -> [u8; TOKEN_KIND_BYTES] {
        next_id.to_le_bytes()
    }

    #[inline]
    pub fn encode(token: u16) -> [u8; TOKEN_KIND_BYTES] {
        token.to_le_bytes()
    }

    #[inline]
    pub fn decode(buffer: &[u8]) -> u16 {
        u16::from_le_bytes([buffer[0], buffer[1]])
    }
}

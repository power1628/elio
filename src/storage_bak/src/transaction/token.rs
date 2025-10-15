//! There're 3 kinds of tokens:
//!  - label token
//!  - reltype token
//!  - propertykey token
//!
//! This module manages register/get tokens.
use mojito_common::{Label, LabelId, PropertyKey, PropertyKeyId, RelationshipType, RelationshipTypeId};
use redb::ReadableTable;

use crate::{
    codec::{TokenFormat, TokenKind},
    error::GraphStoreError,
    transaction::{GraphRead, GraphWrite},
};

impl GraphWrite {
    pub fn register_label(&mut self, label: &Label) -> Result<LabelId, GraphStoreError> {
        self.register_token(&TokenKind::Label, label)
    }

    pub fn register_property_key(&mut self, key: &PropertyKey) -> Result<PropertyKeyId, GraphStoreError> {
        self.register_token(&TokenKind::PropertyKey, key)
    }

    pub fn register_reltype(&mut self, rel_type: &RelationshipType) -> Result<RelationshipTypeId, GraphStoreError> {
        self.register_token(&TokenKind::RelationshipType, rel_type)
    }
}

impl GraphWrite {
    // if token exists, return
    // if not exists, create one
    fn register_token(&mut self, kind: &TokenKind, token: &str) -> Result<u16, GraphStoreError> {
        let token_id = self.get_token(kind, token)?;
        if let Some(id) = token_id {
            return Ok(id);
        }

        let table = self.table_mut();

        let key = TokenFormat::next_id_key(kind);
        let next_id = table
            .get(key.as_slice())
            .map_err(Box::new)?
            .map(|buf| TokenFormat::read_next_id(buf.value()))
            .unwrap_or_default();
        let new_next_id = next_id + 1;
        table
            .insert(key.as_slice(), TokenFormat::encode_next_id(new_next_id).as_slice())
            .map_err(Box::new)?;
        {
            let data_key = TokenFormat::data_key(kind, token);
            table
                .insert(data_key.as_slice(), TokenFormat::encode(new_next_id).as_slice())
                .map_err(Box::new)?;
        }
        Ok(next_id)
    }

    fn get_token(&self, kind: &TokenKind, token: &str) -> Result<Option<u16>, GraphStoreError> {
        let table = self.table();
        let key = TokenFormat::data_key(kind, token);
        let value = table
            .get(key.as_slice())
            .map_err(Box::new)?
            .map(|v| TokenFormat::read_token(v.value()));
        Ok(value)
    }
}

impl GraphRead {
    pub fn get_token(&self, kind: &TokenKind, token: &str) -> Result<Option<u16>, GraphStoreError> {
        let table = self.table();
        let key = TokenFormat::data_key(kind, token);
        let value = table
            .get(key.as_slice())
            .map_err(Box::new)?
            .map(|v| TokenFormat::read_token(v.value()));
        Ok(value)
    }
}

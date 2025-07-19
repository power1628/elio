//! There're 3 kinds of tokens:
//!  - label token
//!  - reltype token
//!  - propertykey token
//!
//! This module manages register/get tokens.
use crate::{
    error::GraphStoreError,
    transaction::GraphWrite,
    types::{Label, LabelId, PropertyKey, PropertyKeyId, RelationshipType, RelationshipTypeId},
};

/// Label API
/// Storage
///   - key   := <LABEL_PREFIX> <label>
///   - value := <label_id>
impl GraphWrite {
    // Register label, if label not exist, create one, if already exists, return the label id.
    pub fn register_label(&mut self, label: &Label) -> Result<LabelId, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn get_label_id(&self, label: &Label) -> Result<Option<LabelId>, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    // TODO(pgao): batch api
}

/// RelType API
/// Storage:
///   - key   := <RELTYPE_PREFIX> <reltype>
///   - value := <reltype_id>
impl GraphWrite {
    pub fn register_reltype(&mut self, reltype: &RelationshipType) -> Result<RelationshipTypeId, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn get_reltype_id(&self, reltype: &RelationshipType) -> Result<Option<RelationshipTypeId>, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }
}

/// PropertyKey API
/// Storage:
///   - key   := <PROPERTYKEY_PREFIX> <propertykey>
///   - value := <propertykey_id>
impl GraphWrite {
    pub fn register_propertykey(&mut self, propertykey: &PropertyKey) -> Result<PropertyKeyId, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }

    pub fn get_property_key_id(&self, propertykey: &PropertyKey) -> Result<Option<PropertyKeyId>, GraphStoreError> {
        // TODO(pgao): impl
        todo!()
    }
}

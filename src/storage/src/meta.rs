use std::{
    collections::HashMap,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU16, Ordering},
    },
};

use mojito_common::{LabelId, PropertyKeyId, RelationshipTypeId};

use crate::{
    CF_META,
    codec::{TokenCodec, TokenKind},
    error::GraphStoreError,
};

pub struct MetaStore {
    db: Arc<rocksdb::TransactionDB>,
    // in memory cache
    next_label_id: AtomicU16,
    next_reltype_id: AtomicU16,
    next_property_key_id: AtomicU16,
    labels: RwLock<HashMap<String, LabelId>>,
    reltypes: RwLock<HashMap<String, RelationshipTypeId>>,
    property_keys: RwLock<HashMap<String, PropertyKeyId>>,
}

impl MetaStore {
    pub fn new(db: Arc<rocksdb::TransactionDB>) -> Result<Self, GraphStoreError> {
        let mut store = Self {
            db,
            next_label_id: AtomicU16::new(0),
            next_reltype_id: AtomicU16::new(0),
            next_property_key_id: AtomicU16::new(0),
            labels: RwLock::new(HashMap::new()),
            reltypes: RwLock::new(HashMap::new()),
            property_keys: RwLock::new(HashMap::new()),
        };
        store.load_from_db()?;
        Ok(store)
    }

    fn load_from_db(&mut self) -> Result<(), GraphStoreError> {
        // load state from db to cache
        // token dict
        self.load_token_dict(TokenKind::Label)?;
        self.load_token_dict(TokenKind::RelationshipType)?;
        self.load_token_dict(TokenKind::PropertyKey)?;
        Ok(())
    }

    pub fn get_label_id(&self, label: &str) -> Option<LabelId> {
        let labels = self.labels.read().unwrap();
        labels.get(label).cloned()
    }

    pub fn get_reltype_id(&self, reltype: &str) -> Option<RelationshipTypeId> {
        let reltypes = self.reltypes.read().unwrap();
        reltypes.get(reltype).cloned()
    }

    pub fn get_property_key_id(&self, property_key: &str) -> Option<PropertyKeyId> {
        let property_keys = self.property_keys.read().unwrap();
        property_keys.get(property_key).cloned()
    }

    pub fn get_or_create_label_id(&self, label: &str) -> Result<LabelId, GraphStoreError> {
        self.get_or_create_token(label, TokenKind::Label)
    }

    pub fn get_or_create_reltype_id(&self, reltype: &str) -> Result<RelationshipTypeId, GraphStoreError> {
        self.get_or_create_token(reltype, TokenKind::RelationshipType)
    }

    pub fn get_or_create_property_key_id(&self, property_key: &str) -> Result<PropertyKeyId, GraphStoreError> {
        self.get_or_create_token(property_key, TokenKind::PropertyKey)
    }
}

impl MetaStore {
    fn get_or_create_token(&self, token: &str, token_kind: TokenKind) -> Result<u16, GraphStoreError> {
        let mut tokens = match token_kind {
            TokenKind::Label => self.labels.write().unwrap(),
            TokenKind::RelationshipType => self.reltypes.write().unwrap(),
            TokenKind::PropertyKey => self.property_keys.write().unwrap(),
        };
        if let Some(token_id) = tokens.get(token) {
            return Ok(*token_id);
        }

        // create token if not exists
        let token_id = match token_kind {
            TokenKind::Label => self.next_label_id.fetch_add(1, Ordering::Relaxed),
            TokenKind::RelationshipType => self.next_reltype_id.fetch_add(1, Ordering::Relaxed),
            TokenKind::PropertyKey => self.next_property_key_id.fetch_add(1, Ordering::Relaxed),
        };
        tokens.insert(token.to_string(), token_id);
        // write to db
        let cf = self.db.cf_handle(CF_META).unwrap();
        {
            // insert token -> id to db
            let key = TokenCodec::data_key(&token_kind, token);
            let value = TokenCodec::encode_data_value(token_id);
            self.db.put_cf(&cf, key, value)?;
        }
        Ok(token_id)
    }

    fn load_token_dict(&mut self, token_kind: TokenKind) -> Result<(), GraphStoreError> {
        let tokens = self.db_get_all_token(token_kind)?;
        let dict = match token_kind {
            TokenKind::Label => &mut self.labels,
            TokenKind::RelationshipType => &mut self.reltypes,
            TokenKind::PropertyKey => &mut self.property_keys,
        };
        // update next id
        let next_id = match tokens.iter().map(|(_, id)| id).max() {
            Some(max_id) => max_id + 1,
            None => 0,
        };
        match token_kind {
            TokenKind::Label => self.next_label_id.store(next_id, Ordering::Relaxed),
            TokenKind::RelationshipType => self.next_reltype_id.store(next_id, Ordering::Relaxed),
            TokenKind::PropertyKey => self.next_property_key_id.store(next_id, Ordering::Relaxed),
        }

        let mut dict = dict.write().unwrap();
        dict.clear();
        dict.extend(tokens);
        Ok(())
    }

    fn db_get_all_token(&self, token_kind: TokenKind) -> Result<Vec<(String, u16)>, GraphStoreError> {
        let cf = self.db.cf_handle(CF_META).unwrap();
        let prefix = TokenCodec::data_key_prefix(&token_kind);
        let iter = self.db.prefix_iterator_cf(&cf, prefix);

        let mut tokens = Vec::new();
        for res in iter {
            let (key, value) = res?;
            let (_, token) = TokenCodec::decode_data_key(&key);
            let token_id = TokenCodec::decode_data_value(&value);
            tokens.push((token, token_id));
        }
        Ok(tokens)
    }
}

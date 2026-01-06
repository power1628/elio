use std::collections::HashMap;
use std::sync::Arc;

use mojito_common::LabelId;
use parking_lot::RwLock;
use rocksdb;
use rocksdb::{ColumnFamilyDescriptor, Options};

use crate::constraint::ConstraintStore;
use crate::dict::IdStore;
use crate::error::GraphStoreError;
use crate::token::TokenStore;
use crate::transaction::TransactionImpl;
use crate::{cf_constraint, cf_meta, cf_property, cf_topology};

pub struct GraphStore {
    db: Arc<rocksdb::TransactionDB>,
    dict: Arc<IdStore>,
    token: Arc<TokenStore>,
    constraint: Arc<ConstraintStore>,
    /// Label-level locks for constraint operations
    /// Read lock: normal writes (CREATE node)
    /// Write lock: CREATE CONSTRAINT (exclusive)
    label_locks: RwLock<HashMap<LabelId, Arc<RwLock<()>>>>,
}

/// Guard for label read lock (allows concurrent writes)
/// Uses Arc to ensure the lock lives long enough and is Send-safe
pub struct LabelReadGuard {
    _lock: Arc<RwLock<()>>,
    _guard: parking_lot::RwLockReadGuard<'static, ()>,
}

// Safety: The guard holds an Arc to the lock, ensuring it lives long enough
unsafe impl Send for LabelReadGuard {}
unsafe impl Sync for LabelReadGuard {}

/// Guard for label write lock (exclusive for CREATE CONSTRAINT)
pub struct LabelWriteGuard {
    _lock: Arc<RwLock<()>>,
    _guard: parking_lot::RwLockWriteGuard<'static, ()>,
}

// Safety: The guard holds an Arc to the lock, ensuring it lives long enough
unsafe impl Send for LabelWriteGuard {}
unsafe impl Sync for LabelWriteGuard {}

pub enum TransactionMode {
    ReadOnly,
    ReadWrite,
}

impl GraphStore {
    pub fn open(path: &str) -> Result<Self, GraphStoreError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new(cf_meta::CF_NAME, Options::default()),
            ColumnFamilyDescriptor::new(cf_topology::CF_NAME, Options::default()),
            ColumnFamilyDescriptor::new(cf_property::CF_NAME, Options::default()),
            ColumnFamilyDescriptor::new(cf_constraint::CF_NAME, Options::default()),
        ];
        let tx_db_opts = rocksdb::TransactionDBOptions::default();

        // create db and create cf if not exist
        let db = match rocksdb::TransactionDB::open_cf_descriptors(&opts, &tx_db_opts, path, cf_descriptors) {
            Ok(db) => db,
            Err(_) => {
                // if db not exists, create one
                let db = rocksdb::TransactionDB::open_default(path)?;

                // create cf
                let cf_opts = Options::default();
                db.create_cf(cf_meta::CF_NAME, &cf_opts)?;
                db.create_cf(cf_topology::CF_NAME, &cf_opts)?;
                db.create_cf(cf_property::CF_NAME, &cf_opts)?;
                db.create_cf(cf_constraint::CF_NAME, &cf_opts)?;

                db
            }
        };

        let db = Arc::new(db);
        let dict = Arc::new(IdStore::new(db.clone())?);
        let token = Arc::new(TokenStore::new(db.clone())?);
        let constraint = Arc::new(ConstraintStore::new(db.clone()));

        Ok(Self {
            db,
            dict,
            token,
            constraint,
            label_locks: RwLock::new(HashMap::new()),
        })
    }

    pub fn token_store(&self) -> &Arc<TokenStore> {
        &self.token
    }

    pub fn constraint_store(&self) -> &Arc<ConstraintStore> {
        &self.constraint
    }

    pub fn db(&self) -> &Arc<rocksdb::TransactionDB> {
        &self.db
    }

    pub fn transaction(&self) -> Arc<TransactionImpl> {
        Arc::new(TransactionImpl::new(
            self.db.clone(),
            self.dict.clone(),
            self.token.clone(),
        ))
    }

    /// Get or create a lock for the given label
    fn get_label_lock(&self, label_id: LabelId) -> Arc<RwLock<()>> {
        // Try read first
        {
            let locks = self.label_locks.read();
            if let Some(lock) = locks.get(&label_id) {
                return lock.clone();
            }
        }
        // Create if not exists
        let mut locks = self.label_locks.write();
        locks
            .entry(label_id)
            .or_insert_with(|| Arc::new(RwLock::new(())))
            .clone()
    }

    /// Acquire read lock for a label (allows concurrent writes)
    /// Used by: CREATE (n:Label ...)
    pub fn acquire_label_read(&self, label_id: LabelId) -> LabelReadGuard {
        let lock = self.get_label_lock(label_id);
        // Safety: We keep the Arc in the guard, so the lock lives long enough
        let lock_ptr = Arc::as_ptr(&lock);
        let guard = unsafe { (*lock_ptr).read() };
        // Transmute to 'static lifetime - safe because we hold the Arc
        let guard: parking_lot::RwLockReadGuard<'static, ()> = unsafe { std::mem::transmute(guard) };
        LabelReadGuard {
            _lock: lock,
            _guard: guard,
        }
    }

    /// Acquire write lock for a label (exclusive)
    /// Used by: CREATE CONSTRAINT FOR (n:Label) ...
    pub fn acquire_label_write(&self, label_id: LabelId) -> LabelWriteGuard {
        let lock = self.get_label_lock(label_id);
        // Safety: We keep the Arc in the guard, so the lock lives long enough
        let lock_ptr = Arc::as_ptr(&lock);
        let guard = unsafe { (*lock_ptr).write() };
        // Transmute to 'static lifetime - safe because we hold the Arc
        let guard: parking_lot::RwLockWriteGuard<'static, ()> = unsafe { std::mem::transmute(guard) };
        LabelWriteGuard {
            _lock: lock,
            _guard: guard,
        }
    }

    /// Acquire read locks for multiple labels (sorted to avoid deadlock)
    pub fn acquire_labels_read(&self, label_ids: &[LabelId]) -> Vec<LabelReadGuard> {
        let mut sorted_ids = label_ids.to_vec();
        sorted_ids.sort();
        sorted_ids.dedup();
        sorted_ids.into_iter().map(|id| self.acquire_label_read(id)).collect()
    }
}

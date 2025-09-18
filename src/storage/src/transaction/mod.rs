use std::{mem::ManuallyDrop, pin::Pin, sync::Arc};

use redb::{ReadOnlyTable, Table};

mod id;
mod node;
mod relationship;
mod token;
use crate::{error::GraphStoreError, graph_store::KVSTORE_TABLE_DEFINITION};

pub struct GraphWrite {
    // pin the transaction to ensure stable memory address
    kv_tx: Pin<Box<redb::WriteTransaction>>,
    // SAFETY: table will be drop before kv_tx
    table: ManuallyDrop<Option<Table<'static, &'static [u8], &'static [u8]>>>,
}

impl GraphWrite {
    pub fn new(db: &Arc<redb::Database>) -> Result<Self, GraphStoreError> {
        let kv_tx = db.begin_write().map_err(Box::new)?;
        let mut container = Self {
            kv_tx: Box::pin(kv_tx),
            table: ManuallyDrop::new(None),
        };
        let tx_ref: &'static redb::WriteTransaction =
            unsafe { std::mem::transmute(container.kv_tx.as_ref().get_ref()) };
        let table = tx_ref.open_table(KVSTORE_TABLE_DEFINITION).map_err(Box::new)?;
        *container.table = Some(table);
        Ok(container)
    }

    pub fn table(&self) -> &Table<&'static [u8], &'static [u8]> {
        // safety: with new, table must be initialized
        self.table.as_ref().unwrap()
    }

    pub fn table_mut(&mut self) -> &mut Table<'static, &'static [u8], &'static [u8]> {
        // safety: with new, table must be initialized
        self.table.as_mut().unwrap()
    }

    pub fn commit(self) -> Result<(), GraphStoreError> {
        let mut this = ManuallyDrop::new(self);

        // manually drop table first
        unsafe {
            ManuallyDrop::drop(&mut this.table);
        }
        let tx = unsafe { Pin::into_inner_unchecked(std::ptr::read(&this.kv_tx)) };
        tx.commit().map_err(Box::new)?;
        Ok(())
    }
}

impl Drop for GraphWrite {
    fn drop(&mut self) {
        // manually drop table first
        unsafe {
            ManuallyDrop::drop(&mut self.table);
        }
        // kv_tx will be dropped after
    }
}

pub struct GraphRead {
    kv_tx: Pin<Box<redb::ReadTransaction>>,
    // SAFETY: table will be drop before kv_tx
    table: ManuallyDrop<Option<ReadOnlyTable<&'static [u8], &'static [u8]>>>,
}

impl GraphRead {
    pub fn new(db: &Arc<redb::Database>) -> Result<Self, GraphStoreError> {
        let kv_tx = db.begin_read().map_err(Box::new)?;
        let mut container = Self {
            kv_tx: Box::pin(kv_tx),
            table: ManuallyDrop::new(None),
        };
        let tx_ref: &'static redb::ReadTransaction = unsafe { std::mem::transmute(container.kv_tx.as_ref().get_ref()) };
        let table = tx_ref.open_table(KVSTORE_TABLE_DEFINITION).map_err(Box::new)?;
        *container.table = Some(table);
        Ok(container)
    }
}

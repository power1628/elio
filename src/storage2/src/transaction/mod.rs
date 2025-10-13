use std::{pin::Pin, sync::Arc};

use rocksdb;

mod token;
use token::*;

pub struct Transaction {
    // SAFETY: inner will be dropped before db
    inner: Pin<Box<rocksdb::Transaction<'static, rocksdb::DB>>>,
    db: Arc<rocksdb::TransactionDB>,
}

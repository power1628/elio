pub struct GraphReadTransaction {
    kv_tx: redb::ReadTransaction,
}

impl GraphReadTransaction {
    pub fn new(kv_tx: redb::ReadTransaction) -> Self {
        Self { kv_tx }
    }
}

pub struct GraphWriteTransaction {
    kv_tx: redb::WriteTransaction,
}

impl GraphWriteTransaction {
    pub fn new(kv_tx: redb::WriteTransaction) -> Self {
        Self { kv_tx }
    }
}

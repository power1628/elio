pub enum Error {
    RedbError(Box<redb::Error>),
    RedbTransactionError(Box<redb::TransactionError>),
}

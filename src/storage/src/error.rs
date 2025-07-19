use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphStoreError {
    #[error("redb error: {0}")]
    RedbError(
        #[from]
        #[source]
        Box<redb::Error>,
    ),
    #[error("redb transaction error: {0}")]
    RedbTransactionError(
        #[from]
        #[source]
        Box<redb::TransactionError>,
    ),
    #[error("redb table error: {0}")]
    RedbTableError(
        #[from]
        #[source]
        Box<redb::TableError>,
    ),
    #[error("redb storage error: {0}")]
    RedbStorageError(
        #[from]
        #[source]
        Box<redb::StorageError>,
    ),
    #[error("ill formatted data: {message}")]
    IllFormattedData { message: String, data: Vec<u8> },
}

impl GraphStoreError {
    pub fn ill_formatted_data(message: impl ToString, data: Vec<u8>) -> Self {
        Self::IllFormattedData {
            message: message.to_string(),
            data,
        }
    }
}

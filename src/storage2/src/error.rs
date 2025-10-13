use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphStoreError {
    #[error("rocksdb error: {0}")]
    Rocksdb(#[from] rocksdb::Error),
}

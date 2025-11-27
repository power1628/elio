use mojito_storage::error::GraphStoreError;

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("Store error: {0}")]
    StoreError(#[from] GraphStoreError),
}

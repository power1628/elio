use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("open db failed")]
    OpenDbFailed {
        #[from]
        source: mojito_storage::error::GraphStoreError,
    },
}

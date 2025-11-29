use mojito_expr::error::EvalError;
use mojito_storage::error::GraphStoreError;

use crate::builder::BuildError;

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("executor build error: {0}")]
    BuildError(#[from] BuildError),
    #[error("Store error: {0}")]
    StoreError(#[from] GraphStoreError),
    #[error("Eval error: {0}")]
    EvalError(#[from] EvalError),
}

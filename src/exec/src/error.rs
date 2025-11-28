use mojito_expr::error::EvalError;
use mojito_storage::error::GraphStoreError;

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("Store error: {0}")]
    StoreError(#[from] GraphStoreError),
    #[error("Eval error: {0}")]
    EvalError(#[from] EvalError),
}

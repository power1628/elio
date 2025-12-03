use std::backtrace::Backtrace;

use mojito_expr::error::EvalError;
use mojito_storage::error::GraphStoreError;

use crate::builder::BuildError;

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("executor build error: {0}")]
    BuildError(#[from] BuildError, #[backtrace] Backtrace),
    #[error("Store error: {0}")]
    StoreError(#[from] GraphStoreError, #[backtrace] Backtrace),
    #[error("Eval error: {0}")]
    EvalError(#[from] EvalError, #[backtrace] Backtrace),
}

use std::backtrace::Backtrace;

use mojito_common::array::PhysicalType;
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
    #[error("type mismatch in {}, expected {:?}, actual {:?}", 0, 1, 2)]
    TypeMismatch {
        context: String,
        expected: String,
        actual: PhysicalType,
        trace: Backtrace,
    },
    #[error("constraint violation: {constraint} - {reason}")]
    ConstraintViolation {
        constraint: String,
        reason: String,
        #[backtrace]
        trace: Backtrace,
    },
}

impl ExecError {
    pub fn type_mismatch<T1: ToString, T2: ToString>(context: T1, expected: T2, actual: PhysicalType) -> Self {
        Self::TypeMismatch {
            context: context.to_string(),
            expected: expected.to_string(),
            actual,
            trace: Backtrace::capture(),
        }
    }
}

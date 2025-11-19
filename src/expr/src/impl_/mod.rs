use mojito_common::{
    array::{ArrayImpl, chunk::DataChunk},
    data_type::DataType,
};

use crate::error::EvalError;

pub mod func_call;
pub mod func_executor;

pub struct ExecCtx;

pub struct EvalCtx {
    // ExecCtx
    pub ectx: ExecCtx,
}

// an evaluatable expression
pub trait Expression: Send + Sync {
    fn typ(&self) -> DataType;
    fn eval_batch(&self, chunk: &DataChunk, ctx: &EvalCtx) -> Result<ArrayImpl, EvalError>;
}

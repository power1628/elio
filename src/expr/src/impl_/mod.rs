
use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;

use crate::error::EvalError;

pub mod constant;
pub mod func_call;
pub mod func_executor;
pub mod label;
pub mod property_access;
pub mod variable_ref;

pub struct EvalCtx {}

impl Default for EvalCtx {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalCtx {
    pub fn new() -> Self {
        Self {}
    }
}

// an evaluatable expression
pub trait Expression: Send + Sync {
    fn typ(&self) -> DataType;
    fn eval_batch(&self, chunk: &DataChunk, ctx: &EvalCtx) -> Result<ArrayImpl, EvalError>;
}

pub type BoxedExpression = Box<dyn Expression>;

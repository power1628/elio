use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;
use mojito_common::{IrToken, TokenId, TokenKind};

use crate::error::EvalError;

pub mod constant;
pub mod create_map;
pub mod func_call;
pub mod func_executor;
pub mod label;
pub mod property_access;
pub mod variable_ref;

// pub struct EvalCtx {}

// impl Default for EvalCtx {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl EvalCtx {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

pub trait EvalCtx {
    fn get_or_create_token(&self, token: &str, kind: TokenKind) -> Result<TokenId, EvalError>;
}

// an evaluatable expression
pub trait Expression: Send + Sync + 'static {
    fn typ(&self) -> DataType;
    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayImpl, EvalError>;
    fn boxed(self) -> BoxedExpression
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type BoxedExpression = Box<dyn Expression>;

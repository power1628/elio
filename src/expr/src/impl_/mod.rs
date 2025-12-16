use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayRef, NodeArray, VirtualNodeArray};
use mojito_common::data_type::DataType;
use mojito_common::{TokenId, TokenKind};

use crate::error::EvalError;

pub mod constant;
pub mod create_struct;
pub mod func_call;
// pub mod func_executor;
pub mod field_access;
pub mod label;
pub mod project_path;
pub mod variable_ref;

pub trait EvalCtx {
    // schema
    fn get_or_create_token(&self, token: &str, kind: TokenKind) -> Result<TokenId, EvalError>;
    // graph storage
    // access the storage engine and materialize node
    fn materialize_node(&self, chunk: &VirtualNodeArray) -> Result<NodeArray, EvalError>;
}

// an evaluatable expression
pub trait Expression: Send + Sync + 'static + std::fmt::Debug {
    fn typ(&self) -> &DataType;
    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError>;
    fn boxed(self) -> BoxedExpression
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub type BoxedExpression = Box<dyn Expression>;

use mojito_common::array::{ArrayImpl, chunk::DataChunk};

use crate::{
    error::EvalError,
    impl_::{EvalCtx, Expression},
};

pub struct FuncCallExpr {
    pub inputs: Vec<Box<dyn Expression>>,
    pub func: Box<dyn Fn(&DataChunk, &EvalCtx) -> Result<ArrayImpl, EvalError>>,
}

// Function 注册的时候，funcimpl 是这样的，

// 不同类型的 Function 名字不一样，通过这种方式来实现多态

pub type FunctionImpl = fn(&DataChunk, &EvalCtx) -> Result<ArrayImpl, EvalError>;

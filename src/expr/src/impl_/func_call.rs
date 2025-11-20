use mojito_common::{
    array::{ArrayImpl, chunk::DataChunk},
    data_type::DataType,
};

use crate::{
    error::EvalError,
    impl_::{EvalCtx, Expression},
};

// used to invoke the function call
pub type FunctionImpl = fn(&DataChunk, &EvalCtx) -> Result<ArrayImpl, EvalError>;

pub struct FuncCallExpr {
    pub inputs: Vec<Box<dyn Expression>>,
    pub func: FunctionImpl,
    typ: DataType,
}

impl Expression for FuncCallExpr {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
        let args = self
            .inputs
            .iter()
            .map(|e| e.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        let chunk = DataChunk::new(args);
        (self.func)(&chunk, ctx)
    }
}

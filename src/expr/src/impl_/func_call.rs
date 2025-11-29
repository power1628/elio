use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;

use crate::error::EvalError;
use crate::impl_::{EvalCtx, Expression};

// used to invoke the function call
pub type FunctionImpl = fn(&DataChunk, &dyn EvalCtx) -> Result<ArrayImpl, EvalError>;

#[derive(Debug)]
pub struct FuncCallExpr {
    pub inputs: Vec<Box<dyn Expression>>,
    pub func: FunctionImpl,
    typ: DataType,
}

impl Expression for FuncCallExpr {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayImpl, EvalError> {
        let args = self
            .inputs
            .iter()
            .map(|e| e.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        let len = args[0].len();
        let chunk = DataChunk::new(args, len);
        (self.func)(&chunk, ctx)
    }
}

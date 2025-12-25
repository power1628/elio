use bitvec::vec::BitVec;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayImpl, ArrayRef};
use mojito_common::data_type::DataType;

use crate::error::EvalError;
use crate::impl_::{EvalCtx, Expression};

// used to invoke the function call
pub type FunctionImpl = fn(&[ArrayRef], vis: &BitVec, len: usize) -> Result<ArrayImpl, EvalError>;

#[derive(Debug)]
pub struct FuncCallExpr {
    pub inputs: Vec<Box<dyn Expression>>,
    pub func: FunctionImpl,
    pub typ: DataType,
}

impl Expression for FuncCallExpr {
    fn typ(&self) -> &DataType {
        &self.typ
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError> {
        let args = self
            .inputs
            .iter()
            .map(|e| e.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        let vis = chunk.visibility();
        let len = chunk.len();
        let res = (self.func)(&args, vis, len)?;
        Ok(res.into())
    }
}

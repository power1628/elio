use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;

use crate::error::EvalError;
use crate::impl_::{EvalCtx, Expression};

#[derive(Debug)]
pub struct VariableRefExpr {
    pub idx: usize,
    typ: DataType,
}

impl VariableRefExpr {
    pub fn new(idx: usize, typ: DataType) -> Self {
        Self { idx, typ }
    }
}

impl Expression for VariableRefExpr {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }

    fn eval_batch(&self, chunk: &DataChunk, _ctx: &dyn EvalCtx) -> Result<ArrayImpl, EvalError> {
        Ok(chunk.column(self.idx).clone())
    }
}

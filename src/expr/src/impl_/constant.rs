use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;
use mojito_common::scalar::Datum;

use crate::error::EvalError;
use crate::impl_::{EvalCtx, Expression};

pub struct ConstantExpr {
    value: Datum,
    typ: DataType,
}

impl Expression for ConstantExpr {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }

    fn eval_batch(&self, chunk: &DataChunk, _ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
        let mut builder = self.typ.array_builder(chunk.len());
        builder.append_n(self.value.clone().as_ref().map(|x| x.as_scalar_ref()), chunk.len());
        Ok(builder.finish())
    }
}

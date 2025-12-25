use std::sync::Arc;

use mojito_common::array::ArrayRef;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;
use mojito_common::scalar::ScalarValue;

use crate::error::EvalError;
use crate::impl_::{EvalCtx, Expression};

#[derive(Debug)]
pub struct ConstantExpr {
    pub value: Option<ScalarValue>,
    pub typ: DataType,
}

impl Expression for ConstantExpr {
    fn typ(&self) -> &DataType {
        &self.typ
    }

    fn eval_batch(&self, chunk: &DataChunk, _ctx: &dyn EvalCtx) -> Result<ArrayRef, EvalError> {
        let mut builder = self.typ.physical_type().array_builder(chunk.len());
        // .into_any()
        // .map_err(|_| EvalError::type_error(format!("consant only allow basic types, got {}", self.typ)))?;

        builder.push_n(self.value.as_ref().map(|x| x.as_scalar_ref()), chunk.len());

        Ok(Arc::new(builder.finish().into()))
    }
}

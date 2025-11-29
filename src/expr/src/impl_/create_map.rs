use mojito_common::array::ArrayImpl;
use mojito_common::array::chunk::DataChunk;
use mojito_common::data_type::DataType;
use mojito_common::{IrToken, TokenKind};

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

pub struct CreateMapExpr {
    pub properties: Vec<(IrToken, BoxedExpression)>,
}

impl Expression for CreateMapExpr {
    fn typ(&self) -> DataType {
        DataType::PropertyMap
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayImpl, EvalError> {
        // create token first
        // This should be optimized
        let mut token_ids = Vec::with_capacity(self.properties.len());
        for (token, _) in &self.properties {
            token_ids.push(ctx.get_or_create_token(token, TokenKind::PropertyKey)?);
        }

        // eval batch
        let mut builder = self.typ().array_builder(chunk.len());
        let children = self
            .properties
            .iter()
            .map(|(_, e)| e.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        todo!()
    }
}

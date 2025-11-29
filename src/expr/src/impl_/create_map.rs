use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayBuilder, ArrayImpl};
use mojito_common::data_type::DataType;
use mojito_common::mapb::PropertyMapMut;
use mojito_common::{IrToken, TokenKind};

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

#[derive(Debug)]
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
            let token_id = match token {
                IrToken::Resolved(token_id) => *token_id,
                IrToken::Unresolved(key) => ctx.get_or_create_token(key, TokenKind::PropertyKey)?,
            };
            token_ids.push(token_id);
        }

        // eval batch
        let mut builder = self.typ().array_builder(chunk.len()).into_property_map().unwrap();
        let children = self
            .properties
            .iter()
            .map(|(_, e)| e.eval_batch(chunk, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        let num_prop = children.len();
        // build array
        for i in 0..chunk.len() {
            // build mapb
            let mut map_builder = PropertyMapMut::with_capacity(num_prop);

            for prop_idx in 0..children.len() {
                let key_id = token_ids[prop_idx];
                let value = children[prop_idx].get(i);
                map_builder
                    .insert(key_id, value)
                    .map_err(|e| EvalError::MapbError(e.to_string()))?
            }
            let map_value = map_builder.freeze();
            builder.append(Some(map_value.as_ref().into()));
        }

        Ok(builder.finish().into())
    }
}

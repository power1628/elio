use mojito_common::TokenId;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{Array, ArrayBuilder, ArrayImpl};
use mojito_common::data_type::DataType;
use mojito_common::store_types::PropertyValue;

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

// This is the property read operation, so token must be exists
pub struct PropertyAccessExpr {
    pub input: BoxedExpression,
    // currently only allow property access by token id
    key: TokenId,
}

impl PropertyAccessExpr {
    pub fn new(input: BoxedExpression, key: TokenId) -> Self {
        Self { input, key }
    }
}

impl Expression for PropertyAccessExpr {
    fn typ(&self) -> DataType {
        DataType::Property
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &dyn EvalCtx) -> Result<ArrayImpl, EvalError> {
        let input = self.input.eval_batch(chunk, ctx)?;
        assert_eq!(input.data_type(), DataType::PropertyMap);
        let input = input.as_property_map().unwrap();
        let mut builder = self.typ().array_builder(input.len()).into_property().unwrap();
        for item in input.iter() {
            match item {
                Some(map_value) => builder.append(
                    map_value
                        .get(self.key)
                        .map(PropertyValue::from_map_entry_value)
                        .as_ref(),
                ),
                None => builder.append(None),
            }
        }
        Ok(builder.finish().into())
    }
}

use mojito_common::TokenId;
use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayBuilder, ArrayImpl};
use mojito_common::data_type::DataType;

use crate::error::EvalError;
use crate::impl_::{BoxedExpression, EvalCtx, Expression};

pub struct PropertyAccessExpr {
    pub input: BoxedExpression,
    // currently only allow property access by token id
    key: TokenId,
}

impl Expression for PropertyAccessExpr {
    fn typ(&self) -> DataType {
        DataType::Property
    }

    fn eval_batch(&self, chunk: &DataChunk, ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
        let input = self.input.eval_batch(chunk, ctx)?;
        assert_eq!(input.data_type(), DataType::PropertyMap);
        let input = input.as_property_map().unwrap();
        // i think we should refactory or array system and type systems
        // use the arrow-rs as physical type and arrays
        todo!()
    }
}

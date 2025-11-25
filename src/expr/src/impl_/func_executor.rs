use mojito_common::array::{Array, ArrayBuilder};
use mojito_common::data_type::DataType;
use mojito_common::scalar::Scalar;

use crate::error::EvalError;

pub(crate) struct UnaryExecutor;

impl UnaryExecutor {
    // Function must have the following properties
    // 1. propagate nulls
    // 2. do not generate nulls
    // 3. do not throw erro
    pub fn execute_simple<I: Scalar, O: Scalar, F>(
        input: &I::ArrayType,
        func: F,
        // output data type
        // this is an hack
        // should get ride of this
        typ: DataType,
    ) -> Result<O::ArrayType, EvalError>
    where
        F: Fn(I::RefType<'_>) -> O,
    {
        let mut builder = <O::ArrayType as Array>::Builder::with_capacity(input.len(), typ);
        for item in input.iter() {
            match item {
                Some(arg) => builder.push(Some(func(arg).as_scalar_ref())),
                None => builder.push(None),
            }
        }
        Ok(builder.finish())
    }
}

pub(crate) struct BinaryExecutor;

/// TODO(pgao): execute with nulls and without nulls
impl BinaryExecutor {
    /// generate case of execution, return Result<Option<Item>>
    pub fn execute<LeftArray: Array, RightArray: Array, ResultArray: Array, F>(
        _left: &LeftArray,
        _right: &RightArray,
        _func: F,
    ) -> ResultArray
    where
        F: Fn(LeftArray::RefItem<'_>, RightArray::RefItem<'_>) -> ResultArray::OwnedItem,
    {
        todo!()
    }
}

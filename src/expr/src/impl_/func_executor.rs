use mojito_common::array::{Array, ArrayBuilder, ArrayImpl};
use mojito_common::scalar::{Scalar, *};

use crate::error::EvalError;

pub(crate) struct UnaryExecutor;

impl UnaryExecutor {
    pub fn execute_simple<I: Scalar, O: Scalar, F>(input: &ArrayImpl, func: F) -> Result<ArrayImpl, EvalError>
    where
        F: Fn(I::RefType<'_>) -> O,
        for<'a> &'a I::ArrayType: From<&'a ArrayImpl>,
    {
        // down cast ArrayImpl to I::ArrayType
        let input: &I::ArrayType = input.into();
        let mut builder = <O::ArrayType as Array>::Builder::with_capacity(input.len());
        for item in input.iter() {
            match item {
                Some(arg) => builder.append(Some(func(arg).as_scalar_ref())),
                None => builder.append(None),
            }
        }
        Ok(builder.finish().into())
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

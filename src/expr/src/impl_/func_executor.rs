use mojito_common::array::{Array, mask::Mask};

use crate::error::EvalError;

pub(crate) struct UnaryExecutor;

impl UnaryExecutor {
    pub fn execute<InputArray: Array, OutputArray: Array, F>(
        input: &InputArray,
        sel: &Mask,
        func: F,
    ) -> Result<OutputArray, EvalError>
    where
        F: Fn(InputArray::RefItem<'_>) -> OutputArray::OwnedItem,
    {
        todo!()
    }
}

pub(crate) struct BinaryExecutor;

impl BinaryExecutor {
    /// generate case of execution, return Result<Option<Item>>
    pub fn execute<LeftArray: Array, RightArray: Array, ResultArray: Array, F>(
        left: &LeftArray,
        right: &RightArray,
        sel: &Mask,
        func: F,
    ) -> ResultArray
    where
        F: Fn(LeftArray::RefItem<'_>, RightArray::RefItem<'_>) -> ResultArray::OwnedItem,
    {
        todo!()
    }

    // execute that do not propagate nulls
    // TODO(pgao): optimize
    pub fn try_execute<LeftArray: Array, RightArray: Array, ResultArray: Array, F>(
        left: &LeftArray,
        right: &RightArray,
        sel: &Mask,
        func: F,
    ) -> Result<ResultArray, EvalError>
    where
        F: Fn(LeftArray::RefItem<'_>, RightArray::RefItem<'_>) -> Result<ResultArray::OwnedItem, EvalError>,
    {
        // prepare nulls

        // call execute func for each item and call builder to build results
        todo!()
    }

    /// execute function that will produce nulls.
    pub fn execute_with_nulls<LeftArray: Array, RightArray: Array, ResultArray: Array, F>(
        left: &LeftArray,
        right: &RightArray,
        sel: &Mask,
        func: F,
    ) -> ResultArray
    where
        F: Fn(LeftArray::RefItem<'_>, RightArray::RefItem<'_>) -> Option<ResultArray::OwnedItem>,
    {
        todo!()
    }

    /// execute function that never produce nulls
    pub fn execute_without_nulls<LeftArray: Array, RightArray: Array, ResultArray: Array, F>(
        left: &LeftArray,
        right: &RightArray,
        func: F,
    ) -> Result<ResultArray, EvalError>
    where
        F: Fn(LeftArray::RefItem<'_>, RightArray::RefItem<'_>) -> ResultArray::OwnedItem,
    {
        todo!()
    }
}

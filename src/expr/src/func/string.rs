//! String functions
//! - uppper
//! - lower

use std::collections::HashMap;

use mojito_common::array::chunk::DataChunk;
use mojito_common::array::{ArrayImpl, StringArray};
use mojito_common::data_type::DataType;

use crate::define_function;
use crate::error::EvalError;
use crate::func::sig::FuncDef;
use crate::impl_::EvalCtx;
use crate::impl_::func_executor::UnaryExecutor;

fn string_upper(s: &str) -> String {
    s.to_uppercase()
}

fn string_lower(s: &str) -> String {
    s.to_lowercase()
}

fn upper_function(input: &DataChunk, _ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
    let arg: &StringArray = input.column(0).as_ref().into();

    let res: StringArray = UnaryExecutor::execute_simple::<String, String, _>(arg, string_upper, DataType::String)?;
    Ok(res.into())
}

fn lower_function(input: &DataChunk, _ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
    let arg: &StringArray = input.column(0).as_ref().into();

    let res: StringArray = UnaryExecutor::execute_simple::<String, String, _>(arg, string_lower, DataType::String)?;
    Ok(res.into())
}

// register function here

pub(crate) fn register(registry: &mut HashMap<String, FuncDef>) {
    // lower
    let lower = define_function!(
        name: "lower",
        impls: [
            {
                args: [String],
                ret: String,
                func: lower_function,
            }
        ],
        is_agg: false
    );
    // upper
    let upper = define_function!(
        name: "upper",
        impls: [
            {
                args: [String],
                ret: String,
                func: upper_function,
            }
        ],
        is_agg: false
    );

    registry.insert(lower.name.clone(), lower);
    registry.insert(upper.name.clone(), upper);
}

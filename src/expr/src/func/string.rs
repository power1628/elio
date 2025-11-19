//! String functions
//! - uppper
//! - lower

use std::collections::HashMap;

use mojito_common::array::{ArrayImpl, StringArray, chunk::DataChunk};

use crate::{
    define_function,
    error::EvalError,
    func::sig::FuncDef,
    impl_::{EvalCtx, func_executor::UnaryExecutor},
};

fn string_upper(s: &str) -> String {
    s.to_uppercase()
}

fn string_lower(s: &str) -> String {
    s.to_lowercase()
}

fn upper_function(input: &DataChunk, _ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
    let arg: &StringArray = input.columns[0].as_ref().into();
    let sel = &input.sel;

    let res: StringArray = UnaryExecutor::execute(arg, sel, string_upper)?;
    Ok(res.into())
}

fn lower_function(input: &DataChunk, _ctx: &EvalCtx) -> Result<ArrayImpl, EvalError> {
    let arg: &StringArray = input.columns[0].as_ref().into();
    let sel = &input.sel;

    let res: StringArray = UnaryExecutor::execute(arg, sel, string_lower)?;
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
}

//! register operators
//! including:
//! - AND
//! - OR
//! - EQ
//! - NOT
//! - GT
//! - LT
//! - GTE
//! - LTE
//! - PLUS
//! - MINUS
//! - MULTIPLY
//! - DIVIDE
//! - MOD
//! - IS NULL
//! - IS NOT NULL

use bitvec::prelude::*;
use expr_macros::cypher_func;
use mojito_common::array::*;
use mojito_common::scalar::*;

use crate::define_function;
use crate::error::EvalError;
use crate::func::FunctionRegistry;

#[cypher_func(batch_name = "bool_and_batch", sig = "(bool, bool) -> bool")]
fn bool_and(arg0: bool, arg1: bool) -> Result<bool, EvalError> {
    Ok(arg0 && arg1)
}

#[cypher_func(batch_name = "bool_or_batch", sig = "(bool, bool) -> bool")]
fn bool_or(arg0: bool, arg1: bool) -> Result<bool, EvalError> {
    Ok(arg0 || arg1)
}

#[cypher_func(batch_name = "any_eq_batch", sig = "(any, any) -> bool")]
fn any_eq(arg0: ScalarRef<'_>, arg1: ScalarRef<'_>) -> Result<bool, EvalError> {
    Ok(arg0 == arg1)
}

pub(crate) fn register(registry: &mut FunctionRegistry) {
    let and = define_function!( name: "and", impls: [ {args: [{exact Bool}, {exact Bool}], ret: Bool, func: bool_and_batch}],is_agg: false);
    let or = define_function!( name: "or", impls: [ {args: [{exact Bool}, {exact Bool}], ret: Bool, func: bool_or_batch}],is_agg: false);
    let equal = define_function!( name: "equal", impls: [ {args: [{exact Any}, {exact Any}], ret: Bool, func: any_eq_batch}],is_agg: false);

    registry.insert(and);
    registry.insert(or);
    registry.insert(equal);
}

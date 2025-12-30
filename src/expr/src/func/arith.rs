//! Arithmetic functions
//! - add
//! - subtract
//! - multiply
//! - divide
//! - modulo
//! - pow

use bitvec::prelude::*;
use expr_macros::cypher_func;
use mojito_common::array::*;
use mojito_common::data_type::F64;
use mojito_common::scalar::*;

use crate::define_function;
use crate::error::EvalError;
use crate::func::FunctionRegistry;

#[cypher_func(batch_name = "any_add_batch", sig = "(any, any) -> any")]
fn any_add(lhs: ScalarRef<'_>, rhs: ScalarRef<'_>) -> Result<ScalarValue, EvalError> {
    match (lhs, rhs) {
        (ScalarRef::Null, _) | (_, ScalarRef::Null) => Ok(ScalarValue::Unknown),
        // Numeric
        (ScalarRef::Integer(a), ScalarRef::Integer(b)) => a
            .checked_add(b)
            .map(ScalarValue::Integer)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![a.to_string(), b.to_string()])),
        (ScalarRef::Float(a), ScalarRef::Float(b)) => Ok(ScalarValue::Float(a + b)),
        (ScalarRef::Integer(a), ScalarRef::Float(b)) => Ok(ScalarValue::Float(F64::from(a as f64) + b)),
        (ScalarRef::Float(a), ScalarRef::Integer(b)) => Ok(ScalarValue::Float(a + F64::from(b as f64))),

        // String
        (ScalarRef::String(a), ScalarRef::String(b)) => Ok(ScalarValue::String(format!("{}{}", a, b))),
        (ScalarRef::String(a), b) => Ok(ScalarValue::String(format!("{}{}", a, b.to_owned_scalar()))),
        (a, ScalarRef::String(b)) => Ok(ScalarValue::String(format!("{}{}", a.to_owned_scalar(), b))),

        // List
        (ScalarRef::List(a), ScalarRef::List(b)) => {
            let values: Vec<ScalarValue> = a.iter().chain(b.iter()).map(|x| x.to_owned_scalar()).collect();
            Ok(ScalarValue::List(Box::new(ListValue::new(values))))
        }
        (ScalarRef::List(a), b) => {
            let mut values: Vec<ScalarValue> = a.iter().map(|x| x.to_owned_scalar()).collect();
            values.push(b.to_owned_scalar());
            Ok(ScalarValue::List(Box::new(ListValue::new(values))))
        }
        (a, ScalarRef::List(b)) => {
            let mut values = vec![a.to_owned_scalar()];
            values.extend(b.iter().map(|x| x.to_owned_scalar()));
            Ok(ScalarValue::List(Box::new(ListValue::new(values))))
        }

        // Temporal
        (ScalarRef::Date(d), ScalarRef::Duration(dur)) => d
            .checked_add(&dur)
            .map(ScalarValue::Date)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![d.to_string(), dur.to_string()])),
        (ScalarRef::Duration(dur), ScalarRef::Date(d)) => d
            .checked_add(&dur)
            .map(ScalarValue::Date)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![d.to_string(), dur.to_string()])),

        (ScalarRef::LocalTime(t), ScalarRef::Duration(dur)) => t
            .checked_add(&dur)
            .map(ScalarValue::LocalTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![t.to_string(), dur.to_string()])),
        (ScalarRef::Duration(dur), ScalarRef::LocalTime(t)) => t
            .checked_add(&dur)
            .map(ScalarValue::LocalTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![t.to_string(), dur.to_string()])),

        (ScalarRef::LocalDateTime(dt), ScalarRef::Duration(dur)) => dt
            .checked_add(&dur)
            .map(ScalarValue::LocalDateTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![dt.to_string(), dur.to_string()])),
        (ScalarRef::Duration(dur), ScalarRef::LocalDateTime(dt)) => dt
            .checked_add(&dur)
            .map(ScalarValue::LocalDateTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![dt.to_string(), dur.to_string()])),

        (ScalarRef::ZonedDateTime(dt), ScalarRef::Duration(dur)) => dt
            .checked_add(&dur)
            .map(ScalarValue::ZonedDateTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![dt.to_string(), dur.to_string()])),
        (ScalarRef::Duration(dur), ScalarRef::ZonedDateTime(dt)) => dt
            .checked_add(&dur)
            .map(ScalarValue::ZonedDateTime)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![dt.to_string(), dur.to_string()])),

        (ScalarRef::Duration(a), ScalarRef::Duration(b)) => a
            .checked_add(&b)
            .map(ScalarValue::Duration)
            .ok_or_else(|| EvalError::arithmetic_overflow("add", vec![a.to_string(), b.to_string()])),

        _ => Err(EvalError::type_error(format!("Cannot add {:?} and {:?}", lhs, rhs))),
    }
}

fn any_sub(lhs: ScalarRef<'_>, rhs: ScalarRef<'_>) -> Result<ScalarValue, EvalError> {
    todo!()
}

pub(crate) fn register(registry: &mut FunctionRegistry) {
    let add = define_function!(
        name: "add",
        impls: [
            {args: [{exact Any}, {exact Any}], ret: Any, func: any_add_batch}
        ],
        is_agg: false
    );
    registry.insert(add);
}

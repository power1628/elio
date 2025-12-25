//! Temporal functions:
//!
//! Date Functions:
//! - date:
//!   - input: String | Date | ZonedDateTime
//!   - output: Date
//!   - propogate nulls: true
//!   - produce nulls: false
//!   - error on invalid input: true
//!   
//!
//! LocalTime Functions:
//!
//! LocalDateTime Functions:
//!
//! ZonedDateTime Functions:
//!
//! Duration Functions:

use bitvec::prelude::*;
use expr_macros::cypher_func;
use mojito_common::array::*;
use mojito_common::scalar::*;

use crate::define_function;
use crate::error::EvalError;
use crate::func::FunctionRegistry;

#[cypher_func(batch_name = "date_batch", sig = "(any) -> any")]
fn date(arg: ScalarRef<'_>) -> Result<ScalarValue, EvalError> {
    match arg {
        ScalarRef::Date(date) => Ok(ScalarValue::Date(date)),
        ScalarRef::LocalDateTime(local_date_time) => Ok(ScalarValue::Date(local_date_time.to_date())),
        ScalarRef::ZonedDateTime(zoned_date_time) => Ok(ScalarValue::Date(zoned_date_time.to_date())),
        ScalarRef::String(s) => Ok(ScalarValue::Date(
            Date::try_from(s).map_err(|_| EvalError::invalid_argument("date()", "yyyy-MM-dd", s))?,
        )),
        _ => Err(EvalError::invalid_argument(
            "date()",
            "Date, LocalTime, LocalDateTime, ZonedDateTime, or String",
            format!("{:?}", arg),
        )),
    }
}

pub(crate) fn register(registry: &mut FunctionRegistry) {
    let date = define_function!( name: "date", impls: [{ args: [{anyof String | Date | LocalDateTime | ZonedDateTime}], ret: Any, func: date_batch}],is_agg: false);

    registry.insert(date);
}

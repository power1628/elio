use mojito_common::{data_type::DataType, value::Value};
use ordered_float::OrderedFloat;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Constant {
    pub data: Value,
    pub typ: DataType,
}

impl Constant {
    pub fn boolean(b: bool) -> Self {
        Self {
            data: Value::Boolean(b),
            typ: DataType::Boolean,
        }
    }

    pub fn integer(i: i64) -> Self {
        Self {
            data: Value::Integer(i),
            typ: DataType::Integer,
        }
    }

    pub fn float(f: f64) -> Self {
        Self {
            data: Value::Float(OrderedFloat(f)),
            typ: DataType::Float,
        }
    }

    pub fn string(s: String) -> Self {
        Self {
            data: Value::String(s),
            typ: DataType::String,
        }
    }

    pub fn null() -> Self {
        Self {
            data: Value::Null,
            typ: DataType::Null,
        }
    }
}

impl ExprNode for Constant {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }
}

impl From<Constant> for Expr {
    fn from(val: Constant) -> Self {
        Expr::Constant(val)
    }
}

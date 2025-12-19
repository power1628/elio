use mojito_common::data_type::{DataType, F64};
use mojito_common::scalar::ScalarValue;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Constant {
    pub data: Option<ScalarValue>,
    pub typ: DataType,
}

impl Constant {
    pub fn boolean(b: bool) -> Self {
        Self {
            data: Some(ScalarValue::Bool(b)),
            typ: DataType::Bool,
        }
    }

    pub fn integer(i: i64) -> Self {
        Self {
            data: Some(ScalarValue::Integer(i)),
            typ: DataType::Integer,
        }
    }

    pub fn float(f: F64) -> Self {
        Self {
            data: Some(ScalarValue::Float(f)),
            typ: DataType::Float,
        }
    }

    pub fn string(s: String) -> Self {
        Self {
            data: Some(ScalarValue::String(s)),
            typ: DataType::String,
        }
    }

    pub fn null(typ: DataType) -> Self {
        Self { data: None, typ }
    }

    pub fn pretty(&self) -> String {
        self.data
            .as_ref()
            .map_or("null".to_string(), |d| d.as_scalar_ref().to_string())
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

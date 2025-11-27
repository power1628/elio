use mojito_common::data_type::{DataType, F64};
use mojito_common::scalar::ScalarImpl;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Constant {
    pub data: Option<ScalarImpl>,
    pub typ: DataType,
}

impl Constant {
    pub fn boolean(b: bool) -> Self {
        Self {
            data: Some(ScalarImpl::Bool(b)),
            typ: DataType::Bool,
        }
    }

    pub fn integer(i: i64) -> Self {
        Self {
            data: Some(ScalarImpl::Integer(i)),
            typ: DataType::Integer,
        }
    }

    pub fn float(f: F64) -> Self {
        Self {
            data: Some(ScalarImpl::Float(f)),
            typ: DataType::Float,
        }
    }

    pub fn string(s: String) -> Self {
        Self {
            data: Some(ScalarImpl::String(s)),
            typ: DataType::String,
        }
    }

    pub fn null() -> Self {
        Self {
            data: None,
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

use mojito_common::data_type::DataType;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct AggCall {
    // TODO(pgao): should be registered name
    pub func: String,
    pub args: Vec<Expr>,
    pub distinct: bool,
    typ: DataType,
}

impl AggCall {
    pub fn new_unchecked(func: String, args: Vec<Expr>, distinct: bool, typ: DataType) -> Self {
        Self {
            func,
            args,
            distinct,
            typ,
        }
    }
}

impl ExprNode for AggCall {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }
}

impl From<AggCall> for Expr {
    fn from(val: AggCall) -> Self {
        Expr::AggCall(val)
    }
}

use mojito_common::data_type::DataType;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct FuncCall {
    // TODO(pgao): We should have an FuncImplName Enum here
    // Or we should have an FuncName and args types to resolve to the function impl
    pub func: String,
    pub args: Vec<Expr>,
    typ: DataType,
}

impl FuncCall {
    pub fn new_unchecked(func: String, args: Vec<Expr>, typ: DataType) -> Self {
        Self { func, args, typ }
    }
}

impl ExprNode for FuncCall {
    fn typ(&self) -> DataType {
        self.typ.clone()
    }
}

impl From<FuncCall> for Expr {
    fn from(val: FuncCall) -> Self {
        Expr::FuncCall(val)
    }
}

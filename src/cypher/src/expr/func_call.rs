use mojito_common::data_type::DataType;

use crate::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncCall {
    // TODO(pgao): We should have an FuncImplName Enum here
    // Or we should have an FuncName and args types to resolve to the function impl
    pub func: String,
    pub args: Vec<Expr>,
    typ: DataType,
}

impl FuncCall {
    pub fn new(func: String, args: Vec<Expr>, typ: DataType) -> Self {
        Self { func, args, typ }
    }
}

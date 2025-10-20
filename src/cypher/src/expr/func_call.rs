use mojito_common::data_type::DataType;
use mojito_expr::func::sig::FuncImpl;

use crate::expr::Expr;

pub struct FuncCall {
    pub func: FuncImpl,
    pub args: Vec<Expr>,
    typ: DataType,
}

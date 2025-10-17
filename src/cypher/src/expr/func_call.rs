use mojito_common::data_type::DataType;

use crate::{catalog::func::FuncImpl, expr::Expr};

pub struct FuncCall {
    pub func: FuncImpl,
    pub args: Vec<Expr>,
    typ: DataType,
}

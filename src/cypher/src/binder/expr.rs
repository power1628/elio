use mojito_parser::ast;

use crate::{error::PlanError, expr::Expr};

pub fn bind_expr(ectx: &ExprContext, expr: &ast::Expr) -> Result<Expr, PlanError> {
    todo!()
}

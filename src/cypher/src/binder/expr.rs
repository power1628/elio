use mojito_parser::ast;

use crate::{
    binder::{BindContext, scope::Scope},
    error::PlanError,
    expr::Expr,
};

pub struct ExprContext<'a> {
    pub bctx: &'a BindContext<'a>,
    pub scope: &'a Scope,
    pub name: &'a str,
}

pub fn bind_expr(ectx: &ExprContext, expr: &ast::Expr) -> Result<Expr, PlanError> {
    todo!()
}

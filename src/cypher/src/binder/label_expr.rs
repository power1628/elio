use std::collections::HashSet;

use mojito_parser::ast;

use crate::{
    binder::pattern::PatternContext,
    error::PlanError,
    expr::{Expr, label::LabelExpr, label::LabelOp},
};

pub(crate) fn bind_label_expr(
    pctx: &PatternContext,
    expr: Box<Expr>,
    label_expr: &ast::LabelExpr,
) -> Result<Box<Expr>, PlanError> {
    match label_expr {
        ast::LabelExpr::Label(label) => {
            // resolve label
            let label = pctx.bctx.catalog().get_label_id(label).into();
            Ok(Expr::Label(LabelExpr {
                expr: expr.clone(),
                op: LabelOp::HasAll(HashSet::from_iter(vec![label])),
            })
            .boxed())
        }
        ast::LabelExpr::Or(lhs, rhs) => {
            let lhs = bind_label_expr(pctx, expr.clone(), lhs)?;
            let rhs = bind_label_expr(pctx, expr.clone(), rhs)?;
            Ok(lhs.and(*rhs).boxed())
        }
    }
}

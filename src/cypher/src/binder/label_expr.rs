use std::collections::HashSet;

use mojito_common::TokenKind;
use mojito_parser::ast;

use crate::binder::pattern::PatternContext;
use crate::error::PlanError;
use crate::expr::Expr;
use crate::expr::label::{LabelExpr, LabelOp};

pub(crate) fn bind_label_expr(
    pctx: &PatternContext,
    expr: Box<Expr>,
    label_expr: &ast::LabelExpr,
) -> Result<Box<Expr>, PlanError> {
    match label_expr {
        ast::LabelExpr::Label(label) => {
            // resolve label
            let label = pctx.bctx.resolve_token(label, TokenKind::Label);
            Ok(Expr::LabelExpr(LabelExpr {
                entity: expr.clone(),
                op: LabelOp::HasAll(HashSet::from_iter(vec![label])),
            })
            .boxed())
        }
        ast::LabelExpr::Or(lhs, rhs) => {
            let lhs = bind_label_expr(pctx, expr.clone(), lhs)?;
            let rhs = bind_label_expr(pctx, expr.clone(), rhs)?;
            Ok(lhs.and(*rhs).boxed())
        }
        ast::LabelExpr::And(lhs, rhs) => {
            let lhs = bind_label_expr(pctx, expr.clone(), lhs)?;
            let rhs = bind_label_expr(pctx, expr.clone(), rhs)?;
            Ok(lhs.and(*rhs).boxed())
        }
    }
}

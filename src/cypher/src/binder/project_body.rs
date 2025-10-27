use std::collections::HashSet;

use indexmap::IndexMap;
use itertools::Itertools;
use mojito_catalog::FunctionCatalog;
use mojito_parser::ast::{self, ProjectionKind, ReturnItem, ReturnItems};

use crate::{
    binder::{
        BindContext,
        builder::IrSingleQueryBuilder,
        expr::bind_expr,
        scope::{Scope, ScopeItem},
    },
    error::{PlanError, SemanticError},
    expr::{Expr, ExprNode, FilterExprs, VariableRef},
    ir::horizon::{DistinctProjection, QueryHorizon, QueryProjection, RegularProjection},
    variable::Variable,
};

pub enum ClauseKind {
    With,
    Return,
}

pub enum BoundReturnItems {
    Project(Vec<(Variable, Expr)>),
    Aggregate(),
}

pub fn bind_return_items(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    scope: Scope,
    distinct: bool,
    for_clause: &ClauseKind,
    return_items @ ast::ReturnItems { projection_kind, items }: &ast::ReturnItems,
) -> Result<Scope, PlanError> {
    let aggs = items
        .iter()
        .map(|x| extract_top_level_aggregate(bctx, &x.expr))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    if aggs.is_empty() {
        // regular projection or distinct
        let mut out_scope = Scope::empty();
        let mut projections = IndexMap::new();
        let ectx = bctx.derive_expr_context(&scope, "WITH Clause");
        for ReturnItem { expr, alias } in items {
            let bound_expr = bind_expr(&ectx, &bctx.outer_scopes, expr)?;
            if matches!(for_clause, ClauseKind::With) & alias.is_none() {
                return Err(SemanticError::return_item_must_be_aliased(&expr.to_string(), "WITH"));
            }
            let symbol = alias.clone().unwrap_or(expr.to_string());
            let var_name = bctx.variable_generator.named(&symbol);
            let item = ScopeItem {
                symbol: Some(symbol.to_string()),
                variable: var_name.clone(),
                expr: HashSet::from_iter(vec![*expr.clone()]),
                typ: bound_expr.typ(),
            };
            out_scope.add_item(item);
            projections.insert(var_name, bound_expr);
        }

        // expand star to output_scope
        if projection_kind.include_existing() {
            for in_item in scope.symbol_items() {
                if out_scope.resolve_symbol(in_item.symbol.as_ref().unwrap()).is_none() {
                    out_scope.add_item(in_item.clone());
                    let bound_expr = VariableRef::from_variable(&in_item.as_variable()).into();
                    projections.insert(in_item.variable.clone(), bound_expr);
                }
            }
        }

        let horizon = {
            if distinct {
                let proj = DistinctProjection {
                    group_by: projections,
                    order_by: Default::default(),
                    pagination: Default::default(),
                    filter: FilterExprs::empty(),
                };
                let proj = QueryProjection::Distinct(proj);
                QueryHorizon::Project(proj)
            } else {
                let proj = QueryProjection::Regular(RegularProjection {
                    items: projections,
                    order_by: Default::default(),
                    pagination: Default::default(),
                    filter: FilterExprs::empty(),
                });
                QueryHorizon::Project(proj)
            }
        };
        builder.tail_mut().unwrap().horizon = horizon;
        return Ok(out_scope);
    }

    // handle aggreegation

    // MATCH (a)--(b) WITH *, a.name AS col1, SUM(a.age + b.age)/COUNT(b) AS b
    // agg_in_scope = [*, a.name AS col1, a.age + b.age AS col2, b]
    // agg_out_scope = [*, col1, SUM(col2), COUNT(b)]
    // project_out_scope = [*, col1, SUM(col2)/COUNT(b) AS b]
    todo!()
}

fn extract_top_level_aggregate(bctx: &BindContext, expr: &ast::Expr) -> Result<Vec<ast::Expr>, PlanError> {
    let mut aggs = vec![];
    match expr {
        ast::Expr::Unary { oprand, .. } => aggs.extend(extract_top_level_aggregate(bctx, oprand)?),
        ast::Expr::Binary { left, right, .. } => {
            let children = extract_top_level_aggregate(bctx, &left)?
                .into_iter()
                .chain(extract_top_level_aggregate(bctx, right)?)
                .collect_vec();
            aggs.extend(children);
        }
        ast::Expr::FunctionCall { name, args, .. } => {
            let func = resolve_function(bctx, name)?;
            if func.func.is_agg {
                aggs.push(expr.clone());
            } else {
                let children = args
                    .iter()
                    .map(|x| extract_top_level_aggregate(bctx, x))
                    .collect::<Result<Vec<_>, _>>()?;
                aggs.extend(children.into_iter().flatten());
            }
        }
        _ => (),
    };
    Ok(aggs)
}

// TODO(pgao): should catalog with static lifetime
fn resolve_function(bctx: &BindContext, name: &str) -> Result<FunctionCatalog, PlanError> {
    bctx.catalog()
        .get_function_by_name(name)
        .cloned()
        .ok_or(PlanError::from(SemanticError::unknown_function(name, "")))
}

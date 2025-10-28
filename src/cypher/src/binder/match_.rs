use mojito_parser::ast;

use crate::{
    binder::{
        BindContext,
        builder::IrSingleQueryBuilder,
        expr::bind_expr,
        pattern::{PatternContext, bind_pattern},
        query::ClauseKind,
        scope::Scope,
    },
    error::PlanError,
    expr::FilterExprs,
    ir::query_graph::QueryGraph,
};

pub(crate) fn bind_match(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    scope: Scope,
    match_ @ ast::MatchClause {
        optional,
        mode,
        pattern,
        where_,
    }: &ast::MatchClause,
) -> Result<Scope, PlanError> {
    if matches!(mode, ast::MatchMode::WALK) {
        return Err(PlanError::not_supported("TRAIL walk mode not supported"));
    }

    // add the pattern graph to builder
    let pctx = PatternContext {
        bctx,
        clause: ClauseKind::Match,
        name: &match_.to_string(),
        allow_update: false,
        reject_qpp: false,
        reject_named_path: false,
        reject_selective: false,
    };
    let (paths, scope) = bind_pattern(&pctx, scope, &pattern.patterns)?;

    let qg = {
        let mut qg = QueryGraph::empty();
        paths.iter().for_each(|path| qg.add_path_pattern(path));
        qg
    };

    if *optional {
        // SAFETY: tail must exists
        builder.tail_mut().unwrap().query_graph.add_optional_qg(qg);
    } else {
        builder.tail_mut().unwrap().query_graph.merge(qg);
    }

    let filter = {
        let mut filter = FilterExprs::empty();
        if let Some(expr) = where_ {
            let ectx = bctx.derive_expr_context(&scope, "MATCH WHERE");
            let expr = bind_expr(&ectx, &[], expr)?;
            filter.push(expr);
        }
        filter
    };

    builder.tail_mut().unwrap().query_graph.add_filter(filter);

    // TODO(pgao): semantic check match mode
    Ok(scope)
}

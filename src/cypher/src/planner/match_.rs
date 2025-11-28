use super::*;
use crate::ir::query_graph::QueryGraph;
use crate::plan_node::{Argument, ArgumentInner};
use crate::planner::component::plan_qg_simple;

pub fn plan_match(
    ctx: &mut PlannerContext,
    _part @ IrSingleQueryPart {
        query_graph,
        horizon: _,
    }: &IrSingleQueryPart,
) -> Result<Box<PlanExpr>, PlanError> {
    // TODO(pgao): pushdown horizon order by to query graph
    plan_query_graph(ctx, query_graph)
}

fn plan_query_graph(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<Box<PlanExpr>, PlanError> {
    // get connected component
    let qgs = qg.connected_component();
    if qgs.len() > 1 {
        return Err(PlanError::not_supported("mutliple query graph not supported."));
    }

    if qgs.is_empty() {
        // put an empty argument here as the start point
        let root = PlanExpr::Argument(Argument::new(ArgumentInner {
            variables: vec![],
            ctx: ctx.ctx.clone(),
        }));
        return Ok(root.boxed());
    }

    // plan component
    let plans = qgs
        .iter()
        .map(move |qg| plan_component(ctx, qg))
        .collect::<Result<Vec<_>, _>>()?;

    // TODO(pgao): connect component by cartisen product
    Ok(plans[0].clone())
}

fn plan_component(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<Box<PlanExpr>, PlanError> {
    // we can have different qg planning strategy here
    plan_qg_simple(ctx, qg)
}

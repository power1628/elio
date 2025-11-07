use crate::ir::query_graph::QueryGraph;

use super::*;

pub fn plan_match(
    ctx: &mut PlannerContext,
    _part @ IrSingleQueryPart { query_graph, horizon }: &IrSingleQueryPart,
) -> Result<PlanExpr, PlanError> {
    // TODO(pgao): pushdown horizon order by to query graph
    plan_query_graph(ctx, query_graph)
}

fn plan_query_graph(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<PlanExpr, PlanError> {
    // get connected component
    let qgs = qg.connected_component();
    if qgs.len() > 1 {
        return Err(PlanError::not_supported("mutliple query graph not supported."));
    }

    // plan component

    // connect component by cartisen product
    todo!()
}

fn plan_component(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<PlanExpr, PlanError> {
    // start with one node

    // plan node connections
    todo!()
}

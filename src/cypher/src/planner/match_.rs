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

    // plan component

    // connect component
    todo!()
}

fn plan_component(ctx: &mut PlannerContext, qg: &QueryGraph) -> Result<PlanExpr, PlanError> {
    // start with one node

    // plan node connections
    todo!()
}

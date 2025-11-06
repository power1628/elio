use crate::{
    error::PlanError,
    ir::query::{IrSingleQuery, IrSingleQueryPart},
    plan_node::PlanExpr,
    planner::PlannerContext,
};

pub fn plan_single_query(
    ctx: &mut PlannerContext,
    _single_query @ IrSingleQuery { parts }: &IrSingleQuery,
) -> Result<PlanExpr, PlanError> {
    let mut part_iter = parts.iter();
    let head = part_iter.next().unwrap();

    // plan head
    let mut root = plan_head(ctx, head)?;

    // plan tail
    for tail in part_iter {
        root = plan_tail_part(ctx, root, tail)?
    }

    Ok(root)
}

fn plan_head(
    ctx: &mut PlannerContext,
    part @ IrSingleQueryPart { query_graph, horizon }: &IrSingleQueryPart,
) -> Result<PlanExpr, PlanError> {
    // plan match
    todo!()
}

fn plan_tail_part(
    ctx: &mut PlannerContext,
    lhs_plan: PlanExpr,
    part @ IrSingleQueryPart { query_graph, horizon }: &IrSingleQueryPart,
) -> Result<PlanExpr, PlanError> {
    // plan query graph with lhs

    // plan horizon
    todo!()
}

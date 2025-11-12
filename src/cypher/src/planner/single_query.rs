use crate::{
    error::PlanError,
    ir::query::{IrSingleQuery, IrSingleQueryPart},
    plan_node::PlanExpr,
    planner::{PlannerContext, horizon::plan_horizon, match_::plan_match},
};

pub fn plan_single_query(
    ctx: &mut PlannerContext,
    _single_query @ IrSingleQuery { parts }: &IrSingleQuery,
) -> Result<Box<PlanExpr>, PlanError> {
    let mut part_iter = parts.iter();
    let head = part_iter.next().unwrap();

    // plan head
    let root = plan_head(ctx, head)?;

    // plan tail
    // for tail in part_iter {
    // root = plan_tail_part(ctx, root, tail)?
    // }

    // todo!()

    Ok(root)
}

fn plan_head(
    ctx: &mut PlannerContext,
    part @ IrSingleQueryPart {
        query_graph: _,
        horizon,
    }: &IrSingleQueryPart,
) -> Result<Box<PlanExpr>, PlanError> {
    // plan match
    let mut root = plan_match(ctx, part)?;
    // plan horizon
    root = plan_horizon(ctx, root, horizon)?;
    Ok(root)
}

fn plan_tail_part(
    ctx: &mut PlannerContext,
    lhs_plan: PlanExpr,
    part @ IrSingleQueryPart { query_graph, horizon }: &IrSingleQueryPart,
) -> Result<Box<PlanExpr>, PlanError> {
    // plan query graph with lhs

    // plan horizon
    todo!()
}

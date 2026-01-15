use crate::error::PlanError;
use crate::ir::mutating_pattern::MutatingPattern;
use crate::ir::query::{IrSingleQuery, IrSingleQueryPart};
use crate::ir::query_project::QueryProjection;
use crate::plan_node::PlanExpr;
use crate::planner::PlannerContext;
use crate::planner::create::plan_create;
use crate::planner::load::plan_load;
use crate::planner::match_::plan_match;
use crate::planner::project::plan_query_projection;

pub fn plan_single_query(
    ctx: &mut PlannerContext,
    _single_query @ IrSingleQuery { parts }: &IrSingleQuery,
) -> Result<Box<PlanExpr>, PlanError> {
    assert!(!parts.is_empty());
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
        query_graph,
        query_project,
    }: &IrSingleQueryPart,
) -> Result<Box<PlanExpr>, PlanError> {
    // Check if this is a LOAD clause - if so, handle it specially
    // NB: if this is an load clause, then there must be no query graph here.
    if let Some(QueryProjection::Load(ir_load)) = query_project {
        assert!(query_graph.mutating_patterns.is_empty());
        assert!(query_graph.rels.is_empty());
        // LOAD is the root - no match or mutating patterns
        return plan_load(ctx, ir_load);
    }

    // plan match
    let mut root = plan_match(ctx, part)?;
    // plan updating pattern
    for mutating_pattern in query_graph.mutating_patterns.iter() {
        root = plan_mutating_pattern(ctx, root, mutating_pattern)?;
    }
    // plan projection
    if let Some(proj) = query_project {
        root = plan_query_projection(ctx, root, proj)?;
    }
    Ok(root)
}

fn plan_mutating_pattern(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    mutating_pattern: &MutatingPattern,
) -> Result<Box<PlanExpr>, PlanError> {
    match mutating_pattern {
        MutatingPattern::Create(create) => plan_create(ctx, root, create),
    }
}

fn plan_tail_part(
    _ctx: &mut PlannerContext,
    _lhs_plan: PlanExpr,
    _part @ IrSingleQueryPart {
        query_graph,
        query_project,
    }: &IrSingleQueryPart,
) -> Result<Box<PlanExpr>, PlanError> {
    // plan query graph with lhs

    // plan apply

    // plan projection
    todo!()
}

use std::collections::HashSet;

use itertools::Itertools;
use mojito_common::order::ColumnOrder;

use crate::{
    error::PlanError,
    expr::FilterExprs,
    ir::{
        horizon::{
            AggregateProjection, DistinctProjection, Pagination, QueryHorizon, QueryProjection, RegularProjection,
            UnwindProjection,
        },
        order::SortItem,
    },
    plan_node::{Filter, FilterInner, PaginationInner, PlanExpr, PlanNode, Project, ProjectInner, Sort, SortInner},
    planner::PlannerContext,
};

pub fn plan_horizon(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    horizon: &QueryHorizon,
) -> Result<Box<PlanExpr>, PlanError> {
    match horizon {
        QueryHorizon::Unwind(unwind) => plan_unwind(ctx, root, unwind),
        QueryHorizon::Project(QueryProjection::Regular(reg)) => plan_project(ctx, root, reg),
        QueryHorizon::Project(QueryProjection::Aggregate(agg)) => plan_aggregate(ctx, root, agg),
        QueryHorizon::Project(QueryProjection::Distinct(dist)) => plan_distinct(ctx, root, dist),
    }
}

fn plan_project(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    _project @ RegularProjection {
        items,
        order_by,
        pagination,
        filter,
    }: &RegularProjection,
) -> Result<Box<PlanExpr>, PlanError> {
    let inner = ProjectInner {
        input: root,
        projections: items.clone().into_iter().collect_vec(),
    };
    let mut root: Box<PlanExpr> = Project::new(inner).into();

    if !filter.is_true() {
        root = plan_selection(ctx, root, filter)?;
    }

    if !order_by.is_empty() {
        root = plan_sort(ctx, root, order_by)?;
    }

    if !pagination.is_empty() {
        root = plan_pagination(ctx, root, pagination)?;
    }

    Ok(root)
}

fn plan_aggregate(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    project @ AggregateProjection {
        group_by,
        aggregate,
        order_by,
        pagination,
        filter,
    }: &AggregateProjection,
) -> Result<Box<PlanExpr>, PlanError> {
    Err(PlanError::not_supported("aggregate clause not implemented yet."))
}

fn plan_distinct(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    project @ DistinctProjection {
        group_by,
        order_by,
        pagination,
        filter,
    }: &DistinctProjection,
) -> Result<Box<PlanExpr>, PlanError> {
    Err(PlanError::not_supported("distinct clause not implemented yet."))
}

fn plan_unwind(
    ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    unwind @ UnwindProjection { variable, expr }: &UnwindProjection,
) -> Result<Box<PlanExpr>, PlanError> {
    Err(PlanError::not_supported("unwind clause not implemented yet."))
}

// WITH a, a.id + 1 AS b, c ORDER BY c.id + 1 ASC
// 1. [optional] project, if sort item needs extra projection
// 2. sort
// 3. [optional] project, remove extra projection items
fn plan_sort(
    ctx: &mut PlannerContext,
    mut root: Box<PlanExpr>,
    order_by: &[SortItem],
) -> Result<Box<PlanExpr>, PlanError> {
    let mut extra_projections = vec![];
    let mut column_orders = vec![];
    for item in order_by {
        if item.needs_extra_project() {
            // TODO(pgao): we can have named once the expr have display trait
            let var = ctx.ctx.var_gen().unnamed();
            extra_projections.push((var, item.expr.clone()));
            column_orders.push(ColumnOrder {
                column: var,
                direction: item.direction,
            });
        } else {
            column_orders.push(ColumnOrder {
                column: item.expr.as_variable_ref().unwrap().name.clone(),
                direction: item.direction,
            });
        }
    }

    // extra project
    if !extra_projections.is_empty() {
        // add extra project
        let empty = PlanExpr::empty(root.ctx());
        let mut inner = ProjectInner::new_from_input(std::mem::replace(&mut root, Box::new(empty)));
        extra_projections
            .iter()
            .for_each(|(name, expr)| inner.add_unchecked(name.clone(), *expr.clone()));
        root = Project::new(inner).into();
    }

    // sort
    {
        let inner = SortInner {
            input: root,
            items: column_orders,
        };
        root = Sort::new(inner).into();
    }

    // remove extra project
    // TODO(pgao): maybe we can use the opt rule to remove unnecessary project
    if !extra_projections.is_empty() {
        let empty = PlanExpr::empty(root.ctx());
        let mut inner = ProjectInner::new_from_input(std::mem::replace(&mut root, Box::new(empty)));
        // inner.retain(|(name, expr)| !extra_projections.iter().any(|(n, _)| name == n));
        let extra_names: HashSet<_> = extra_projections.iter().map(|(n, _)| n).collect();
        inner.retain(|(name, expr)| !extra_names.contains(name));
        root = Project::new(inner).into();
    }
    Ok(root)
}

fn plan_pagination(
    _ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    _pagination @ Pagination { offset, limit }: &Pagination,
) -> Result<Box<PlanExpr>, PlanError> {
    let inner = PaginationInner {
        input: root,
        offset: offset.unwrap_or(0),
        limit: limit.unwrap_or(-1),
    };
    Ok(crate::plan_node::Pagination::new(inner).into())
}

fn plan_selection(
    _ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    filter: &FilterExprs,
) -> Result<Box<PlanExpr>, PlanError> {
    let inner = FilterInner {
        input: root,
        condition: filter.to_owned(),
    };

    Ok(Filter::new(inner).into())
}

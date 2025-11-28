use std::sync::Arc;

use indexmap::IndexMap;
use mojito_common::variable::VariableName;

use crate::error::PlanError;
use crate::ir::query::{IrQuery, IrQueryRoot, IrSingleQueryPart};
use crate::plan_context::PlanContext;
use crate::plan_node::PlanExpr;
use crate::planner::single_query::plan_single_query;

mod component;
mod create;
mod horizon;
mod match_;
mod single_query;
mod tail;

pub struct PlannerContext {
    ctx: Arc<PlanContext>,
    config: PlannerConfig,
}

pub struct PlannerConfig {
    // planner options here
}

pub struct RootPlan {
    pub plan: Box<PlanExpr>,
    pub names: IndexMap<VariableName, String>,
}

pub fn plan_root(
    ctx: &mut PlannerContext,
    _root @ IrQueryRoot { inner, names }: &IrQueryRoot,
) -> Result<RootPlan, PlanError> {
    let IrQuery { queries, union_all: _ } = inner;
    assert!(!queries.is_empty());
    if queries.len() > 1 {
        return Err(PlanError::not_supported("Union all is not supported yet".to_string()));
    }

    let plan = plan_single_query(ctx, &queries[0])?;
    Ok(RootPlan {
        plan,
        names: names.clone(),
    })
}

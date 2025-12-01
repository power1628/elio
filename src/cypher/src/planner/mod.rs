use std::sync::Arc;

use indexmap::IndexMap;
use mojito_common::variable::VariableName;

use crate::error::PlanError;
use crate::ir::query::{IrQuery, IrQueryRoot, IrSingleQueryPart};
use crate::plan_context::PlanContext;
use crate::plan_node::PlanExpr;
use crate::planner::single_query::plan_single_query;
use crate::session::SessionContext;

mod component;
mod create;
mod horizon;
mod match_;
mod single_query;
mod tail;

// planner temporaray state
pub struct PlannerContext {
    ctx: Arc<PlanContext>,
    config: PlannerConfig,
}

#[derive(Default)]
pub struct PlannerConfig {
    // planner options here
}

pub struct RootPlan {
    pub plan: Box<PlanExpr>,
    pub names: IndexMap<VariableName, String>,
}

pub fn plan_root(
    sctx: Arc<dyn SessionContext>,
    _root @ IrQueryRoot { inner, names }: &IrQueryRoot,
) -> Result<RootPlan, PlanError> {
    let plan_ctx = sctx.clone().derive_plan_context();
    let mut ctx = PlannerContext {
        ctx: plan_ctx,
        // generate from session context
        config: Default::default(),
    };

    let IrQuery { queries, union_all: _ } = inner;
    assert!(!queries.is_empty());
    if queries.len() > 1 {
        return Err(PlanError::not_supported("Union all is not supported yet".to_string()));
    }

    let plan = plan_single_query(&mut ctx, &queries[0])?;
    Ok(RootPlan {
        plan,
        names: names.clone(),
    })
}

use std::sync::Arc;

use crate::{plan_context::PlanContext, plan_node::PlanExpr};

use crate::{error::PlanError, ir::query::IrSingleQueryPart};

mod component;
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

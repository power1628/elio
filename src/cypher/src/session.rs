use std::fmt::Debug;
use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_parser::ast;
use mojito_parser::parser::cypher_parser;

use crate::binder::query::bind_root_query;
use crate::error::PlanError;
use crate::plan_context::PlanContext;
use crate::planner::{RootPlan, plan_root};

pub trait SessionContext: Debug + Send + Sync {
    fn catalog(&self) -> &Arc<Catalog>;
    // send notification
    fn derive_plan_context(self: Arc<Self>) -> Arc<PlanContext>;
}

// #[derive(Debug)]
// pub struct SessionContext {
//     pub catalog: Arc<Catalog>,
//     // notification_tx: UnboundedSender<String>,
// }

// impl SessionContext {
//     pub fn derive_plan_context(self: Arc<Self>) -> PlanContext {
//         PlanContext::new(self)
//     }
// }

pub fn parse_statement(stmt: &str) -> Result<ast::Statement, PlanError> {
    cypher_parser::statement(stmt).map_err(PlanError::parse_error)
}

pub fn handle_query(ctx: Arc<dyn SessionContext>, query: &ast::RegularQuery) -> Result<RootPlan, PlanError> {
    // bind
    let ir = bind_root_query(ctx.clone(), query)?;
    // plan
    let plan = plan_root(ctx, &ir)?;
    Ok(plan)
}

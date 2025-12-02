use std::fmt::Debug;
use std::sync::Arc;

use mojito_catalog::FunctionCatalog;
use mojito_catalog::error::CatalogError;
use mojito_common::{TokenId, TokenKind};
use mojito_parser::ast;
use mojito_parser::parser::cypher_parser;

use crate::binder::query::bind_root_query;
use crate::error::PlanError;
use crate::plan_context::PlanContext;
use crate::planner::{RootPlan, plan_root};

/// SessionContext for Cypher Planner
pub trait PlannerSession: Debug + Send + Sync {
    // send notification
    fn derive_plan_context(self: Arc<Self>) -> Arc<PlanContext>;
    // catalog
    fn get_or_create_token(&self, token: &str, kind: TokenKind) -> Result<TokenId, CatalogError>;
    fn get_function_by_name(&self, name: &str) -> Option<&FunctionCatalog>;
    fn get_token_id(&self, token: &str, kind: TokenKind) -> Option<TokenId>;
    // TODO(impl send notification)
    fn send_notification(&self, notification: String);
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

pub fn plan_query(ctx: Arc<dyn PlannerSession>, query: &ast::RegularQuery) -> Result<RootPlan, PlanError> {
    // bind
    let ir = bind_root_query(ctx.clone(), query)?;
    // plan
    let plan = plan_root(ctx, &ir)?;
    Ok(plan)
}

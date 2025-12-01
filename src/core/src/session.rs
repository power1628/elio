use std::collections::HashMap;
use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_common::scalar::Datum;
use mojito_cypher::plan_context::PlanContext;
use mojito_cypher::session::{SessionContext, handle_query, parse_statement};
use mojito_exec::task::{ExecContext, TaskHandle, create_task};
use mojito_parser::ast;

use crate::error::Error;
use crate::result::ResultHandle;

#[derive(Debug)]
pub struct Session {
    pub catalog: Arc<Catalog>,
    pub exec_ctx: Arc<ExecContext>,
}

impl SessionContext for Session {
    fn catalog(&self) -> &Arc<Catalog> {
        &self.catalog
    }

    fn derive_plan_context(self: Arc<Self>) -> Arc<PlanContext> {
        Arc::new(PlanContext::new(self))
    }
}

impl Session {
    pub async fn execute(
        self: &Arc<Self>,
        query: String,
        params: HashMap<String, Datum>,
    ) -> Result<ResultHandle, Error> {
        let ast = parse_statement(&query)?;
        match ast {
            ast::Statement::Query(regular_query) => self.handle_query(&regular_query).await,
        }
    }

    async fn handle_query(self: &Arc<Self>, query: &ast::RegularQuery) -> Result<ResultHandle, Error> {
        let plan = handle_query(self.clone(), query)?;
        // execute query
        let query_id = uuid::Uuid::new_v4().to_string().into();
        let handle = create_task(&self.exec_ctx, query_id, plan).await?;
        Ok(handle)
    }
}

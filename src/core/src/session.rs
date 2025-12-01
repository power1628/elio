use std::collections::HashMap;
use std::sync::Arc;

use async_stream::stream;
use futures::Stream;
use futures::stream::BoxStream;
use mojito_catalog::Catalog;
use mojito_common::array::chunk::DataChunk;
use mojito_common::scalar::{Datum, Row};
use mojito_cypher::plan_context::PlanContext;
use mojito_cypher::session::{SessionContext, handle_query, parse_statement};
use mojito_exec::error::ExecError;
use mojito_exec::task::{ExecContext, create_task};
use mojito_parser::ast;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::error::Error;
use crate::result::ResultHandle;

#[derive(Debug)]
pub struct Session {
    pub catalog: Arc<Catalog>,
    pub exec_ctx: Arc<ExecContext>,
}

impl Session {
    pub fn new(catalog: Arc<Catalog>, exec_ctx: Arc<ExecContext>) -> Self {
        Self { catalog, exec_ctx }
    }
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
        _params: HashMap<String, Datum>,
    ) -> Result<Box<dyn ResultHandle>, Error> {
        let ast = parse_statement(&query)?;
        match ast {
            ast::Statement::Query(regular_query) => self.handle_query(&regular_query).await,
        }
    }

    async fn handle_query(self: &Arc<Self>, query: &ast::RegularQuery) -> Result<Box<dyn ResultHandle>, Error> {
        let plan = handle_query(self.clone(), query)?;
        // execute query
        let query_id = uuid::Uuid::new_v4().to_string().into();
        let handle = create_task(&self.exec_ctx, query_id, plan).await?;
        let bridge = TaskHandleBridge::new(handle.columns.clone(), handle.recv);
        Ok(Box::new(bridge))
    }
}

pub struct TaskHandleBridge {
    pub stream: BoxStream<'static, Result<Row, Error>>,
    columns: Vec<String>,
}

impl TaskHandleBridge {
    pub fn new(columns: Vec<String>, mut data: UnboundedReceiver<Result<DataChunk, ExecError>>) -> Self {
        let s = Box::pin(stream! {
            while let Some(msg) = data.recv().await {
                match msg {
                    Ok(chunk) => {
                        for i in 0..chunk.len() {
                            let row = chunk.get_row_by_idx(i);
                            yield Ok(row);
                        }
                    }
                    Err(e) =>{
                        yield Err(e.into())
                    }
                }
            }
        });

        Self { stream: s, columns }
    }
}

impl Stream for TaskHandleBridge {
    type Item = Result<Row, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

impl ResultHandle for TaskHandleBridge {
    fn columns(&self) -> Vec<String> {
        self.columns.clone()
    }
}

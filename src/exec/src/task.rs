use std::sync::Arc;

use futures::StreamExt;
use indexmap::IndexMap;
use mojito_catalog::Catalog;
use mojito_common::array::chunk::DataChunk;
use mojito_common::schema::Schema;
use mojito_common::variable::VariableName;
use mojito_common::{TokenId, TokenKind};
use mojito_cypher::planner::RootPlan;
use mojito_expr::error::EvalError;
use mojito_expr::impl_::EvalCtx;
use mojito_storage::graph::GraphStore;
use mojito_storage::transaction::Transaction;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::builder::{ExecutorBuildContext, build_executor};
use crate::error::ExecError;
use crate::executor::BoxedExecutor;

// global execution context
pub struct ExecContext {
    catalog: Arc<Catalog>,
    // global resources here
    store: Arc<GraphStore>,
}

impl ExecContext {
    pub fn catalog(&self) -> &Arc<Catalog> {
        &self.catalog
    }

    pub fn store(&self) -> &Arc<GraphStore> {
        &self.store
    }
}

pub struct EvalCtxImpl {
    pub catalog: Arc<Catalog>,
}

impl EvalCtx for EvalCtxImpl {
    fn get_or_create_token(&self, token: &str, kind: TokenKind) -> Result<TokenId, EvalError> {
        self.catalog
            .get_or_create_token(token, kind)
            .map_err(|e| EvalError::GetOrCreateTokenError(e.to_string()))
    }
}

/// Task execution context contains the global resources needed by the task execution
pub struct TaskExecContext {
    exec_ctx: Arc<ExecContext>,
    // task specific context here
    // TODO(pgao): maybe we should transaction also into catalog api?
    tx: Arc<dyn Transaction>,
}

impl TaskExecContext {
    pub fn catalog(&self) -> &Arc<Catalog> {
        self.exec_ctx.catalog()
    }

    pub fn store(&self) -> &Arc<GraphStore> {
        self.exec_ctx.store()
    }

    pub fn tx(&self) -> &Arc<dyn Transaction> {
        &self.tx
    }

    pub fn derive_eval_ctx(&self) -> EvalCtxImpl {
        EvalCtxImpl {
            catalog: self.exec_ctx.catalog().clone(),
        }
    }
}

// TODO(pgao): task manager

/// receiver side of task
pub struct TaskHandle {
    pub query_id: Arc<str>,
    pub schema: Schema,
    pub output_names: IndexMap<VariableName, String>,

    // pub task_id: Arc<str>,
    recv: UnboundedReceiver<Result<DataChunk, ExecError>>,
    // output channnel for task results
}

impl TaskHandle {
    pub async fn cancel(&self) {
        todo!()
    }

    // fetch next data chunk result
    pub async fn next(&mut self) -> Result<Option<DataChunk>, ExecError> {
        todo!()
    }
}

/// create task and spawn running task execution
pub async fn create_task(ectx: &Arc<ExecContext>, query_id: Arc<str>, plan: RootPlan) -> Result<TaskHandle, ExecError> {
    let tx = ectx.store.transaction();
    let task_context = Arc::new(TaskExecContext {
        exec_ctx: ectx.clone(),
        tx,
    });

    // compile to executor
    let mut bctx = ExecutorBuildContext::new(task_context.clone());
    let root_executor = build_executor(&mut bctx, &plan)?;

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let handle = TaskHandle {
        query_id,
        recv: rx,
        schema: root_executor.schema().clone(),
        output_names: plan.names,
    };

    let runner = TaskRunner {
        ctx: task_context,
        tx,
        root_executor,
    };

    runner.start();

    Ok(handle)
}

pub struct TaskRunner {
    ctx: Arc<TaskExecContext>,
    tx: UnboundedSender<Result<DataChunk, ExecError>>,
    root_executor: BoxedExecutor,
    // TODO(pgao): cancellation token
}

impl TaskRunner {
    pub fn start(self) {
        // spawn task and drive task to finish
        let TaskRunner { ctx, tx, root_executor } = self;
        let stream = root_executor.build_stream(ctx).unwrap();
        let mut stream = stream.boxed();
        tokio::spawn(async move {
            // TODO(pgao): cancellation token
            while let Some(chunk) = stream.next().await {
                tx.send(chunk).unwrap();
            }
        });
    }
}

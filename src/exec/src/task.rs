use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_common::array::chunk::DataChunk;
use mojito_cypher::planner::RootPlan;
use mojito_storage::graph::GraphStore;
use mojito_storage::transaction::Transaction;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::error::ExecError;
use crate::executor::BoxedExecutor;

// global execution context
pub struct ExecContext {
    catalog: Arc<Catalog>,
    // global resources here
    store: Arc<GraphStore>,
}

/// Task execution context contains the global resources needed by the task execution
pub struct TaskExecContext {
    exec_ctx: Arc<ExecContext>,
    // task specific context here
    tx: Arc<dyn Transaction>,
}

impl TaskExecContext {
    pub fn store(&self) -> &Arc<GraphStore> {
        &self.exec_ctx.store
    }

    pub fn tx(&self) -> &Arc<dyn Transaction> {
        &self.tx
    }
}

/// receiver side of task
pub struct TaskHandle {
    pub query_id: Arc<str>,
    pub task_id: Arc<str>,
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
pub async fn create_task(
    _ectx: &Arc<ExecContext>,
    _query_id: Arc<str>,
    _plan: RootPlan,
) -> Result<TaskHandle, ExecError> {
    // compile to executor

    // spawn task runner and return task handle
    todo!()
}

pub struct TaskRunner {
    ctx: Arc<TaskExecContext>,
    tx: UnboundedSender<Result<DataChunk, ExecError>>,
    root_executor: BoxedExecutor,
}

impl TaskRunner {
    pub fn start(self) {
        // tokio spawn the executor work
    }
}

use std::sync::Arc;

use mojito_common::array::chunk::DataChunk;
use mojito_cypher::planner::RootPlan;
use mojito_storage::graph::GraphStore;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::error::ExecError;
use crate::executor::BoxedExecutor;

// global execution context
pub struct ExecContext {
    // global resources here
    store: Arc<GraphStore>,
}

/// Task execution context contains the global resources needed by the task execution
pub struct TaskExecContext {
    exec_ctx: Arc<ExecContext>,
    // task specific context here
}

impl TaskExecContext {
    pub fn store(&self) -> &Arc<GraphStore> {
        &self.exec_ctx.store
    }
}

/// receiver side of task
pub struct TaskHandle {
    pub query_id: Arc<str>,
    pub task_id: Arc<str>,
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
pub async fn create_task(_ectx: &Arc<ExecContext>, _query_id: Arc<str>, _plan: RootPlan) -> Result<TaskHandle, ExecError> {
    // compile to executor

    // spawn task runner and return task handle
    todo!()
}

pub struct TaskRunner {
    ctx: Arc<TaskExecContext>,
    tx: UnboundedReceiver<Result<DataChunk, ExecError>>,
    root_executor: BoxedExecutor,
}

impl TaskRunner {
    pub fn start(self) {
        // tokio spawn the executor work
    }
}

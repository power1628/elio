use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use mojito_common::array::chunk::DataChunk;
use mojito_cypher::plan_node::PlanExpr;

use crate::error::ExecError;
use crate::task::TaskExecContext;

pub mod create_node;
pub mod filter;
pub mod relscan;

pub type DataChunkStream = Pin<Box<dyn Stream<Item = Result<DataChunk, ExecError>> + Send>>;

pub trait Executor: Send + Sync {
    /// Build the output data chunk stream
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError>;
}

pub type BoxedExecutor = Box<dyn Executor>;

pub struct ExecutorBuilder {
    ctx: Arc<TaskExecContext>,
}

impl ExecutorBuilder {
    pub fn build(&self, _plan: &PlanExpr) -> Result<BoxedExecutor, ExecError> {
        todo!()
    }
}

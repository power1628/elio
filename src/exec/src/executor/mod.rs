use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use mojito_common::array::chunk::DataChunk;
use mojito_cypher::plan_node::PlanExpr;

use crate::error::ExecError;
use crate::task::TaskExecContext;

pub mod filter;
pub mod relscan;

pub trait DataChunkStream: Stream<Item = Result<DataChunk, ExecError>> {}

pub type SendableDataChunkStream = Pin<Box<dyn DataChunkStream + Send>>;

pub trait Executor: Send + Sync {
    /// Build the output data chunk stream
    fn build(self, ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError>;
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

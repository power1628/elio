use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use mojito_common::array::chunk::DataChunk;

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

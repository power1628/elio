use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use mojito_common::array::chunk::DataChunk;
use mojito_common::schema::Schema;

use crate::error::ExecError;
use crate::task::TaskExecContext;

pub mod create_node;
pub mod filter;
pub mod project;
pub mod relscan;
pub mod unit;

pub type DataChunkStream = Pin<Box<dyn Stream<Item = Result<DataChunk, ExecError>> + Send>>;

pub trait Executor: Send + Sync + std::fmt::Debug {
    /// Build the output data chunk stream
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError>;

    fn schema(&self) -> &Schema;

    fn boxed(self) -> BoxedExecutor
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub type BoxedExecutor = Box<dyn Executor>;

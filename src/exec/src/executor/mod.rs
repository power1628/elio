use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use mojito_common::array::chunk::DataChunk;
use mojito_common::schema::Schema;
use mojito_expr::impl_::BoxedExpression;
use tracing;

use crate::error::ExecError;
use crate::task::TaskExecContext;

pub mod all_node_scan;
pub mod constraint;
pub mod create_node;
pub mod create_rel;
pub mod expand;
pub mod filter;
pub mod produce_result;
pub mod project;
pub mod relscan;
pub mod unit;
pub mod var_expand;

pub type DataChunkStream = Pin<Box<dyn Stream<Item = Result<DataChunk, ExecError>> + Send>>;

// maybe chunk size should be put to exec ctx as an configuration
pub const CHUNK_SIZE: usize = 4096;

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

    fn name(&self) -> &'static str;
}

pub type BoxedExecutor = Box<dyn Executor>;

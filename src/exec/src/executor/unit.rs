use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::schema::Schema;

use super::*;
use crate::executor::Executor;

#[derive(Debug, Default)]
pub struct UnitExecutor {
    schema: Schema,
}

impl Executor for UnitExecutor {
    fn build_stream(self: Box<Self>, _ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            // return one empty row
            yield DataChunk::unit();
        }
        .boxed();

        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

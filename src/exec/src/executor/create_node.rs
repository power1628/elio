use async_stream::try_stream;
use futures::StreamExt;

use super::*;

// Input should have the schema of [List<u16>, PropertyMap]
pub struct CreateNodeExectuor {
    input: BoxedExecutor,
    // create node info
    label_column: usize,
    prop_column: usize,
}

impl Executor for CreateNodeExectuor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        self.execute(ctx)
    }
}

impl CreateNodeExectuor {
    fn execute(self: Box<CreateNodeExectuor>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let mut input_stream = self.input.build_stream(ctx.clone())?;
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                let label = chunk.column(self.label_column);
                let prop = chunk.column(self.prop_column);
                let ids = ctx.tx().node_create(label.as_list().unwrap(), prop.as_property_map().unwrap())?;
                chunk.add_column(ids.into());
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }
}

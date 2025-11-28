use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::LabelId;
use mojito_expr::impl_::{BoxedExpression, EvalCtx};

use super::*;

// Input should have the schema of [List<u16>, PropertyMap]
pub struct CreateNodeExectuor {
    input: BoxedExecutor,
    labels: Vec<LabelId>,
    properties: BoxedExpression,
}

impl Executor for CreateNodeExectuor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        self.execute(ctx)
    }
}

impl CreateNodeExectuor {
    fn execute(self: Box<CreateNodeExectuor>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let eval_ctx = EvalCtx::new();

            let mut input_stream = self.input.build_stream(ctx.clone())?;
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                let prop = self.properties.eval_batch(&chunk, &eval_ctx)?;
                let ids = ctx.tx().node_create(&self.labels, prop.as_property_map().unwrap())?;
                // TODO add node column
                chunk.add_column(ids.into());
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }
}

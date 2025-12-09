use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::IrToken;
use mojito_expr::impl_::BoxedExpression;

use super::*;

// input: Schema
// output: Schema + Node
#[derive(Debug)]
pub struct CreateNodeExectuor {
    pub input: BoxedExecutor,
    pub labels: Vec<IrToken>,
    // the return type should be struct
    pub properties: BoxedExpression,
    pub schema: Arc<Schema>,
}

impl Executor for CreateNodeExectuor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        self.execute(ctx)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl CreateNodeExectuor {
    fn execute(self: Box<CreateNodeExectuor>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let eval_ctx = ctx.derive_eval_ctx();

            // create string to token mapping
            let mut labels = vec![];
            for label in self.labels.iter() {
                labels.push(label.name().to_string());
            }

            // execute the stream
            let mut input_stream = self.input.build_stream(ctx.clone())?;
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                let prop = self.properties.eval_batch(&chunk, &eval_ctx)?;
                let output= ctx.tx().node_create(&labels, &prop)?;
                chunk.add_column(Arc::new(output.into()));
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }
}

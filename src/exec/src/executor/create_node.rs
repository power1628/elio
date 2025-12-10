use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::IrToken;
use mojito_common::schema::Variable;
use mojito_expr::impl_::BoxedExpression;

use super::*;

// input: Schema
// output: Schema + Node
#[derive(Debug)]
pub struct CreateNodeExectuor {
    pub input: BoxedExecutor,
    pub schema: Arc<Schema>,
    pub items: Vec<CreateNodeItem>,
}

#[derive(Debug)]
pub struct CreateNodeItem {
    pub labels: Vec<IrToken>,
    // the return type should be struct
    pub properties: BoxedExpression,
    pub variable: Variable,
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
            let mut input_stream = self.input.build_stream(ctx.clone())?;

            let label_vec: Vec<Vec<String>> = self.items.iter().map(|item| item.labels.iter().map(|label| label.name().to_string()).collect()).collect();

            // execute the stream
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                // for each variable execute create node
                for (i, item) in self.items.iter().enumerate() {
                    let prop = item.properties.eval_batch(&chunk, &eval_ctx)?;
                    let output= ctx.tx().node_create(&label_vec[i], &prop)?;
                    chunk.add_column(Arc::new(output.into()));
                }
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }
}

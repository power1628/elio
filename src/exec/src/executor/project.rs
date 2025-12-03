use async_stream::try_stream;
use futures::StreamExt;
use mojito_expr::impl_::BoxedExpression;

use super::*;

#[derive(Debug)]
pub struct ProjectExecutor {
    pub(crate) input: BoxedExecutor,
    pub(crate) exprs: Vec<BoxedExpression>,
    pub(crate) schema: Arc<Schema>,
}

impl Executor for ProjectExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let eval_ctx = ctx.derive_eval_ctx();

            let mut input_stream = self.input.build_stream(ctx.clone())?;
            while let Some(chunk) = input_stream.next().await {
                let chunk = chunk?;
                let mut output = DataChunk::empty();
                for expr in self.exprs.iter() {
                    let column = expr.eval_batch(&chunk, &eval_ctx)?;
                    output.add_column(column);
                }
                yield output;
           }
        }
        .boxed();
        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

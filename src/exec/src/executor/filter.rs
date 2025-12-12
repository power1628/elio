use async_stream::try_stream;
use futures::StreamExt;

use super::*;
use crate::error::ExecError;
use crate::executor::Executor;

#[derive(Debug)]
pub struct FilterExecutor {
    input: BoxedExecutor,
    filter: BoxedExpression,
    schema: Arc<Schema>,
}

// TODO(pgao): compact data chunk before expand and others
impl Executor for FilterExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {

            let eval_ctx  = ctx.derive_eval_ctx();
            let input_stream = self.input.build_stream(ctx.clone())?;

            for await chunk in input_stream {
                let mut chunk = chunk?;
                let res = self.filter.eval_batch(&chunk, &eval_ctx)?;
                let bool_array = res.as_bool().expect("filter should result in bool array");
                let mask  = bool_array.to_filter_mask();
                let visibility =  chunk.visibility_mut();
                *visibility &= mask;
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

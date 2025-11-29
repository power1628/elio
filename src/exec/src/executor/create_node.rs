use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::IrToken;
use mojito_expr::impl_::{BoxedExpression, EvalCtx};

use super::*;

// Input should have the schema of [List<u16>, PropertyMap]
pub struct CreateNodeExectuor {
    input: BoxedExecutor,
    labels: Vec<IrToken>,
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
            let eval_ctx = ctx.derive_eval_ctx();

            // create string to token mapping
            let mut labels = vec![];
            for label in self.labels.iter() {
                match label{
                    IrToken::Resolved(label) => labels.push(*label),
                    IrToken::Unresolved(key) => {
                        let token_id = ctx.catalog().get_or_create_label_id(key)?;
                        labels.push(token_id);
                    },
                }
            }

            // execute the stream
            let mut input_stream = self.input.build_stream(ctx.clone())?;
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                let prop = self.properties.eval_batch(&chunk, &eval_ctx)?;
                let ids = ctx.tx().node_create(&labels, prop.as_property_map().unwrap())?;
                // TODO add node column
                chunk.add_column(ids.into());
                yield chunk;
            }
        }
        .boxed();
        Ok(stream)
    }
}

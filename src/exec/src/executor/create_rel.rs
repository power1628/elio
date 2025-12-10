use async_stream::try_stream;
use futures::StreamExt;
use mojito_common::array::ArrayImpl;
use mojito_expr::impl_::BoxedExpression;

use super::*;

// input: Schema
// output: Schema + Node
#[derive(Debug)]
pub struct CreateRelExectuor {
    pub input: BoxedExecutor,
    pub schema: Arc<Schema>,
    pub items: Vec<CreateRelItem>,
}

#[derive(Debug)]
pub struct CreateRelItem {
    pub rtype: Arc<str>,
    pub start: usize, // index of start node
    pub end: usize,   // index of end node
    // the return type should be struct
    pub properties: BoxedExpression,
}

impl Executor for CreateRelExectuor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {
            let eval_ctx = ctx.derive_eval_ctx();
            let mut input_stream = self.input.build_stream(ctx.clone())?;

            // execute the stream
            while let Some(chunk) = input_stream.next().await{
                let mut chunk = chunk?;
                // for each variable execute create node
                for (i, item) in self.items.iter().enumerate() {
                    let prop = item.properties.eval_batch(&chunk, &eval_ctx)?;
                    let prop = prop.as_ref().as_struct().ok_or_else(||
                        ExecError::type_mismatch(
                            format!("create rel item {}", i),
                            "struct",
                            prop.physical_type(),
                        ))?;

                    let start = chunk.column(item.start);
                    let end = chunk.column(item.end);

                    let output = match (start.as_ref(), end.as_ref()) {
                        (ArrayImpl::VirtualNode(start), ArrayImpl::VirtualNode(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop)?
                        }
                        (ArrayImpl::Node(start), ArrayImpl::Node(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop)?
                        }
                        (ArrayImpl::VirtualNode(start), ArrayImpl::Node(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop)?
                        }
                        (ArrayImpl::Node(start), ArrayImpl::VirtualNode(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop)?
                        }
                        _=> {
                            let err = Err(ExecError::type_mismatch(
                                format!("create rel item {}", i),
                                "node or virtual node",
                                start.physical_type(),
                            ));
                            err?;
                            continue;
                        }
                    };
                    chunk.add_column(Arc::new(output.into()));
                }
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

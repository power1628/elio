use async_stream::try_stream;
use elio_common::array::ArrayImpl;
use elio_expr::impl_::BoxedExpression;
use futures::StreamExt;

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
            let input_stream = self.input.build_stream(ctx.clone())?;

            // execute the stream
            for await chunk in input_stream {
                let chunk = chunk?;
                // TODO(pgao): do not eager compact the chunk
                let mut chunk = chunk.compact();
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
                            ctx.tx().relationship_create(&item.rtype, start, end, prop).map_err(|e| e.into())
                        }
                        (ArrayImpl::Node(start), ArrayImpl::Node(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop).map_err(|e| e.into())
                        }
                        (ArrayImpl::VirtualNode(start), ArrayImpl::Node(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop).map_err(|e| e.into())
                        }
                        (ArrayImpl::Node(start), ArrayImpl::VirtualNode(end)) => {
                            ctx.tx().relationship_create(&item.rtype, start, end, prop).map_err(|e| e.into())
                        }
                        (s, e) => {
                            let pt = if !matches!(s, ArrayImpl::Node(_) | ArrayImpl::VirtualNode(_)) {
                                s.physical_type()
                            } else {
                                e.physical_type()
                            };
                            Err(ExecError::type_mismatch(
                                format!("create rel item {} node", i),
                                "node or virtual node",
                                pt,
                            ))
                        }
                    }?;
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

    fn name(&self) -> &'static str {
        "CreateRel"
    }
}

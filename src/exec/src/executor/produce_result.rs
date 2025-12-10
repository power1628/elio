use async_stream::try_stream;
use futures::StreamExt;

use super::*;

// materialize virtual node to node
// change output column order
#[derive(Debug)]
pub struct ProduceResultExecutor {
    pub input: BoxedExecutor,
    // return_columns[i] = j means
    // the i-th column of the output is the j-th column of the input
    pub return_columns: Vec<usize>,
    pub schema: Arc<Schema>,
}

impl Executor for ProduceResultExecutor {
    fn build_stream(self: Box<Self>, ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        let stream = try_stream! {

            let mut input_stream = self.input.build_stream(ctx.clone())?;

            while let Some(input) = input_stream.next().await{
                let input = input?;

                let mut out_cols = vec![];
                for col_idx in self.return_columns.iter(){
                    let col = input.column(*col_idx);
                    // if col is type of virtual node, materialize it
                    if let Some(virtual_col) = col.as_virtual_node(){
                        let node_col= ctx.tx().materialize_node(virtual_col)?;
                        out_cols.push(Arc::new(node_col.into()));
                    } else {
                        out_cols.push(col.clone());
                    }
               }

               let output = DataChunk::new(out_cols);
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

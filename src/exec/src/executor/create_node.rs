use mojito_common::array::Array;

use super::*;

// Input should have the schema of [List<u16>, PropertyMap]
pub struct CreateNodeExectuor {
    input: Box<dyn Executor>,
}

impl CreateNodeExectuor {
    async fn execute(self, ctx: &Arc<TaskExecContext>) -> Result<DataChunk, ExecError> {
        let input = self.input.build(ctx)?;
        gen move {
            for chunk in input {
                let ids = ctx.tx().node_create(label, prop)?;
                let output = DataChunk::new(vec![ids], ids.len());
                yield Ok(output)
            }
        }
    }
}

impl Executor for CreateNodeExectuor {
    fn build(self, ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError> {
        self.execute(ctx)
    }
}

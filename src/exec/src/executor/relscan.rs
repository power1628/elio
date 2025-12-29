use super::*;

#[derive(Debug)]
pub struct AllRelScanExecutor {
    schema: Schema,
}

impl Executor for AllRelScanExecutor {
    fn build_stream(self: Box<Self>, _ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        todo!()
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn name(&self) -> &'static str {
        "AllRelScan"
    }
}

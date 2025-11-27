use super::*;
pub struct AllRelScanExecutor {}

impl Executor for AllRelScanExecutor {
    fn build_stream(self: Box<Self>, _ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        todo!()
    }
}

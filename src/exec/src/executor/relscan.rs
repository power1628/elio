use super::*;
pub struct AllRelScanExecutor {}

impl Executor for AllRelScanExecutor {
    fn build(self, _ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError> {
        todo!()
    }
}

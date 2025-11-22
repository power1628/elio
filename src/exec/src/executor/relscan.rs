use super::*;
pub struct AllRelScanExectutor {}

impl Executor for AllRelScanExectutor {
    fn build(self, ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError> {
        todo!()
    }
}

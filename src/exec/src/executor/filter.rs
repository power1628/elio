use mojito_expr::impl_::Expression;

use super::*;
use crate::error::ExecError;
use crate::executor::{Executor, SendableDataChunkStream};

pub struct FilterExecutor {
    input: Box<dyn Executor>,
    filter: Box<dyn Expression>,
}

impl FilterExecutor {
    async fn execute(
        self,
        input: SendableDataChunkStream,
        ctx: &Arc<TaskExecContext>,
    ) -> Result<SendableDataChunkStream, ExecError> {
        todo!()
    }
}

impl Executor for FilterExecutor {
    fn build(self, ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError> {
        let input = self.build(ctx)?;
        self.execute(input, ctx)
    }
}

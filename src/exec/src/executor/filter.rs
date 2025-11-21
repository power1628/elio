use mojito_expr::impl_::Expression;

use super::*;
use crate::error::ExecError;
use crate::executor::{Executor, SendableDataChunkStream};

pub struct FilterExecutor {
    input: Box<dyn Executor>,
    filter: Box<dyn Expression>,
}

impl Executor for FilterExecutor {
    fn build(self, _ctx: &Arc<TaskExecContext>) -> Result<SendableDataChunkStream, ExecError> {
        todo!()
    }
}

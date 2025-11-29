use mojito_expr::impl_::Expression;

use super::*;
use crate::error::ExecError;
use crate::executor::Executor;

#[derive(Debug)]
pub struct FilterExecutor {
    input: Box<dyn Executor>,
    filter: Box<dyn Expression>,
    schema: Schema,
}

impl Executor for FilterExecutor {
    fn build_stream(self: Box<Self>, _ctx: Arc<TaskExecContext>) -> Result<DataChunkStream, ExecError> {
        todo!()
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

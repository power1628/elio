
use crate::session::SessionContext;

#[derive(Debug, Clone)]
pub struct StmtContext<'a> {
    pub session: &'a SessionContext,
    // TODO(power): parameters
}

use mojito_catalog::Catalog;

use crate::session::SessionContext;

pub struct StmtContext<'a> {
    pub session: &'a SessionContext,
    // TODO(power): parameters
}

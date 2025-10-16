use crate::meta::{self, CatalogReader};

pub struct StmtContext<'a> {
    pub meta: &'a dyn CatalogReader,
    // TODO(power): parameters
}

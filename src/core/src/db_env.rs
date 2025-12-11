use std::path::Path;
use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_exec::task::ExecContext;
use mojito_storage::graph::GraphStore;

use crate::error::Error;
use crate::session::Session;

pub struct DbConfig {
    store_path: String,
}

impl DbConfig {
    pub fn with_db_path<P: AsRef<Path>>(db_path: P) -> Self {
        Self {
            store_path: db_path.as_ref().to_path_buf().into_os_string().into_string().unwrap(),
        }
    }
}

pub struct DbEnv {
    // TODO(pgao): QueryManager which manages query execution
    catalog: Arc<Catalog>,
    exec_ctx: Arc<ExecContext>,
}

impl DbEnv {
    pub fn open(config: &DbConfig) -> Result<Arc<DbEnv>, Error> {
        let store = Arc::new(GraphStore::open(&config.store_path)?);
        let catalog = Arc::new(Catalog::new(store.token_store().clone()));
        let exec_ctx = Arc::new(ExecContext::new(catalog.clone(), store.clone()));
        let me = Self { catalog, exec_ctx };
        Ok(Arc::new(me))
    }

    pub fn new_session(&self) -> Arc<Session> {
        Arc::new(Session::new(self.catalog.clone(), self.exec_ctx.clone()))
    }
}

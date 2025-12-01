use std::collections::HashMap;
use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_common::scalar::Datum;
use mojito_cypher::session::SessionContext;
use mojito_exec::task::ExecContext;
use mojito_storage::graph::GraphStore;

use crate::error::Error;
use crate::result::ResultHandle;
use crate::session::Session;

pub struct DbConfig {
    store_path: String,
}

pub struct DbEnv {
    // TODO(pgao): QueryManager which manages query execution
    catalog: Arc<Catalog>,
    exec_ctx: Arc<ExecContext>,
}

impl DbEnv {
    pub fn open(config: &DbConfig) -> Result<Arc<DbEnv>, Error> {
        let store = Arc::new(GraphStore::open(&config.store_path));
        let catalog = Arc::new(Catalog::new(store.token_store().clone()));
        let exec_ctx = Arc::new(ExecContext::new(catalog.clone(), store.clone()));
        let me = Self { catalog, exec_ctx };
        Ok(Arc::new(me))
    }

    pub fn new_session(&self) -> Arc<Session> {
        Arc::new(Session::new(self.catalog.clone(), self.exec_ctx.clone()))
    }
}

impl DbEnv {
    pub async fn execute(
        self: &Arc<Self>,
        stmt: String,
        params: HashMap<String, Datum>,
    ) -> Result<Box<dyn ResultHandle>, Error> {
        let session = self.new_session();
        session.execute(stmt, params).await
    }
}

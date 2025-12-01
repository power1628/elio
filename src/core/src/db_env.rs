use std::collections::HashMap;
use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_common::scalar::Datum;
use mojito_cypher::binder::BindContext;
use mojito_cypher::error::PlanError;
use mojito_cypher::session::SessionContext;
use mojito_exec::task::ExecContext;
use mojito_parser::parser;
use mojito_storage::graph::GraphStore;

use crate::error::Error;
use crate::result::ResultHandle;

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
}

impl DbEnv {
    pub fn execute(self: &Arc<Self>, stmt: String, _params: HashMap<String, Datum>) -> Result<ResultHandle, Error> {
        let sess_ctx = SessionContext {
            catalog: self.catalog.clone(),
        };
        // parse
        let ast = {
           parser::cypher_parser::statement(&stmt).map_err(PlanError::parse_error)?
        };


        // bind
        let ir = {
            BindContext::new(sctx)
        }

        // plan

        // exec

        todo!()
    }
}

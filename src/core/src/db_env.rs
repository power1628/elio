use std::collections::HashMap;
use std::sync::Arc;

use mojito_common::scalar::Datum;
use mojito_storage::graph::GraphStore;

use crate::error::Error;
use crate::result::ResultHandle;

pub struct DbConfig {
    store_path: String,
}

pub struct DbEnv {
    store: Arc<GraphStore>,
    // TODO(pgao): QueryManager which manages query execution
}

impl DbEnv {
    pub fn open(config: &DbConfig) -> Result<Arc<DbEnv>, Error> {
        let store = GraphStore::open(&config.store_path);
        let me = Self { store: Arc::new(store) };
        Ok(Arc::new(me))
    }
}

impl DbEnv {
    pub fn execute(self: &Arc<Self>, _query: String, _params: HashMap<String, Datum>) -> Result<ResultHandle, Error> {
        todo!()
    }
}

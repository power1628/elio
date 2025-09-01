use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mojito_common::value::Value;
use mojito_storage::graph_store::{GraphStore, GraphStoreConfig};

use crate::{
    error::Error,
    session::{Session, SessionId},
    transaction::ResultHandle,
};

pub struct DbConfig {
    store_config: GraphStoreConfig,
}

pub struct DbEnv {
    store: Arc<GraphStore>,
    // TODO(pgao): QueryManager which manages query execution
}

impl DbEnv {
    pub fn open(config: &DbConfig) -> Result<Arc<DbEnv>, Error> {
        let store = GraphStore::open(&config.store_config)?;
        let me = Self { store: Arc::new(store) };
        Ok(Arc::new(me))
    }
}

impl DbEnv {
    pub fn execute(self: &Arc<Self>, query: String, params: HashMap<String, Value>) -> Result<ResultHandle, Error> {
        todo!()
    }
}

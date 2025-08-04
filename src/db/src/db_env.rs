use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use mojito_storage::graph_store::{GraphStore, GraphStoreConfig};

use crate::{
    error::Error,
    session::{Session, SessionId},
};

pub struct DbConfig {
    store_config: GraphStoreConfig,
}

pub struct DbEnv {
    store: Arc<GraphStore>,
    sessions: Arc<Mutex<HashMap<SessionId, Arc<Session>>>>,
}

impl DbEnv {
    pub fn open(config: &DbConfig) -> Result<Arc<DbEnv>, Error> {
        let store = GraphStore::open(&config.store_config)?;
        let me = Self {
            store: Arc::new(store),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        };
        Ok(Arc::new(me))
    }
}

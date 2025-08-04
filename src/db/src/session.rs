use std::{collections::HashMap, sync::Arc};

use mojito_common::value::Value;

use crate::{
    db_env::DbEnv,
    error::Error,
    transaction::{ResultHandle, Transaction},
};

pub type SessionId = String;

pub struct Session {
    env: Arc<DbEnv>,
    id: SessionId,
}

impl Session {
    pub fn new(env: Arc<DbEnv>, id: SessionId) -> Arc<Self> {
        Self { env, id }.into()
    }
}

impl Session {
    // TODO(pgao): impl
    pub fn begin_transaction(self: Arc<Session>) -> Arc<Transaction> {
        todo!()
    }

    /// Execute query in implicit transaction
    pub fn execute(
        self: Arc<Session>,
        query: String,
        params: HashMap<String, Value>,
    ) -> Result<Box<dyn ResultHandle>, Error> {
        todo!()
    }
}

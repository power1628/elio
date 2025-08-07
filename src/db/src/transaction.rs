use std::{collections::HashMap, sync::Arc};

use mojito_common::value::Value;

use crate::{db_env::DbEnv, error::Error};

// TODO(power): impl transaction
pub struct Transaction {
    env: Arc<DbEnv>,
}

impl Transaction {
    pub fn execute(&self, query: String, params: HashMap<String, Value>) -> Result<Box<dyn ResultHandle>, Error> {
        // parse query
        // bind query
        // execute query
        todo!()
    }
}

// ResultHandle also act as an result iterator.
pub trait ResultHandle: Iterator<Item = Vec<(String, Value)>> {
    // return the result column names
    fn columns(&self) -> Vec<String>;
}

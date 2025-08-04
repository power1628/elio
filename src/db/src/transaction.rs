use std::collections::HashMap;

use mojito_common::value::Value;

use crate::error::Error;

// TODO(power): impl transaction
pub struct Transaction {}

impl Transaction {
    pub fn execute(&self, query: String, params: HashMap<String, Value>) -> Result<Box<dyn ResultHandle>, Error> {
        todo!()
    }
}

// ResultHandle also act as an result iterator.
pub trait ResultHandle: Iterator<Item = Vec<(String, Value)>> {
    // return the result column names
    fn columns(&self) -> Vec<String>;
}

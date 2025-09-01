use std::{collections::HashMap, sync::Arc};

use mojito_common::value::Value;

use crate::{db_env::DbEnv, error::Error};

pub enum QueryKind {
    // read only query
    Read,
    // update or create data, also produce results
    ReadWrite,
    // write only query, update or create data, without produce any results
    Write,
    // schema change query, update schema but does not change data nor produces any results
    SchemaWrite,
    // dbms query
    Dbms,
}

pub enum QueryExecutionKind {
    Query(QueryType),
    Profile,
    Explain,
}

/// Query Result Handle.
/// Result handle contains three parts:
/// 1. meta data: query static info(static, resolved after planning):
///     - query execution kind
///     - columns
/// 2. meta data: query execution info(dynamic, resolved after execution):
///     - query statistics
///     - execution plan
///     - gql status object
///     - notifications
/// 3. data: query result data(if any)
/// 4. listeners
///     - on query failed
///     - on query finished
///     - ...
/// ResultHandle communicate with execution engine with QueryExecutionHandle object
pub struct ResultHandle {}

impl ResultHandle {
    pub fn execution_kind(&self) -> QueryExecutionKind {
        todo!()
    }

    // result column names
    pub fn column(&self) -> Vec<String> {
        todo!()
    }
}

// ResultHandle also act as an result iterator.
pub trait ResultHandle: Iterator<Item = Vec<(String, Value)>> {
    // return the result column names
    fn columns(&self) -> Vec<String>;
}

use mojito_parser::ast::{self};

use crate::binder::{BindContext, pattern::QueryGraph};

pub struct BoundQuery {
    pub queries: Vec<BoundSingleQuery>,
    pub union_all: bool,
}

pub struct BoundSingleQuery {
    pub query_graph: QueryGraph,
}

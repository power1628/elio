use crate::ir::{order::OrderingChoice, query_graph::QueryGraph};

pub struct IrQuery {
    pub queries: Vec<IrSingleQuery>,
    pub union_all: bool,
}

pub struct IrSingleQuery {
    pub query_graph: QueryGraph,
    // TODO(pgao): the interesting_order may be derived here.
    pub interesting_order: OrderingChoice,
    // TODO(pgao): other
}

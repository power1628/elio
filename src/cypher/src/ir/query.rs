use indexmap::IndexMap;

use crate::{
    ir::{horizon::QueryHorizon, query_graph::QueryGraph},
    variable::VariableName,
};

pub struct IrQueryRoot {
    pub inner: IrQuery,
    // mapping from variable name to output names
    pub names: IndexMap<VariableName, String>,
}

pub struct IrQuery {
    pub queries: Vec<IrSingleQuery>,
    pub union_all: bool,
}

#[derive(Default)]
pub struct IrSingleQuery {
    pub parts: Vec<IrSingleQueryPart>,
    // pub query_graph: QueryGraph,
    // pub horizon: QueryHorizon,
    // pub tail: Option<Box<IrSingleQuery>>,
    // TODO(pgao): the interesting_order may be derived here.
    // pub interesting_order: OrderingChoice,
}

impl IrSingleQuery {
    pub fn empty() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct IrSingleQueryPart {
    pub query_graph: QueryGraph,
    pub horizon: QueryHorizon,
}

impl IrSingleQueryPart {
    pub fn empty() -> Self {
        Self::default()
    }
}

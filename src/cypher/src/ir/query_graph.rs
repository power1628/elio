use std::collections::HashSet;

use crate::{
    expr::FilterExprs,
    ir::{
        mutating_pattern::MutatingPattern,
        node_connection::{QuantifiedPathPattern, RelPattern},
        path_pattern::SelectivePathPattern,
    },
    variable::Variable,
};

#[derive(Default)]
pub struct QueryGraph {
    // node patterns
    nodes: HashSet<Variable>,
    // node connections
    rels: HashSet<RelPattern>,
    quantified_paths: HashSet<QuantifiedPathPattern>,
    // selective path patterns
    selective_paths: HashSet<SelectivePathPattern>,
    // predicate, i.e. post filter
    filter: FilterExprs,
    // optional matches
    optional_matches: Vec<QueryGraph>,
    // mutating patterns
    mutating_patterns: Vec<MutatingPattern>,
    // arguments
    arguments: HashSet<Variable>,
    // path projections
}

impl QueryGraph {
    pub fn empty() -> Self {
        Self::default()
    }
}

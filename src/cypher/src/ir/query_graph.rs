use std::collections::HashSet;

use crate::{
    expr::FilterExprs,
    ir::{
        mutating_pattern::MutatingPattern,
        node_connection::{QuantifiedPathPattern, RelPattern, SelectivePathPattern},
    },
    variable::Variable,
};

pub struct QueryGraph {
    // relationship patterns
    rels: HashSet<RelPattern>,
    quantified_paths: HashSet<QuantifiedPathPattern>,
    selective_paths: HashSet<SelectivePathPattern>,
    // node patterns
    nodes: HashSet<Variable>,
    // predicate
    filter: FilterExprs,
    // optional matches
    optional_matches: Vec<QueryGraph>,
    // mutating patterns
    mutating_patterns: Vec<MutatingPattern>,
    // arguments
    arguments: HashSet<Variable>,
}

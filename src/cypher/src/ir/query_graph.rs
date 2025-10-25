use indexmap::IndexSet;

use crate::{
    binder::pattern::PathPatternWithExtra,
    expr::FilterExprs,
    ir::{
        mutating_pattern::MutatingPattern,
        node_connection::{ExhaustiveNodeConnection, QuantifiedPathPattern, RelPattern},
        path_pattern::{PathPattern, SelectivePathPattern, SingleNode},
    },
    variable::{Variable, VariableName},
};

#[derive(Default)]
pub struct QueryGraph {
    // node patterns
    nodes: IndexSet<VariableName>,
    // node connections
    rels: IndexSet<RelPattern>,
    quantified_paths: Vec<QuantifiedPathPattern>,
    // selective path patterns
    selective_paths: Vec<SelectivePathPattern>,
    // predicate, i.e. post filter
    filter: FilterExprs,
    // optional matches
    optional_matches: Vec<QueryGraph>,
    // mutating patterns
    mutating_patterns: Vec<MutatingPattern>,
    // imported variables as query graph inputs
    imported: IndexSet<Variable>,
    // outer referenced variables,
    outer: IndexSet<VariableName>,
    // path projection
}

impl QueryGraph {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn add_path_pattern(&mut self, path: &PathPatternWithExtra) {
        let PathPatternWithExtra { pattern, extra } = path;
        match pattern {
            PathPattern::SingleNode(SingleNode { variable: node }) => self.add_node(node),
            PathPattern::NodeConnections(node_connections) => node_connections.connections.iter().for_each(|nc| {
                self.add_node_connection(nc);
            }),
            PathPattern::SelectivePathPattern(selective_path_pattern) => {
                self.add_selective_path(selective_path_pattern);
            }
        }
        // add extra
        self.outer.extend(extra.outer.clone().into_iter());
        self.filter = self.filter.clone().and(extra.post_filter.clone());
    }

    pub fn add_node(&mut self, node: &VariableName) {
        self.nodes.insert(node.clone());
    }

    // add relationship endpoint nodes and the relationship itself
    pub fn add_rel(&mut self, rel: &RelPattern) {
        rel.endpoint_nodes().iter().for_each(|x| self.add_node(x));
        self.rels.insert(rel.clone());
    }

    pub fn add_quantifled_path(&mut self, qpp: &QuantifiedPathPattern) {
        qpp.endpoint_nodes().iter().for_each(|x| self.add_node(x));
        self.quantified_paths.push(qpp.clone());
    }

    pub fn add_selective_path(&mut self, spp: &SelectivePathPattern) {
        spp.endpoint_nodes().iter().for_each(|x| self.add_node(x));
        self.selective_paths.push(spp.clone());
    }

    pub fn add_node_connection(&mut self, conn: &ExhaustiveNodeConnection) {
        match conn {
            ExhaustiveNodeConnection::RelPattern(rel_pattern) => self.add_rel(rel_pattern),
            ExhaustiveNodeConnection::QuantifiedPathPattern(quantified_path_pattern) => {
                self.add_quantifled_path(quantified_path_pattern)
            }
        }
    }
}

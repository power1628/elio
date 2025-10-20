use std::collections::HashSet;

use mojito_common::RelationshipType;
use mojito_parser::ast::SemanticDirection;

use crate::{expr::FilterExprs, variable::Variable};

pub struct RelPattern {
    variable: Variable,
    endpoints: (Variable, Variable),
    dir: SemanticDirection,
    types: Vec<RelationshipType>,
    length: PatternLength,
}

pub struct NodeBinding {
    pub inner: Variable,
    pub outer: Variable,
}
pub struct QuantifiedPathPattern {
    left_binding: NodeBinding,
    right_binding: NodeBinding,
    rels: Vec<RelPattern>,
    repetition: Repetition,
    node_grouping: HashSet<VariableGrouping>,
    rel_grouping: HashSet<VariableGrouping>,
}

pub enum ExhaustiveNodeConnection {
    RelPattern(RelPattern),
    QuantifiedPathPattern(QuantifiedPathPattern),
}

/// Path pattern of length 1 or more
pub struct NodeConnections {
    connections: Vec<ExhaustiveNodeConnection>,
}

pub enum Selector {
    /// Any k paths
    AnyK(i64),
    // shortest k paths
    ShortestK(i64),
}

pub struct SelectivePathPattern {
    path_pattern: NodeConnections,
    filter: FilterExprs,
    selector: Selector,
}

pub enum PatternLength {
    Simple,
    Var { min: i64, max: Option<i64> },
}

pub struct Repetition {
    pub min: i64,
    // None means infinite
    pub max: Option<i64>,
}

pub struct VariableGrouping {
    pub singleton: Variable,
    pub group: Variable,
}

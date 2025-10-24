use std::collections::HashSet;

use mojito_parser::ast::SemanticDirection;

use crate::{expr::IrToken, variable::VariableName};

pub struct RelPattern {
    pub variable: VariableName,
    pub endpoints: (VariableName, VariableName),
    pub dir: SemanticDirection,
    pub types: Vec<IrToken>,
    pub length: PatternLength,
}

pub struct NodeBinding {
    pub inner: VariableName,
    pub outer: VariableName,
}
pub struct QuantifiedPathPattern {
    pub left_binding: NodeBinding,
    pub right_binding: NodeBinding,
    pub rels: Vec<RelPattern>,
    pub repetition: Repetition,
    pub node_grouping: HashSet<VariableGrouping>,
    pub rel_grouping: HashSet<VariableGrouping>,
}

pub enum ExhaustiveNodeConnection {
    RelPattern(RelPattern),
    QuantifiedPathPattern(QuantifiedPathPattern),
}

pub enum Selector {
    /// Any k paths
    AnyK(i64),
    // shortest k paths
    ShortestK(i64),
    // TODO(pgao): Shortest K GROUPS
    ShortestKGroup(i64),
}

pub enum PatternLength {
    Simple,
    Var { min: i64, max: Option<i64> },
}

pub struct Repetition {
    pub min: i64,
    // None means infinite
    // inclusive
    pub max: Option<i64>,
}

pub struct VariableGrouping {
    pub singleton: VariableName,
    pub group: VariableName,
}

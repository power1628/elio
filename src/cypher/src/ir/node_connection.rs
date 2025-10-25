use indexmap::IndexSet;
use mojito_parser::ast::SemanticDirection;

use crate::{
    expr::{FilterExprs, IrToken},
    variable::VariableName,
};

#[derive(Clone)]
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
    // this filter works when expanding each hop
    // pre-filter
    pub filter: FilterExprs,
    pub repetition: Repetition,
    pub node_grouping: IndexSet<VariableGrouping>,
    pub rel_grouping: IndexSet<VariableGrouping>,
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

#[derive(Copy, Clone)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct VariableGrouping {
    pub singleton: VariableName,
    pub group: VariableName,
}

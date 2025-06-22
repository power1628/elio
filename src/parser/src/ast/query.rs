use std::ops::Range;

use crate::ast::{Expr, LabelExpr};

pub struct RegularQuery {
    pub queries: Vec<SingleQuery>,
    pub union_all: bool,
}

pub struct SingleQuery {
    pub clauses: Vec<Clause>,
}

pub enum Clause {
    Create(CreateClause),
}

pub struct CreateClause {
    // pattern: Pattern,
    pub pattern: Vec<PatternPart>,
}

pub struct PatternPart {
    pub shortest_kind: Option<ShortestKind>,
    pub nodes: Vec<NodePattern>,
    pub relationships: Vec<RelationshipPattern>,
    pub variable: Option<String>, // named pattern part or not
}

pub enum ShortestKind {
    Single, // single shortest path
    All,    // all shortest path
}

pub struct NodePattern {
    pub variable: Option<String>,
    pub label_expr: Option<LabelExpr>,
    pub properties: Option<Expr>,
    pub predicate: Option<Expr>,
}

pub struct RelationshipPattern {
    pub variable: Option<String>,
    pub label_expr: Option<LabelExpr>,
    pub properties: Option<Expr>,
    pub predicate: Option<Expr>,
    // (a)-[]->(b): None, no length
    // (a)-[*]->(b): Some(None), any length
    // (a)-[*1..3]->(b): Some(Some(1, 3)), min..max
    // (a)-[*1]->(b): Some(Some(1, 1)), length
    // (a)[*3..]->(b): Some(Some(3, None)), min..inf
    pub length: Option<Option<Range<usize>>>,
    pub direction: SemanticDirection,
}

pub enum SemanticDirection {
    Outgoing, // ->
    Incoming, // <-
    Both,     // -
}

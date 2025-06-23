use std::ops::Range;

use derive_more::Display;

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

#[derive(Debug)]
pub struct PatternPart {
    pub shortest_kind: Option<ShortestKind>,
    pub nodes: Vec<NodePattern>,
    pub relationships: Vec<RelationshipPattern>,
    pub variable: Option<String>, // named pattern part or not
}

impl PatternPart {
    pub fn new_anonymous(nodes: Vec<NodePattern>, rels: Vec<RelationshipPattern>) -> Self {
        Self {
            shortest_kind: None,
            nodes,
            relationships: rels,
            variable: None,
        }
    }

    pub fn new_named(variable: String, nodes: Vec<NodePattern>, rels: Vec<RelationshipPattern>) -> Self {
        Self {
            shortest_kind: None,
            nodes,
            relationships: rels,
            variable: Some(variable),
        }
    }
}

impl std::fmt::Display for PatternPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(variable) = &self.variable {
            write!(f, "{} = ", variable)?;
        }
        let mut nodes_iter = self.nodes.iter();
        let rels_iter = self.relationships.iter();
        write!(f, "{}", nodes_iter.next().unwrap())?;
        for (node, rel) in nodes_iter.zip(rels_iter) {
            write!(f, "{}", rel)?;
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ShortestKind {
    Single, // single shortest path
    All,    // all shortest path
}

#[derive(Debug, Display)]
#[display("({}{}{})", variable.as_ref().map(|x| x.to_string()).unwrap_or_default(),
    label_expr.as_ref().map(|x| format!(":{}", x)).unwrap_or_default(),
    properties.as_ref().map(|x| x.to_string()).unwrap_or_default())]
pub struct NodePattern {
    pub variable: Option<String>,
    pub label_expr: Option<LabelExpr>,
    pub properties: Option<Expr>,
    pub predicate: Option<Expr>,
}

#[derive(Default, Debug)]
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

impl RelationshipPattern {
    pub fn new() -> Self {
        Self {
            variable: None,
            label_expr: None,
            properties: None,
            predicate: None,
            length: None,
            direction: SemanticDirection::Both,
        }
    }
}

impl std::fmt::Display for RelationshipPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            SemanticDirection::Outgoing => write!(f, "-[")?,
            SemanticDirection::Incoming => write!(f, "<-[")?,
            SemanticDirection::Both => { write!(f, "-[") }?,
        };
        write!(
            f,
            "{}",
            self.variable.as_ref().map(|x| x.to_string()).unwrap_or_default()
        )?;
        write!(
            f,
            "{}",
            self.label_expr.as_ref().map(|x| format!(":{}", x)).unwrap_or_default()
        )?;
        write!(
            f,
            "{}",
            self.properties.as_ref().map(|x| x.to_string()).unwrap_or_default()
        )?;
        // TODO(length)

        match self.direction {
            SemanticDirection::Outgoing => write!(f, "]->"),
            SemanticDirection::Incoming => write!(f, "]-"),
            SemanticDirection::Both => write!(f, "]-"),
        }
    }
}

#[derive(Default, Debug)]
pub enum SemanticDirection {
    #[default]
    Outgoing, // ->
    Incoming, // <-
    Both,     // -
}

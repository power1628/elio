use std::ops::Range;

use derive_more::Display;
use itertools::{self, Itertools};

use crate::ast::{Expr, LabelExpr};

#[derive(Debug, Display)]
#[display("{}", patterns.iter().map(|x| x.to_string()).join(", "))]
pub struct MatchPattern {
    pub patterns: Vec<PatternPartWithSelector>,
}

#[derive(Debug, Display)]
#[display("{}", patterns.iter().map(|x| x.to_string()).join(", "))]
pub struct UpdatePattern {
    pub patterns: Vec<PatternPart>,
}

#[derive(Default, Debug, Display)]
#[display("{}", _0)]
pub enum Selector {
    #[default]
    #[display("")]
    AllPaths, // ALL PATHS
    #[display("ANY {} PATHS", _0)]
    AnyPath(u32), // ANY <count> PATHS
    #[display("SHORTEST {} PATHS", _0)]
    AnyShortestPath(u32), // SHORTEST <count> PATHS
    #[display("ALL SHORTEST PATHS")]
    AllShortest, // ALL SHORTEST PATHS
    #[display("SHORTEST {} PATH GROUPS", _0)]
    ShortestGroup(u32), // SHORTEST <count> PATH GROUPS
}

impl Selector {
    pub fn is_selective(&self) -> bool {
        !matches!(self, Self::AllPaths)
    }

    pub fn is_counted(&self) -> bool {
        matches!(
            self,
            Self::AnyPath(_) | Self::AnyShortestPath(_) | Self::ShortestGroup(_)
        )
    }
}

#[derive(Debug, Display)]
#[display("{} {}", selector, part)]
pub struct PatternPartWithSelector {
    pub selector: Selector,
    pub part: PatternPart,
}

#[derive(Debug)]
pub struct PatternPart {
    pub name: Option<String>,           // pattern part with name
    pub shortest: Option<ShortestKind>, // not, single, all shortest
    pub factors: Vec<PathFactor>,       // must start with simple and ends with simple
}

impl PatternPart {
    pub fn is_anonymous(&self) -> bool {
        self.name.is_none()
    }
}

impl std::fmt::Display for PatternPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = self.name.as_ref() {
            write!(f, "{} = ", path)?;
        }
        if let Some(ref shortest) = self.shortest {
            write!(f, "{}", shortest)?;
        }
        write!(
            f,
            "{}",
            self.factors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("")
        )?;
        Ok(())
    }
}
#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum ShortestKind {
    #[display("SINGLE")]
    Single, // single shortest path
    #[display("ALL")]
    All, // all shortest path
}

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum PathFactor {
    #[display("{}", _0)]
    Simple(SimplePattern),
    #[display("{}", _0)]
    Quantified(QuantifiedPath),
}

#[derive(Debug)]
pub struct SimplePattern {
    pub nodes: Vec<NodePattern>,
    pub relationships: Vec<RelationshipPattern>,
}

impl std::fmt::Display for SimplePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.nodes.iter().map(|x| x.to_string());
        let e = self.relationships.iter().map(|x| x.to_string());
        for x in n.interleave(e) {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct QuantifiedPath {
    pub non_selective_part: Box<PatternPart>,
    pub quantifier: PatternQuantifier,
    pub filter: Option<Box<Expr>>,
}

impl std::fmt::Display for QuantifiedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(f, "{}", self.non_selective_part)?;
        if let Some(filter) = self.filter.as_ref() {
            write!(f, " WHERE {}", filter)?;
        }
        write!(f, ")")?;
        write!(f, "{}", self.quantifier)?;
        Ok(())
    }
}

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum PatternQuantifier {
    #[display("{{+}}")]
    Plus, // {+}
    #[display("{{*}}")]
    Start, // {*}
    #[display("{{{}}}", _0)]
    Fixed(u32), // {n}
    #[display("{{{},{}}}", lower.map(|x| x.to_string()).unwrap_or_default(), upper.map(|x| x.to_string()).unwrap_or_default())]
    Interval { lower: Option<u32>, upper: Option<u32> }, // {n,m}
}

#[derive(Debug, Display)]
#[display("({}{}{})", variable.as_ref().map(|x| x.to_string()).unwrap_or_default(),
    label_expr.as_ref().map(|x| format!(":{x}")).unwrap_or_default(),
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
            self.label_expr.as_ref().map(|x| format!(":{x}")).unwrap_or_default()
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

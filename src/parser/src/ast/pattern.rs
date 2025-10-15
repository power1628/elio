use std::ops::Range;

use derive_more::Display;
use itertools::{self, Itertools};

use crate::ast::{AstMeta, Expr, LabelExpr, RawMeta};

#[derive(Debug)]
pub struct MatchPattern<T: AstMeta> {
    pub patterns: Vec<PatternPart<T>>,
}

impl std::fmt::Display for MatchPattern<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.patterns.iter().map(|x| x.to_string()).join(", "))
    }
}

#[derive(Debug)]
pub struct UpdatePattern<T: AstMeta> {
    pub patterns: Vec<PatternPart<T>>,
}

impl std::fmt::Display for UpdatePattern<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.patterns.iter().map(|x| x.to_string()).join(", "))
    }
}

#[derive(Default, Debug, Display, PartialEq, Eq)]
#[display("{}", _0)]
pub enum Selector {
    #[default]
    #[display("")]
    AllPaths, // ALL PATHS
    #[display("ANY {} PATHS", _0)]
    AnyPath(u32), // ANY <count> PATHS
    #[display("ALL SHORTEST PATHS")]
    AllShortest, // ALL SHORTEST PATHS
    #[display("ANY SHORTEST PATHS")]
    AnyShortestPath, // ANY SHORTEST PATHS
    #[display("SHORTEST {} PATHS", _0)]
    CountedShortestPath(u32),
    #[display("SHORTEST {} PATH GROUPS", _0)]
    CountedShortestGroup(u32),
}

impl Selector {
    pub fn is_selective(&self) -> bool {
        !matches!(self, Self::AllPaths)
    }

    pub fn is_counted(&self) -> bool {
        matches!(
            self,
            Self::AnyPath(_) | Self::CountedShortestPath(_) | Self::CountedShortestGroup(_)
        )
    }
}

#[derive(Debug)]
pub struct PatternPart<T: AstMeta> {
    pub variable: Option<String>, // pattern part with name
    pub selector: Selector,
    pub factors: Vec<PathFactor<T>>, // must start with simple and ends with simple
}

impl<T: AstMeta> PatternPart<T> {
    pub fn is_anonymous(&self) -> bool {
        self.variable.is_none()
    }
}

impl std::fmt::Display for PatternPart<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.variable.as_ref() {
            write!(f, "{name} = ")?;
        }
        if self.selector.is_selective() {
            write!(f, "{} ", self.selector)?;
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
pub enum PathFactor<T: AstMeta> {
    #[display("{}", _0)]
    Simple(SimplePathPattern<T>),
    #[display("{}", _0)]
    Quantified(QuantifiedPathPattern<T>),
}

#[derive(Debug)]
pub struct SimplePathPattern<T: AstMeta> {
    pub nodes: Vec<NodePattern<T>>,
    pub relationships: Vec<RelationshipPattern<T>>,
}

impl std::fmt::Display for SimplePathPattern<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.nodes.iter().map(|x| x.to_string());
        let e = self.relationships.iter().map(|x| x.to_string());
        for x in n.interleave(e) {
            write!(f, "{x}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct QuantifiedPathPattern<T: AstMeta> {
    pub non_selective_part: Box<PatternPart<T>>,
    pub quantifier: PatternQuantifier,
    pub filter: Option<Box<Expr<T>>>,
}

impl std::fmt::Display for QuantifiedPathPattern<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " (")?;
        write!(f, "{}", self.non_selective_part)?;
        if let Some(filter) = self.filter.as_ref() {
            write!(f, " WHERE {filter}")?;
        }
        write!(f, ")")?;
        write!(f, "{} ", self.quantifier)?;
        Ok(())
    }
}

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum PatternQuantifier {
    #[display("+")]
    Plus, // {+}
    #[display("*")]
    Star, // {*}
    #[display("{{{}}}", _0)]
    Fixed(u32), // {n}
    #[display("{{{},{}}}", lower.map(|x| x.to_string()).unwrap_or_default(), upper.map(|x| x.to_string()).unwrap_or_default())]
    Interval { lower: Option<u32>, upper: Option<u32> }, // {n,m}
}

#[derive(Debug)]
pub struct NodePattern<T: AstMeta> {
    pub variable: Option<String>,
    pub label_expr: Option<LabelExpr>,
    pub properties: Option<Expr<T>>,
    pub predicate: Option<Expr<T>>,
}

impl std::fmt::Display for NodePattern<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}{}{})",
            self.variable.as_ref().map(|x| x.to_string()).unwrap_or_default(),
            self.label_expr.as_ref().map(|x| format!(":{x}")).unwrap_or_default(),
            self.properties.as_ref().map(|x| x.to_string()).unwrap_or_default()
        )
    }
}

#[derive(Default, Debug)]
pub struct RelationshipPattern<T: AstMeta> {
    pub variable: Option<String>,
    pub label_expr: Option<LabelExpr>,
    pub properties: Option<Expr<T>>,
    pub predicate: Option<Expr<T>>,
    // (a)-[]->(b): None, no length
    // (a)-[*]->(b): Some(None), any length
    // (a)-[*1..3]->(b): Some(Some(1, 3)), min..max
    // (a)-[*1]->(b): Some(Some(1, 1)), length
    // (a)[*3..]->(b): Some(Some(3, None)), min..inf
    pub length: Option<Option<Range<usize>>>,
    pub direction: SemanticDirection,
}

impl std::default::Default for RelationshipPattern<RawMeta> {
    fn default() -> Self {
        Self {
            variable: Default::default(),
            label_expr: Default::default(),
            properties: Default::default(),
            predicate: Default::default(),
            length: Default::default(),
            direction: Default::default(),
        }
    }
}

impl<T: AstMeta> RelationshipPattern<T> {
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

impl std::fmt::Display for RelationshipPattern<RawMeta> {
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

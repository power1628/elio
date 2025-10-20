use derive_more::Display;
use itertools::{self, Itertools};

use crate::ast::{Expr, MatchPattern, OrderBy, ReturnItems, pattern::UpdatePattern};

#[derive(Debug)]
pub struct RegularQuery {
    pub queries: Vec<SingleQuery>,
    pub union_all: bool,
}

impl std::fmt::Display for RegularQuery {
    #[allow(unstable_name_collisions)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let queries = self.queries.iter().map(|x| x.to_string());
        let sep = if self.union_all { " UNION ALL " } else { " UNION " };
        write!(f, "{}", queries.intersperse(sep.to_string()).join(""))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SingleQuery {
    pub clauses: Vec<Clause>,
}

impl std::fmt::Display for SingleQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clauses.iter().map(|x| x.to_string()).join(" "))
    }
}

#[derive(Debug, Display)]
pub enum Clause {
    #[display("CREATE {}", _0)]
    Create(CreateClause),
    // INSERT
    // DELETE
    // SET
    // REMOVE
    // MERGE
    // LET
    #[display("{}", _0)]
    Match(MatchClause),
    #[display("{}", _0)]
    With(WithClause),
    #[display("{}", _0)]
    Return(ReturnClause),
    #[display("{}", _0)]
    Unwind(UnwindClause),
}

#[derive(Debug, Display)]
pub struct CreateClause {
    // pattern: Pattern,
    pub pattern: UpdatePattern,
}

#[derive(Debug)]
pub struct MatchClause {
    pub optional: bool,
    pub mode: MatchMode,
    pub pattern: MatchPattern,
    // TODO(pgao): hints
    pub where_: Option<Box<Expr>>,
}

impl std::fmt::Display for MatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.optional {
            write!(f, "OPTIONAL ")?;
        }
        write!(f, "MATCH")?;
        write!(f, "{}", self.mode)?;
        write!(f, " {}", self.pattern)?;
        if let Some(where_) = self.where_.as_ref() {
            write!(f, " WHERE {where_}")?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Display)]
pub enum MatchMode {
    WALK, // no constraint
    #[default]
    #[display("")]
    TRAIL, // different relationship
}

#[derive(Debug)]
pub struct WithClause {
    pub distinct: bool,
    pub return_items: ReturnItems,
    pub order_by: Option<OrderBy>,
    pub skip: Option<Box<Expr>>,
    pub limit: Option<Box<Expr>>,
    pub where_: Option<Box<Expr>>,
}

impl std::fmt::Display for WithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WITH")?;
        if self.distinct {
            write!(f, " DISTINCT")?;
        }
        write!(f, " {}", self.return_items)?;
        if let Some(order_by) = self.order_by.as_ref() {
            write!(f, " ORDER BY {order_by}")?;
        }
        if let Some(skip) = self.skip.as_ref() {
            write!(f, " SKIP {skip}")?;
        }
        if let Some(limit) = self.limit.as_ref() {
            write!(f, " LIMIT {limit}")?;
        }
        if let Some(where_) = self.where_.as_ref() {
            write!(f, " WHERE {where_}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReturnClause {
    pub distinct: bool,
    pub return_items: ReturnItems,
    pub order_by: Option<OrderBy>,
    pub skip: Option<Box<Expr>>,
    pub limit: Option<Box<Expr>>,
}

impl std::fmt::Display for ReturnClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RETURN")?;
        if self.distinct {
            write!(f, " DISTINCT")?;
        }
        write!(f, " {}", self.return_items)?;
        if let Some(order_by) = self.order_by.as_ref() {
            write!(f, " ORDER BY {order_by}")?;
        }
        if let Some(skip) = self.skip.as_ref() {
            write!(f, " SKIP {skip}")?;
        }
        if let Some(limit) = self.limit.as_ref() {
            write!(f, " LIMIT {limit}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Display)]
#[display("UNWIND {} AS {}", self.expr, self.variable)]
pub struct UnwindClause {
    pub expr: Box<Expr>,
    pub variable: String,
}

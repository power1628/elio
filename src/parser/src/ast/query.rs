use derive_more::Display;
use itertools::{self, Itertools};

use crate::ast::pattern::UpdatePattern;

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
}

#[derive(Debug, Display)]
pub struct CreateClause {
    // pattern: Pattern,
    pub pattern: UpdatePattern,
}

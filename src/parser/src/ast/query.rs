use derive_more::Display;
use itertools::{self, Itertools};

use crate::ast::{AstMeta, RawMeta, pattern::UpdatePattern};

#[derive(Debug)]
pub struct RegularQuery<T: AstMeta> {
    pub queries: Vec<SingleQuery<T>>,
    pub union_all: bool,
}

impl std::fmt::Display for RegularQuery<RawMeta> {
    #[allow(unstable_name_collisions)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let queries = self.queries.iter().map(|x| x.to_string());
        let sep = if self.union_all { " UNION ALL " } else { " UNION " };
        write!(f, "{}", queries.intersperse(sep.to_string()).join(""))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SingleQuery<T: AstMeta> {
    pub clauses: Vec<Clause<T>>,
}

impl std::fmt::Display for SingleQuery<RawMeta> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clauses.iter().map(|x| x.to_string()).join(" "))
    }
}

#[derive(Debug, Display)]
pub enum Clause<T: AstMeta> {
    #[display("CREATE {}", _0)]
    Create(CreateClause<T>),
}

#[derive(Debug, Display)]
pub struct CreateClause<T: AstMeta> {
    // pattern: Pattern,
    pub pattern: UpdatePattern<T>,
}

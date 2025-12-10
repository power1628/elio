use derive_more::Display;

use crate::ast::RegularQuery;

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum Statement {
    // Analyze
    // Explain
    Query(Box<RegularQuery>),
}

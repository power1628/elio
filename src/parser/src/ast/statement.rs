use crate::ast::RegularQuery;

use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum Statement {
    // Analyze
    // Explain
    Query(Box<RegularQuery>),
}

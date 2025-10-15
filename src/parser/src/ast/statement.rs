use crate::ast::{AstMeta, RegularQuery};

use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", _0)]
pub enum Statement<T: AstMeta> {
    // Analyze
    // Explain
    Query(Box<RegularQuery<T>>),
}

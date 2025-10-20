use indexmap::IndexMap;
use mojito_parser::ast::SortDirection;

use crate::{expr::Expr, variable::Variable};

pub struct SortItem {
    pub expr: Box<Expr>,
    pub projections: IndexMap<Variable, Expr>,
    pub direction: SortDirection,
}

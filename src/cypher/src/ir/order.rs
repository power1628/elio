use indexmap::IndexMap;
use mojito_parser::ast::SortDirection;

use crate::{expr::Expr, variable::Variable};

pub struct SortItem {
    pub expr: Box<Expr>,
    pub projections: IndexMap<Variable, Expr>,
    pub direction: SortDirection,
}

pub struct OrderingSet {
    choices: Vec<OrderingChoice>,
}

// TODO(pgao): ordering choices should contain variable groups.
// some variables may be equal.
pub struct OrderingChoice {
    pub items: Vec<SortItem>,
}

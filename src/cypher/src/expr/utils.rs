use crate::{expr::Expr, variable::Variable};

pub struct FilterExprs {
    exprs: Vec<Expr>,
}

pub struct ProjectItem {
    pub variable: Variable,
    pub expr: Box<Expr>,
}

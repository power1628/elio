use crate::{expr::Expr, variable::Variable};

#[derive(Default)]
pub struct FilterExprs {
    exprs: Vec<Expr>,
}

pub struct ProjectItem {
    pub variable: Variable,
    pub expr: Box<Expr>,
}

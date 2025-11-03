use mojito_common::schema::Variable;

use crate::expr::Expr;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct FilterExprs {
    // conjuncted by AND
    exprs: Vec<Expr>,
}

impl FilterExprs {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_true(&self) -> bool {
        self.exprs.is_empty()
    }

    pub fn push(&mut self, expr: Expr) {
        self.exprs.push(expr);
    }

    pub fn and(mut self, other: Self) -> Self {
        self.exprs.extend(other.exprs);
        self.normalize()
    }

    pub fn or(self, other: Self) -> Self {
        let lhs: Expr = self.into();
        let rhs: Expr = other.into();
        let or = lhs.or(rhs);
        let mut ret = Self::empty();
        ret.exprs.push(or);
        ret.normalize()
    }

    // TODO(pgao): remove false
    fn normalize(self) -> Self {
        self
    }
}

impl From<FilterExprs> for Expr {
    fn from(val: FilterExprs) -> Self {
        val.exprs.into_iter().fold(Expr::boolean(true), |acc, e| acc.and(e))
    }
}

impl From<Expr> for FilterExprs {
    fn from(val: Expr) -> Self {
        let mut ret = Self::empty();
        ret.exprs.push(val);
        ret
    }
}

pub struct ProjectItem {
    pub variable: Variable,
    pub expr: Box<Expr>,
}

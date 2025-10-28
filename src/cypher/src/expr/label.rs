use std::collections::HashSet;

use mojito_common::{TokenId, data_type::DataType};

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum IrToken {
    Resolved(TokenId),
    Unresolved(String),
}

impl From<Option<TokenId>> for IrToken {
    fn from(token: Option<TokenId>) -> Self {
        match token {
            Some(token) => Self::Resolved(token),
            None => Self::Unresolved("".to_string()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LabelExpr {
    pub entity: Box<Expr>,
    pub op: LabelOp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LabelOp {
    // at least one label
    // unreachable
    HasA,
    // has any label contained
    // unreachable
    HasAny(HashSet<IrToken>),
    // has exact given labels
    HasAll(HashSet<IrToken>),
}

impl ExprNode for LabelExpr {
    fn typ(&self) -> DataType {
        DataType::Boolean
    }
}

impl From<LabelExpr> for Expr {
    fn from(val: LabelExpr) -> Self {
        Expr::Label(val)
    }
}

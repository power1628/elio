use std::collections::HashSet;

use educe::Educe;
use mojito_common::IrToken;
use mojito_common::data_type::DataType;

use crate::expr::{Expr, ExprNode};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LabelExpr {
    pub entity: Box<Expr>,
    pub op: LabelOp,
}

#[derive(Educe, derive_more::Display)]
#[educe(Debug, Hash, Clone, Eq, PartialEq)]
pub enum LabelOp {
    // at least one label
    // unreachable
    #[display("HasOne")]
    HasA,
    // has any label contained
    // unreachable
    #[display("HasAny[{}]", _0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))]
    HasAny(#[educe(Hash(ignore))] HashSet<IrToken>),
    // has exact given labels
    #[display("HasAll[{}]", _0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))]
    HasAll(#[educe(Hash(ignore))] HashSet<IrToken>),
}

impl ExprNode for LabelExpr {
    fn typ(&self) -> DataType {
        DataType::Bool
    }
}

impl From<LabelExpr> for Expr {
    fn from(val: LabelExpr) -> Self {
        Expr::LabelExpr(val)
    }
}

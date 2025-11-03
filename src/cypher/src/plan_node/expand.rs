use mojito_common::variable::VariableName;
use mojito_parser::ast::SemanticDirection;

use crate::{
    expr::IrToken,
    plan_node::{PlanExpr, plan_base::PlanBase},
};

#[derive(Debug, Clone, Copy)]
pub enum ExpandKind {
    // input (a), output (a)-[r]-(b)
    All,
    // input (a), (b), output (a)-[r]-(b)
    Into,
}

#[derive(Clone, Debug)]
pub struct Expand {
    pub base: PlanBase,
    pub input: Box<PlanExpr>,
    pub from: VariableName,
    pub to: Option<VariableName>,
    pub direction: SemanticDirection,
    pub types: Vec<IrToken>,
    pub kind: ExpandKind,
}

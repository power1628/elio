use mojito_common::variable::VariableName;

use crate::{expr::Expr, plan_node::plan_base::PlanBase};

#[derive(Debug, Clone)]
pub struct Project {
    pub base: PlanBase,
    pub projections: Vec<(VariableName, Expr)>,
    // private
    // TODO(pgao): internal func deps
}

use mojito_common::{schema::Variable, variable::VariableName};

use crate::plan_node::plan_base::PlanBase;

#[derive(Debug, Clone)]
pub struct AllNodeScan {
    base: PlanBase,
    pub variable: VariableName,
    pub arguments: Vec<Variable>,
}

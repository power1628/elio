use mojito_common::{schema::Variable, variable::VariableName};

use crate::plan_node::plan_base::PlanBase;

// TODO(pgao): associate the catalog object here?
// seems we should have an logical plan here?
#[derive(Debug, Clone)]
pub struct AllNodeScan {
    pub base: PlanBase,
    pub variable: VariableName,
    pub arguments: Vec<Variable>,
}

impl AllNodeScan {}

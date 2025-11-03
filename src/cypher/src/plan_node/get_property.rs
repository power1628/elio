use mojito_common::variable::VariableName;

use crate::plan_node::{PlanExpr, plan_base::PlanBase};

/// Fetch all properties of given entity
#[derive(Clone, Debug)]
pub struct FetchAllProperties {
    pub base: PlanBase,
    pub input: Box<PlanExpr>,
    pub entities: Vec<VariableName>,
}

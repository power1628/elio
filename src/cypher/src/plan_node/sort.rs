use mojito_common::order::ColumnOrder;

use crate::plan_node::plan_base::PlanBase;

#[derive(Debug, Clone)]
pub struct Sort {
    pub base: PlanBase,
    pub items: Vec<ColumnOrder>,
}

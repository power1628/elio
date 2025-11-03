use crate::plan_node::{PlanExpr, plan_base::PlanBase};

#[derive(Debug, Clone)]
pub struct Apply {
    pub base: PlanBase,
    pub left: Box<PlanExpr>,
    pub right: Box<PlanExpr>,
}

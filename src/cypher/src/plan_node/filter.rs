use crate::{
    expr::FilterExprs,
    plan_node::{PlanExpr, plan_base::PlanBase},
};

#[derive(Debug, Clone)]
pub struct Filter {
    pub base: PlanBase,
    pub input: Box<PlanExpr>,
    pub exprs: FilterExprs,
}

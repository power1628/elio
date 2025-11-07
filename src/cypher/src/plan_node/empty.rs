use std::sync::Arc;

use mojito_common::schema::Schema;

use crate::{plan_context::PlanContext, plan_node::plan_base::PlanBase};

#[derive(Debug, Clone)]
pub struct Empty {
    pub base: PlanBase,
}

impl Empty {
    pub fn new(ctx: Arc<PlanContext>) -> Self {
        Self {
            base: PlanBase::new(Schema::empty().into(), ctx),
        }
    }
}

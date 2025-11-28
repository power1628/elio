use std::sync::Arc;

use mojito_common::LabelId;
use mojito_common::schema::{Schema, Variable};

use crate::expr::BoxedExpr;
use crate::plan_node::plan_base::PlanBase;
use crate::plan_node::{InnerNode, PlanExpr, PlanNode};

#[derive(Clone, Debug)]
pub struct CreateRel {
    pub base: PlanBase,
    inner: CreateRelInner,
}

#[derive(Clone, Debug)]
pub struct CreateRelInner {
    input: Box<PlanExpr>,
    reltype: LabelId,
    start_node: BoxedExpr,
    end_node: BoxedExpr,
    properties: BoxedExpr,
    variable: Variable,
}

impl CreateRelInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.input.schema());
        schema.add_column(self.variable.clone());
        schema.into()
    }
}

impl InnerNode for CreateRelInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.input.ctx())
    }
}

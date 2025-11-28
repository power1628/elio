use std::sync::Arc;

use mojito_common::LabelId;
use mojito_common::schema::{Schema, Variable};

use crate::expr::BoxedExpr;
use crate::plan_node::plan_base::PlanBase;
use crate::plan_node::{InnerNode, PlanExpr, PlanNode};

#[derive(Clone, Debug)]
pub struct CreateNode {
    pub base: PlanBase,
    inner: CreateNodeInner,
}

#[derive(Clone, Debug)]
pub struct CreateNodeInner {
    input: Box<PlanExpr>,
    labels: Vec<LabelId>,
    properties: BoxedExpr,
    variable: Variable,
}

impl CreateNodeInner {
    fn build_schema(&self) -> Arc<Schema> {
        let mut schema = Schema::from_arc(self.input.schema());
        schema.add_column(self.variable.clone());
        schema.into()
    }
}
impl InnerNode for CreateNodeInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.input.ctx())
    }
}

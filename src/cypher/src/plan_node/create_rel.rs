use std::sync::Arc;

use mojito_common::schema::{Schema, Variable};

use crate::expr::{BoxedExpr, IrToken};
use crate::plan_node::plan_base::PlanBase;
use crate::plan_node::{InnerNode, PlanExpr, PlanNode};

#[derive(Clone, Debug)]
pub struct CreateRel {
    pub base: PlanBase,
    inner: CreateRelInner,
}

impl CreateRel {
    pub fn new(inner: CreateRelInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateRelInner {
    pub(crate) input: Box<PlanExpr>,
    pub(crate) reltype: IrToken,
    pub(crate) start_node: BoxedExpr,
    pub(crate) end_node: BoxedExpr,
    pub(crate) properties: BoxedExpr,
    pub(crate) variable: Variable,
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

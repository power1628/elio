use std::sync::Arc;

use mojito_common::IrToken;
use mojito_common::schema::{Schema, Variable};

use crate::expr::BoxedExpr;
use crate::plan_node::plan_base::PlanBase;
use crate::plan_node::{InnerNode, PlanExpr};

#[derive(Clone, Debug)]
pub struct CreateNode {
    pub base: PlanBase,
    pub inner: CreateNodeInner,
    _priv: (),
}

impl CreateNode {
    pub fn new(inner: CreateNodeInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
            _priv: (),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateNodeInner {
    pub input: Box<PlanExpr>,
    pub labels: Vec<IrToken>,
    pub properties: BoxedExpr,
    pub variable: Variable,
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

    fn inputs(&self) -> Vec<&PlanExpr> {
        vec![&self.input]
    }
}

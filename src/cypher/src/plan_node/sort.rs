use std::sync::Arc;

use mojito_common::{order::ColumnOrder, schema::Schema};

use crate::plan_node::{InnerNode, PlanExpr, PlanNode, plan_base::PlanBase};

#[derive(Debug, Clone)]
pub struct Sort {
    pub base: PlanBase,
    inner: SortInner,
}

impl Sort {
    pub fn new(inner: SortInner) -> Self {
        Self {
            base: inner.build_base(),
            inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortInner {
    pub input: Box<PlanExpr>,
    pub items: Vec<ColumnOrder>,
}
impl SortInner {
    fn build_schema(&self) -> Arc<Schema> {
        self.input.schema()
    }
}

impl InnerNode for SortInner {
    fn build_base(&self) -> PlanBase {
        PlanBase::new(self.build_schema(), self.input.ctx())
    }
}

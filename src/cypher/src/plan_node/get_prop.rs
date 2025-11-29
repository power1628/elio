use std::sync::Arc;

use mojito_common::schema::Schema;
use mojito_common::variable::VariableName;

use crate::plan_node::plan_base::PlanBase;
use crate::plan_node::{InnerNode, PlanExpr};

/// Fetch all properties of given entity
/// This should be an enforce operator
#[derive(Clone, Debug)]
pub struct GetProperty {
    pub base: PlanBase,
    pub(crate) inner: GetPropertyInner,
}

#[derive(Clone, Debug)]
pub struct GetPropertyInner {
    input: Box<PlanExpr>,
    entities: Vec<VariableName>,
}

impl GetPropertyInner {
    fn build_schema(&self) -> Arc<Schema> {
        self.input.schema()
    }
}

impl InnerNode for GetPropertyInner {
    fn build_base(&self) -> PlanBase {
        let schema = self.build_schema();
        PlanBase::new(schema, self.input.ctx())
    }

    fn inputs(&self) -> Vec<&PlanExpr> {
        vec![&self.input]
    }
}

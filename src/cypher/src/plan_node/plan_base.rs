use std::sync::Arc;

use mojito_common::schema::Schema;

use crate::props::order::Ordering;

pub type PlanNodeId = usize;

// plan base structure and properties
#[derive(Debug, Clone)]
pub struct PlanBase {
    id: PlanNodeId,
    schema: Arc<Schema>,
    // pub func_deps: FuncDepSet,
    // plan node properties
    pub attrs: PlanNodeAttrs,
}

impl PlanBase {
    pub fn new(id: PlanNodeId, schema: Arc<Schema>) -> Self {
        Self {
            id,
            schema,
            attrs: PlanNodeAttrs::empty(),
        }
    }

    pub fn id(&self) -> PlanNodeId {
        self.id
    }

    pub fn schema(&self) -> &Arc<Schema> {
        &self.schema
    }
}

#[derive(Debug, Clone)]
pub struct PlanNodeAttrs {
    // bitset to mark if ordering is valid
    // none for ordering is not derived and needs to be computed
    pub order: Option<Ordering>,
}

impl PlanNodeAttrs {
    pub fn empty() -> Self {
        Self { order: None }
    }
}

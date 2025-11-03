use std::sync::Arc;

use mojito_common::schema::Schema;

use crate::props::{func_dep::FuncDepSet, order::Ordering};

pub type PlanNodeId = usize;

// plan base structure and properties
#[derive(Debug, Clone)]
pub struct PlanBase {
    pub id: PlanNodeId,
    pub schema: Arc<Schema>,
    pub func_deps: FuncDepSet,
    // plan node properties
    pub attrs: PlanNodeAttrs,
}

#[derive(Debug, Clone)]
pub struct PlanNodeAttrs {
    // bitset to mark if ordering is valid
    pub order: Ordering,
}

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use mojito_catalog::Catalog;

use crate::plan_node::plan_base::PlanNodeId;
use crate::session::SessionContext;
use crate::variable::VariableGenerator;

pub struct PlanContext {
    pub sctx: Arc<dyn SessionContext>,
    plan_node_gen: AtomicUsize,
    var_gen: Arc<VariableGenerator>,
}

impl PlanContext {
    pub fn new(sctx: Arc<dyn SessionContext>) -> Self {
        Self {
            sctx,
            plan_node_gen: AtomicUsize::new(0),
            var_gen: Arc::new(VariableGenerator::default()),
        }
    }

    pub fn plan_node_id(&self) -> PlanNodeId {
        self.plan_node_gen.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn var_gen(&self) -> &Arc<VariableGenerator> {
        &self.var_gen
    }

    pub fn catalog(&self) -> &Arc<Catalog> {
        &self.sctx.catalog()
    }
}

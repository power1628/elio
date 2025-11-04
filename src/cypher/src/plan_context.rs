use std::sync::atomic::AtomicUsize;

use crate::{plan_node::plan_base::PlanNodeId, statement::StmtContext};

pub struct PlanContext<'a> {
    pub sctx: StmtContext<'a>,
    pub next_plan_node_id: AtomicUsize,
}

impl<'a> PlanContext<'a> {
    pub fn new(sctx: StmtContext<'a>) -> Self {
        Self {
            sctx,
            next_plan_node_id: AtomicUsize::new(0),
        }
    }

    pub fn plan_id(&self) -> PlanNodeId {
        self.next_plan_node_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

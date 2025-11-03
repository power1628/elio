use std::sync::Arc;

use mojito_common::schema::Schema;

use crate::plan_node::{
    all_node_scan::AllNodeScan,
    plan_base::{PlanNodeAttrs, PlanNodeId},
};

pub mod all_node_scan;
pub mod expand;
pub mod plan_base;
pub mod rel_scan;

pub enum PlanExpr {
    AllNodeScan(AllNodeScan),
}

pub trait PlanNode {
    fn id(&self) -> PlanNodeId;
    fn schema(&self) -> Arc<Schema>;
    fn attrs(&self) -> PlanNodeAttrs;
}

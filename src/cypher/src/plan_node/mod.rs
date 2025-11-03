use std::sync::Arc;

use mojito_common::schema::Schema;

use crate::plan_node::plan_base::{PlanNodeAttrs, PlanNodeId};

pub mod all_node_scan;
pub mod apply;
pub mod expand;
pub mod filter;
pub mod get_property;
pub mod plan_base;
pub mod project;
pub mod sort;
pub use all_node_scan::*;
pub use apply::*;
pub use expand::*;
pub use filter::*;
pub use get_property::*;
pub use project::*;
pub use sort::*;

#[derive(Clone, Debug)]
pub enum PlanExpr {
    // graph
    AllNodeScan(AllNodeScan),
    FetchAllProperties(FetchAllProperties),
    Expand(Expand),
    Apply(Apply),
    // relational
    Project(Project),
    Sort(Sort),
    Filter(Filter),
}

pub trait PlanNode {
    fn id(&self) -> PlanNodeId;
    fn schema(&self) -> Arc<Schema>;
    fn attrs(&self) -> PlanNodeAttrs;
}

macro_rules! impl_plan_node {
    ($($plan_node:ident),*) => {
        $(
            impl PlanNode for $plan_node {
                fn id(&self) -> PlanNodeId {
                    self.base.id
                }

                fn schema(&self) -> Arc<Schema> {
                    self.base.schema.clone()
                }

                fn attrs(&self) -> PlanNodeAttrs {
                    self.base.attrs.clone()
                }
            }
        )*
    };
}

impl_plan_node!(AllNodeScan, FetchAllProperties, Expand, Apply, Project, Sort, Filter);

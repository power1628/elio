use std::sync::Arc;

use crate::expr::Expr;
use crate::expr::ExprNode;
use mojito_common::data_type::DataType;
use mojito_common::schema::Schema;
use mojito_common::{schema::Variable, variable::VariableName};

use crate::{
    plan_context::PlanContext,
    plan_node::plan_base::{PlanBase, PlanNodeId},
};

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
    MaterializeEntity(MaterializeEntity),
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
    fn ctx(&self) -> Arc<PlanContext>;
}

pub trait InnerNode {
    fn build_base(&self) -> PlanBase;
}

macro_rules! impl_plan_node {
    ($($plan_node:ident),*) => {
        $(
            impl PlanNode for $plan_node {
                fn id(&self) -> PlanNodeId {
                    self.base.id()
                }

                fn schema(&self) -> Arc<Schema> {
                    self.base.schema().clone()
                }

                fn ctx(&self) -> Arc<PlanContext> {
                    self.base.ctx()
                }
            }
        )*
    };
}

impl_plan_node!(AllNodeScan, MaterializeEntity, Expand, Apply, Project, Sort, Filter);

macro_rules! impl_plan_node_for_expr {
    ($($plan_node:ident),*) => {
        impl PlanNode for PlanExpr {
            fn id(&self) -> PlanNodeId {
                match self {
                    $(PlanExpr::$plan_node(p) => p.id(),)*
                }
            }

            fn schema(&self) -> Arc<Schema> {
                match self {
                    $(PlanExpr::$plan_node(p) => p.schema(),)*
                }
            }

            fn ctx(&self) -> Arc<PlanContext>{
                match self {
                    $(PlanExpr::$plan_node(p) => p.ctx(),)*
                }
            }
        }
    };
}

impl_plan_node_for_expr!(AllNodeScan, MaterializeEntity, Expand, Apply, Project, Sort, Filter);

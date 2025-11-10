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
pub mod argument;
pub mod empty;
pub mod expand;
pub mod filter;
pub mod get_prop;
pub mod pagination;
pub mod plan_base;
pub mod project;
pub mod sort;
pub use all_node_scan::*;
pub use apply::*;
pub use argument::*;
pub use empty::*;
pub use expand::*;
pub use filter::*;
pub use get_prop::*;
pub use pagination::*;
pub use project::*;
pub use sort::*;

#[derive(Clone, Debug)]
pub enum PlanExpr {
    // graph
    AllNodeScan(AllNodeScan),
    GetProperty(GetProperty),
    Expand(Expand),
    Apply(Apply),
    Argument(Argument),
    // relational
    Project(Project),
    Sort(Sort),
    Filter(Filter),
    Pagination(Pagination),
    Empty(Empty),
}

impl PlanExpr {
    pub fn empty(ctx: Arc<PlanContext>) -> Self {
        Self::Empty(Empty::new(ctx))
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }
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

            // impl To for plan_node
            impl From<$plan_node> for PlanExpr {
                fn from(value: $plan_node) -> Self {
                    Self::$plan_node(value)
                }
            }

            impl From<$plan_node> for Box<PlanExpr> {
                fn from(value: $plan_node) -> Self {
                    Box::new(PlanExpr::$plan_node(value))
                }
            }
        )*
    };
}

impl_plan_node!(
    AllNodeScan,
    GetProperty,
    Expand,
    Apply,
    Argument,
    Project,
    Sort,
    Filter,
    Pagination,
    Empty
);

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

impl_plan_node_for_expr!(
    AllNodeScan,
    GetProperty,
    Expand,
    Apply,
    Argument,
    Project,
    Sort,
    Filter,
    Pagination,
    Empty
);

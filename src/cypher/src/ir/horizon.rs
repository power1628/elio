use indexmap::IndexMap;
use mojito_common::schema::Variable;
use mojito_common::variable::VariableName;

use crate::expr::{Expr, FilterExprs};
use crate::ir::order::SortItem;

pub enum QueryHorizon {
    Unwind(UnwindProjection),
    Project(QueryProjection),
}

impl QueryHorizon {
    pub fn set_order_by(&mut self, order_by: Vec<SortItem>) {
        match self {
            QueryHorizon::Unwind(_) => unreachable!(),
            QueryHorizon::Project(q) => q.set_order_by(order_by),
        }
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        match self {
            QueryHorizon::Unwind(_) => unreachable!(),
            QueryHorizon::Project(q) => q.set_pagination(pagination),
        }
    }

    pub fn set_filter(&mut self, filter: FilterExprs) {
        match self {
            QueryHorizon::Unwind(_) => unreachable!(),
            QueryHorizon::Project(q) => q.set_filter(filter),
        }
    }
}

impl std::default::Default for QueryHorizon {
    fn default() -> Self {
        Self::Project(QueryProjection::empty())
    }
}

impl QueryHorizon {
    pub fn empty() -> Self {
        Self::default()
    }
}

pub enum QueryProjection {
    Regular(RegularProjection),
    Aggregate(AggregateProjection),
    Distinct(DistinctProjection),
}

impl QueryProjection {
    pub fn empty() -> Self {
        Self::Regular(RegularProjection::default())
    }

    pub fn set_order_by(&mut self, order_by: Vec<SortItem>) {
        match self {
            QueryProjection::Regular(r) => r.order_by = order_by,
            QueryProjection::Aggregate(a) => a.order_by = order_by,
            QueryProjection::Distinct(d) => d.order_by = order_by,
        }
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        match self {
            QueryProjection::Regular(r) => r.pagination = pagination,
            QueryProjection::Aggregate(a) => a.pagination = pagination,
            QueryProjection::Distinct(d) => d.pagination = pagination,
        }
    }

    pub fn set_filter(&mut self, filter: FilterExprs) {
        match self {
            QueryProjection::Regular(r) => r.filter = filter,
            QueryProjection::Aggregate(a) => a.filter = filter,
            QueryProjection::Distinct(d) => d.filter = filter,
        }
    }
}

#[derive(Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Pagination {
    pub fn is_empty(&self) -> bool {
        self.offset.is_none() && self.limit.is_none()
    }
}

pub struct UnwindProjection {
    pub variable: Variable,
    pub expr: Expr,
}

#[derive(Default)]
pub struct RegularProjection {
    pub items: IndexMap<VariableName, Expr>,
    pub order_by: Vec<SortItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    // pub imported_variable: HashSet<Variable>,
}

pub struct AggregateProjection {
    pub group_by: IndexMap<VariableName, Expr>,
    pub aggregate: IndexMap<VariableName, Expr>,
    pub order_by: Vec<SortItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    // TODO(pgao): others
    // pub imported_variables: HashSet<Variable>,
}

pub struct DistinctProjection {
    pub group_by: IndexMap<VariableName, Expr>,
    pub order_by: Vec<SortItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    // pub imported_variables: HashSet<Variable>,
}

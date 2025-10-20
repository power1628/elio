use std::collections::HashSet;

use crate::{
    expr::{Expr, FilterExprs, ProjectItem},
    variable::Variable,
};

pub enum QueryHorizon {
    Unwind(UnwindProjection),
    Project(RegularProjection),
    Aggregate(AggregateProjection),
    Distinct(DistinctProjection),
}

pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

pub struct UnwindProjection {
    pub variable: Variable,
    pub expr: Expr,
}

pub struct RegularProjection {
    pub items: Vec<ProjectItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    pub imported_variable: HashSet<Variable>,
}

pub struct AggregateProjection {
    pub group_by: Vec<ProjectItem>,
    pub aggregate: Vec<ProjectItem>,
    pub pagination: Pagination,
    // TODO(pgao): others
    pub imported_variables: HashSet<Variable>,
}

pub struct DistinctProjection {
    pub group_by: Vec<ProjectItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    pub imported_variables: HashSet<Variable>,
}

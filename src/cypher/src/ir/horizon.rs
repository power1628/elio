use indexmap::IndexMap;
use mojito_common::schema::Variable;
use mojito_common::variable::VariableName;
use pretty_xmlish::{Pretty, XmlNode};

use crate::expr::{Expr, FilterExprs};
use crate::ir::order::SortItem;
use crate::pretty_utils::{pretty_order_items, pretty_project_items};

// Rename to QueryProjection
pub enum QueryHorizon {
    Unwind(UnwindProjection),
    Project(QueryProjection),
}

impl QueryHorizon {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        match self {
            QueryHorizon::Unwind(u) => u.xmlnode(),
            QueryHorizon::Project(p) => p.xmlnode(),
        }
    }
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
    pub fn xmlnode(&self) -> XmlNode<'_> {
        match self {
            QueryProjection::Regular(r) => r.xmlnode(),
            QueryProjection::Aggregate(a) => a.xmlnode(),
            QueryProjection::Distinct(d) => d.xmlnode(),
        }
    }
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

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(off) = self.offset {
            write!(f, "offset: {}", off)?;
        }
        if let Some(lim) = self.limit {
            write!(f, " limit: {}", lim)?;
        }
        Ok(())
    }
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

impl UnwindProjection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        XmlNode::simple_record(
            "UnwindProjection",
            vec![
                ("variable", Pretty::display(&self.variable.name)),
                ("expr", self.expr.pretty().into()),
            ],
            vec![],
        )
    }
}

#[derive(Default)]
pub struct RegularProjection {
    pub items: IndexMap<VariableName, Expr>,
    pub order_by: Vec<SortItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    // pub imported_variable: HashSet<Variable>,
}

impl RegularProjection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        let mut fields = vec![];
        fields.push(("items", pretty_project_items(self.items.iter())));
        if !self.order_by.is_empty() {
            fields.push(("order_by", pretty_order_items(&self.order_by)));
        };
        if !self.pagination.is_empty() {
            fields.push(("pagination", Pretty::display(&self.pagination)));
        }
        if !self.filter.is_true() {
            fields.push(("filter", Pretty::display(&self.filter.pretty())));
        }

        XmlNode::simple_record("Project", fields, vec![])
    }
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

impl AggregateProjection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        let mut fields = vec![];
        if !self.group_by.is_empty() {
            fields.push(("group_by", pretty_project_items(self.group_by.iter())));
        };
        if !self.aggregate.is_empty() {
            fields.push(("aggregate", pretty_project_items(self.aggregate.iter())));
        };
        if !self.order_by.is_empty() {
            fields.push(("order_by", pretty_order_items(&self.order_by)));
        };
        if !self.pagination.is_empty() {
            fields.push(("pagination", Pretty::display(&self.pagination)));
        }
        if !self.filter.is_true() {
            fields.push(("filter", Pretty::display(&self.filter.pretty())));
        }

        XmlNode::simple_record("Aggregate", fields, vec![])
    }
}

pub struct DistinctProjection {
    pub group_by: IndexMap<VariableName, Expr>,
    pub order_by: Vec<SortItem>,
    pub pagination: Pagination,
    pub filter: FilterExprs,
    // pub imported_variables: HashSet<Variable>,
}

impl DistinctProjection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        let mut fields = vec![];
        if !self.group_by.is_empty() {
            fields.push(("group_by", pretty_project_items(self.group_by.iter())));
        };
        if !self.order_by.is_empty() {
            fields.push(("order_by", pretty_order_items(&self.order_by)));
        };
        if !self.pagination.is_empty() {
            fields.push(("pagination", Pretty::display(&self.pagination)));
        }
        if !self.filter.is_true() {
            fields.push(("filter", Pretty::display(&self.filter.pretty())));
        }

        XmlNode::simple_record("Distinct", fields, vec![])
    }
}

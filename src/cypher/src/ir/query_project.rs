use indexmap::IndexMap;
use mojito_common::schema::Variable;
use mojito_common::variable::VariableName;
use pretty_xmlish::{Pretty, XmlNode};

use crate::expr::{Expr, FilterExprs};
use crate::ir::order::SortItem;
use crate::pretty_utils::{pretty_order_items, pretty_project_items};

// Rename to QueryProjection
pub enum QueryProjection {
    Unwind(Unwind),
    Project(Projection),
}

impl QueryProjection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        match self {
            QueryProjection::Unwind(u) => u.xmlnode(),
            QueryProjection::Project(p) => p.xmlnode(),
        }
    }
}

impl QueryProjection {
    pub fn set_order_by(&mut self, order_by: Vec<SortItem>) {
        match self {
            QueryProjection::Unwind(_) => unreachable!(),
            QueryProjection::Project(q) => q.set_order_by(order_by),
        }
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        match self {
            QueryProjection::Unwind(_) => unreachable!(),
            QueryProjection::Project(q) => q.set_pagination(pagination),
        }
    }

    pub fn set_filter(&mut self, filter: FilterExprs) {
        match self {
            QueryProjection::Unwind(_) => unreachable!(),
            QueryProjection::Project(q) => q.set_filter(filter),
        }
    }
}

impl std::default::Default for QueryProjection {
    fn default() -> Self {
        Self::Project(Projection::empty())
    }
}

impl QueryProjection {
    pub fn empty() -> Self {
        Self::default()
    }
}

pub enum Projection {
    Regular(RegularProjection),
    Aggregate(AggregateProjection),
    Distinct(DistinctProjection),
}
impl Projection {
    pub fn xmlnode(&self) -> XmlNode<'_> {
        match self {
            Projection::Regular(r) => r.xmlnode(),
            Projection::Aggregate(a) => a.xmlnode(),
            Projection::Distinct(d) => d.xmlnode(),
        }
    }
}

impl Projection {
    pub fn empty() -> Self {
        Self::Regular(RegularProjection::default())
    }

    pub fn set_order_by(&mut self, order_by: Vec<SortItem>) {
        match self {
            Projection::Regular(r) => r.order_by = order_by,
            Projection::Aggregate(a) => a.order_by = order_by,
            Projection::Distinct(d) => d.order_by = order_by,
        }
    }

    pub fn set_pagination(&mut self, pagination: Pagination) {
        match self {
            Projection::Regular(r) => r.pagination = pagination,
            Projection::Aggregate(a) => a.pagination = pagination,
            Projection::Distinct(d) => d.pagination = pagination,
        }
    }

    pub fn set_filter(&mut self, filter: FilterExprs) {
        match self {
            Projection::Regular(r) => r.filter = filter,
            Projection::Aggregate(a) => a.filter = filter,
            Projection::Distinct(d) => d.filter = filter,
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

pub struct Unwind {
    pub variable: Variable,
    pub expr: Expr,
}

impl Unwind {
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
        add_common_projection_fields(&mut fields, &self.order_by, &self.pagination, &self.filter);

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
        add_common_projection_fields(&mut fields, &self.order_by, &self.pagination, &self.filter);

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
        add_common_projection_fields(&mut fields, &self.order_by, &self.pagination, &self.filter);

        XmlNode::simple_record("Distinct", fields, vec![])
    }
}

fn add_common_projection_fields<'a>(
    fields: &mut Vec<(&'a str, Pretty<'a>)>,
    order_by: &'a [SortItem],
    pagination: &'a Pagination,
    filter: &'a FilterExprs,
) {
    if !order_by.is_empty() {
        fields.push(("order_by", pretty_order_items(order_by)));
    }
    if !pagination.is_empty() {
        fields.push(("pagination", Pretty::display(pagination)));
    }
    if !filter.is_true() {
        fields.push(("filter", Pretty::display(&filter.pretty())));
    }
}

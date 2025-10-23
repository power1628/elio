use std::collections::HashSet;

use mojito_common::data_type::DataType;
use mojito_parser::ast;

use crate::variable::VariableName;

pub struct ScopeItem {
    pub is_argument: bool,
    // symbol in original query
    pub symbol: String,
    // variable bound to this symbel
    pub variable: VariableName,
    pub expr: HashSet<ast::Expr>,
    pub typ: DataType,
}

pub struct Scope {
    items: Vec<ScopeItem>,
}

impl Scope {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    pub fn resolve_expr(&self, expr: &ast::Expr) -> Option<&ScopeItem> {
        self.items.iter().find(|item| item.expr.contains(expr))
    }
}

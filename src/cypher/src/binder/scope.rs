use std::collections::HashSet;

use mojito_common::data_type::DataType;
use mojito_parser::ast;

use crate::variable::{Variable, VariableName};

#[derive(Debug, Clone)]
pub struct ScopeItem {
    // pub is_argument: bool,
    // symbol in original query
    // none for anonymous variable
    pub symbol: Option<String>,
    // variable bound to this symbel
    pub variable: VariableName,
    pub expr: HashSet<ast::Expr>,
    pub typ: DataType,
}

impl ScopeItem {
    pub fn new_variable(variable: VariableName, symbol: Option<String>, typ: DataType) -> Self {
        Self {
            symbol,
            variable,
            expr: HashSet::new(),
            typ,
        }
    }

    pub fn as_variable(&self) -> Variable {
        Variable::new(&self.variable, &self.typ)
    }
}

#[derive(Debug, Clone)]
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

    pub fn resolve_symbol(&self, name: &str) -> Option<&ScopeItem> {
        self.items.iter().find(|item| item.symbol.as_deref() == Some(name))
    }

    pub fn add_item(&mut self, item: ScopeItem) {
        self.items.push(item);
    }
}

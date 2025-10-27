use std::collections::HashSet;

use mojito_common::data_type::DataType;
use mojito_parser::ast;

use crate::{
    expr::{Expr, VariableRef},
    variable::{Variable, VariableName},
};

#[derive(Debug, Clone)]
pub struct ScopeItem {
    // symbol in original query
    // none for anonymous variable
    pub symbol: Option<String>,
    // variable bound to this symbel
    pub variable: VariableName,
    pub expr: HashSet<ast::Expr>,
    pub typ: DataType,
}

impl ScopeItem {
    pub fn new_variable(variable: VariableName, symbol: Option<&str>, typ: DataType) -> Self {
        Self {
            symbol: symbol.map(|s| s.to_string()),
            variable,
            expr: HashSet::new(),
            typ,
        }
    }

    pub fn as_variable(&self) -> Variable {
        Variable::new(&self.variable, &self.typ)
    }

    pub fn as_expr(&self) -> Expr {
        VariableRef::new_unchecked(self.variable.clone(), self.typ.clone()).into()
    }

    pub fn is_anonymous(&self) -> bool {
        self.symbol.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub items: Vec<ScopeItem>,
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

    pub fn resolve_variable(&self, variable: &VariableName) -> Option<&ScopeItem> {
        self.items.iter().find(|item| &item.variable == variable)
    }

    pub fn add_item(&mut self, item: ScopeItem) {
        self.items.push(item);
    }

    pub fn symbol_items(&self) -> impl Iterator<Item = &ScopeItem> {
        self.items.iter().filter(|item| item.symbol.is_some())
    }
}

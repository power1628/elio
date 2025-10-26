use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_expr::func::sig::FuncDef;

use crate::{
    binder::{expr::ExprContext, scope::Scope},
    statement::StmtContext,
    variable::VariableGenerator,
};
mod builder;
pub mod expr;
pub mod label_expr;
pub mod match_;
pub mod pattern;
pub mod query;
pub mod scope;

/// Context to bind a query
#[derive(Debug, Clone)]
pub struct BindContext<'a> {
    pub sctx: &'a StmtContext<'a>,
    // TODO(pgao): seems outer_scopes is not needed?
    pub outer_scopes: Vec<Scope>,
    pub variable_generator: Arc<VariableGenerator>,
    // TODO(pgao): semantic context like disable some semantics
}

impl<'a> BindContext<'a> {
    pub fn new(sctx: &'a StmtContext<'a>) -> Self {
        Self {
            sctx,
            outer_scopes: Vec::new(),
            variable_generator: Arc::new(VariableGenerator::default()),
        }
    }

    pub fn derive_expr_context(&'a self, scope: &'a Scope, name: &'a str) -> ExprContext<'a> {
        ExprContext {
            bctx: self,
            scope,
            name,
        }
    }
}

impl<'a> BindContext<'a> {
    pub fn catalog(&self) -> &Arc<Catalog> {
        &self.sctx.session.catalog
    }

    pub fn resolve_function(&self, name: &str) -> Option<&FuncDef> {
        self.catalog().get_function_by_name(name).map(|x| &x.func)
    }
}

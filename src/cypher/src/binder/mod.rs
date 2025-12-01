use std::sync::Arc;

use mojito_catalog::Catalog;
use mojito_expr::func::sig::FuncDef;

use crate::binder::expr::ExprContext;
use crate::binder::scope::Scope;
use crate::session::SessionContext;
use crate::variable::VariableGenerator;
mod builder;
pub mod create;
pub mod expr;
pub mod label_expr;
pub mod match_;
pub mod pattern;
pub mod project_body;
pub mod query;
pub mod scope;

/// Context to bind a query
#[derive(Debug, Clone)]
pub struct BindContext {
    pub sctx: Arc<dyn SessionContext>,
    // TODO(pgao): seems outer_scopes is not needed?
    pub outer_scopes: Vec<Scope>,
    pub variable_generator: Arc<VariableGenerator>,
    // TODO(pgao): semantic context like disable some semantics
}

impl BindContext {
    pub fn new(sctx: Arc<dyn SessionContext>) -> Self {
        Self {
            sctx,
            outer_scopes: Vec::new(),
            variable_generator: Arc::new(VariableGenerator::default()),
        }
    }

    pub fn derive_expr_context<'a>(&'a self, scope: &'a Scope, name: &'a str) -> ExprContext<'a> {
        ExprContext {
            bctx: self,
            scope,
            name,
            sema_flags: Default::default(),
        }
    }
}

impl BindContext {
    pub fn catalog(&self) -> &Arc<Catalog> {
        self.sctx.catalog()
    }

    pub fn resolve_function(&self, name: &str) -> Option<&FuncDef> {
        self.catalog().get_function_by_name(name).map(|x| &x.func)
    }
}

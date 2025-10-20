use std::rc::Rc;

use mojito_expr::func::sig::FuncDef;

use crate::{binder::scope::Scope, statement::StmtContext, variable::VariableGenerator};
pub mod expr;
pub mod pattern;
pub mod query;
pub mod scope;

/// Context to bind a query
pub struct BindContext<'a> {
    pub sctx: StmtContext<'a>,
    pub outer_scopes: Vec<Scope>,
    pub variable_generator: Rc<VariableGenerator>,
}

impl<'a> BindContext<'a> {
    pub fn resolve_function(&self, name: &str) -> Option<FuncDef> {
        todo!()
    }
}

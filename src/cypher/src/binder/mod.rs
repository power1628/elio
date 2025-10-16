use std::rc::Rc;

use crate::{binder::scope::Scope, statement::StmtContext, variable::VariableGenerator};
pub mod scope;

/// Context to bind a query
pub struct BindContext<'a> {
    pub sctx: StmtContext<'a>,
    pub outer_scopes: Vec<Scope>,
    pub variable_generator: Rc<VariableGenerator>,
}

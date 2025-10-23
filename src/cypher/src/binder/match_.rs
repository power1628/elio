use mojito_parser::ast;

use crate::{
    binder::{BindContext, builder::IrSingleQueryBuilder, scope::Scope},
    error::PlanError,
};

pub(crate) fn bind_match(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    in_scope: Scope,
    match_: &ast::MatchClause,
) -> Result<Scope, PlanError> {
    // add the pattern graph to builder
    todo!()
}

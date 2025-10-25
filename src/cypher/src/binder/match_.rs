use mojito_parser::ast;

use crate::{
    binder::{BindContext, builder::IrSingleQueryBuilder, scope::Scope},
    error::PlanError,
};

pub(crate) fn bind_match(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    mut scope: Scope,
    _match @ ast::MatchClause {
        optional,
        mode,
        pattern,
        where_,
    }: &ast::MatchClause,
) -> Result<Scope, PlanError> {
    // add the pattern graph to builder
    todo!()
}

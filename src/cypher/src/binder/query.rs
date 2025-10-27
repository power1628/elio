use mojito_parser::ast;

use crate::{
    binder::{
        BindContext, builder::IrSingleQueryBuilder, match_::bind_match, project_body::bind_return_items, scope::Scope,
    },
    error::PlanError,
    ir::query::{IrQuery, IrQueryRoot, IrSingleQuery, IrSingleQueryPart},
    statement::StmtContext,
};

pub fn bind_root_query(sctx: &StmtContext, query: ast::RegularQuery) -> Result<IrQueryRoot, PlanError> {
    let bctx = BindContext::new(sctx);
    todo!()
}

fn bind_query(bctx: &BindContext, query: ast::RegularQuery) -> Result<(IrQuery, Scope), PlanError> {
    todo!()
}

fn bind_single_query(bctx: &BindContext, query: ast::SingleQuery) -> Result<(IrSingleQuery, Scope), PlanError> {
    let ast::SingleQuery { clauses } = query;
    let mut builder = IrSingleQueryBuilder::new();
    let mut in_scope = Scope::empty();
    for clause in clauses.iter() {
        in_scope = match clause {
            ast::Clause::Create(create_clause) => todo!(),
            ast::Clause::Match(match_clause) => bind_match(bctx, &mut builder, in_scope, match_clause)?,
            ast::Clause::With(with_clause) => bind_with(bctx, &mut builder, in_scope, with_clause)?,
            ast::Clause::Return(return_clause) => bind_return(bctx, &mut builder, in_scope, return_clause)?,
            ast::Clause::Unwind(unwind_clause) => todo!(),
        };
    }
    Ok((builder.build(), in_scope))
}

/// Execution order of with clause is
///  - Project/Distinct/Aggregate/Unwind
///  - OrderBy
///  - Pagination
///  - Where
/// If the projection is an aggregation, the order by and where subclause
/// only sees variables defined in with clause.
/// Otherwise, the order by and where subclause sees all variables defined
/// in previous with clause and all variables defined in incomming scope
fn bind_with(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    in_scope: Scope,
    _with @ ast::WithClause {
        distinct,
        return_items,
        order_by,
        skip,
        limit,
        where_,
    }: &ast::WithClause,
) -> Result<Scope, PlanError> {
    todo!()
}

fn bind_return(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    in_scope: Scope,
    return_: &ast::ReturnClause,
) -> Result<Scope, PlanError> {
    todo!()
}

use mojito_parser::ast;

use crate::{
    binder::{
        BindContext,
        builder::IrSingleQueryBuilder,
        expr::bind_where,
        match_::bind_match,
        project_body::{ClauseKind, bind_order_by, bind_pagination, bind_return_items},
        scope::Scope,
    },
    error::PlanError,
    ir::query::{IrQuery, IrQueryRoot, IrSingleQuery},
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

fn bind_create(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    mut in_scope: Scope,
    _create @ ast::CreateClause { pattern }: &ast::CreateClause,
) -> Result<Scope, PlanError> {
    todo!()
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
    mut in_scope: Scope,
    _with @ ast::WithClause {
        distinct,
        return_items,
        order_by,
        skip,
        limit,
        where_,
    }: &ast::WithClause,
) -> Result<Scope, PlanError> {
    // remove anonymous variables in in_scope
    in_scope.remove_anonymous();
    // bind projection
    let scope = bind_return_items(bctx, builder, in_scope, *distinct, &ClauseKind::With, return_items)?;
    // bind order by
    if let Some(order_by) = order_by {
        bind_order_by(bctx, builder, &scope, order_by)?;
    }
    // bind pagination
    bind_pagination(bctx, builder, &scope, skip.as_deref(), limit.as_deref())?;
    // bind where_
    if let Some(where_) = where_ {
        let filter = bind_where(bctx, &scope, where_)?;
        builder.tail_mut().unwrap().horizon.set_filter(filter);
    }
    Ok(scope)
}

fn bind_return(
    bctx: &BindContext,
    builder: &mut IrSingleQueryBuilder,
    mut in_scope: Scope,
    _return @ ast::ReturnClause {
        distinct,
        return_items,
        order_by,
        skip,
        limit,
    }: &ast::ReturnClause,
) -> Result<Scope, PlanError> {
    // remove anonymous variables in in_scope
    in_scope.remove_anonymous();
    // bind projection
    let scope = bind_return_items(bctx, builder, in_scope, *distinct, &ClauseKind::Return, return_items)?;
    // bind order by
    if let Some(order_by) = order_by {
        bind_order_by(bctx, builder, &scope, order_by)?;
    }
    // bind pagination
    bind_pagination(bctx, builder, &scope, skip.as_deref(), limit.as_deref())?;
    Ok(scope)
}

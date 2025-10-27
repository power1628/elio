use core::f64;
use paste::paste;

use itertools::Itertools;
use mojito_catalog::FunctionCatalog;
use mojito_common::data_type::DataType;
use mojito_expr::func::sig::FuncImpl;
use mojito_parser::ast;
use mojito_storage::codec::TokenKind;

use crate::{
    binder::{BindContext, scope::Scope},
    error::{PlanError, SemanticError},
    expr::{AggCall, Expr, ExprNode, FuncCall, IrToken, PropertyAccess, VariableRef, value::Constant},
    not_supported,
};

pub struct ExprContext<'a> {
    pub bctx: &'a BindContext<'a>,
    pub scope: &'a Scope,
    pub name: &'a str,
    pub sema_flags: ExprSemanticFlag,
}
impl<'a> ExprContext<'a> {}

#[derive(Default)]
pub struct ExprSemanticFlag(u64);

const EXPR_REJECT_OUTER_REFERENCE: u64 = 0x1;
const EXPR_REJECT_AGGREGATE: u64 = 0x2;

macro_rules! impl_sema_flag {
    ($flag:ident, $mask:ident) => {
        paste! {
            pub fn [<set_ $flag>](&mut self, value: bool) {
                if value {
                    self.0 |= $mask;
                } else {
                    self.0 &= !$mask;
                }
            }
            pub fn [<$flag>](&self) -> bool {
                self.0 & $mask != 0
            }
        }
    };
}

impl ExprSemanticFlag {
    impl_sema_flag!(reject_outer_reference, EXPR_REJECT_OUTER_REFERENCE);
    impl_sema_flag!(reject_aggregate, EXPR_REJECT_AGGREGATE);
}

pub fn bind_expr(ectx: &ExprContext, outer_scope: &[Scope], expr: &ast::Expr) -> Result<Expr, PlanError> {
    match expr {
        ast::Expr::Literal { lit } => bind_constant(ectx, lit).map(Into::into),
        ast::Expr::Variable { name } => bind_variable(ectx, name, outer_scope).map(Into::into),
        ast::Expr::Parameter { .. } => not_supported!("parameter binding"),
        ast::Expr::MapExpression { .. } => not_supported!("map projection"),
        ast::Expr::PropertyAccess { map, key } => {
            let expr = bind_expr(ectx, outer_scope, map)?;
            // resolve property keys
            let token: IrToken = ectx.bctx.catalog().get_token_id(key, TokenKind::PropertyKey).into();
            // TODO(pgao): maybe we can resolve the property types here
            let pa = PropertyAccess::new_unchecked(expr.boxed(), &token, &DataType::Any);
            Ok(pa.into())
        }
        ast::Expr::Unary { op, oprand } => bind_unary(ectx, outer_scope, op, oprand),
        ast::Expr::Binary { left, op, right } => bind_binary(ectx, outer_scope, left, op, right),
        ast::Expr::FunctionCall { name, distinct, args } => bind_func_call(ectx, outer_scope, name, *distinct, args),
    }
}

fn bind_constant(_ectx: &ExprContext, lit: &ast::Literal) -> Result<Constant, PlanError> {
    match lit {
        ast::Literal::Boolean(b) => Ok(Constant::boolean(*b)),
        ast::Literal::Integer(i) => {
            if let Ok(i) = i.parse::<i64>() {
                Ok(Constant::integer(i))
            } else {
                Err(SemanticError::invalid_literal(&DataType::Integer, &lit.to_string()).into())
            }
        }
        ast::Literal::Float(f) => {
            if let Ok(f) = f.parse::<f64>() {
                Ok(Constant::float(f))
            } else {
                Err(SemanticError::invalid_literal(&DataType::Float, &lit.to_string()).into())
            }
        }
        ast::Literal::String(s) => Ok(Constant::string(s.clone())),
        ast::Literal::Null => Ok(Constant::null()),
        ast::Literal::Inf => Ok(Constant::float(f64::INFINITY)),
    }
}

fn bind_variable(ectx: &ExprContext, name: &str, outer_scope: &[Scope]) -> Result<VariableRef, PlanError> {
    let item = ectx.scope.resolve_symbol(name);
    if ectx.sema_flags.reject_outer_reference() {
        if item.is_none() {
            return Err(SemanticError::variable_not_defined(name, ectx.name).into());
        }
    }
    // bind variable in outer scope
    for scope in outer_scope.iter() {
        let item = scope.resolve_symbol(name);
        if let Some(item) = item {
            return Ok(VariableRef::from_variable(&item.as_variable()));
        }
    }
    Err(SemanticError::variable_not_defined(name, ectx.name).into())
}

fn bind_unary(
    ectx: &ExprContext,
    outer_scope: &[Scope],
    op: &ast::UnaryOperator,
    oprand: &ast::Expr,
) -> Result<Expr, PlanError> {
    let oprand = bind_expr(ectx, outer_scope, oprand)?;
    let args = vec![oprand];

    // SAFETY: builtin operator are always ok
    let func_name = op.as_func_name();
    let (_func_impl, _is_agg, typ) = resolve_func(ectx, func_name, &args)?;
    let func_call = FuncCall::new_unchecked(func_name.to_string(), args, typ);
    Ok(func_call.into())
}

fn bind_binary(
    ectx: &ExprContext,
    outer_scope: &[Scope],
    left: &ast::Expr,
    op: &ast::BinaryOperator,
    right: &ast::Expr,
) -> Result<Expr, PlanError> {
    let left = bind_expr(ectx, outer_scope, left)?;
    let right = bind_expr(ectx, outer_scope, right)?;
    let args = vec![left, right];

    // SAFETY: builtin operator are always ok
    let func_name = op.as_func_name();
    let (_func_impl, _is_agg, typ) = resolve_func(ectx, func_name, &args)?;
    let func_call = FuncCall::new_unchecked(func_name.to_string(), args, typ);
    Ok(func_call.into())
}

fn bind_func_call(
    ectx: &ExprContext,
    outer_scope: &[Scope],
    name: &str,
    distinct: bool,
    args: &[ast::Expr],
) -> Result<Expr, PlanError> {
    let args = args
        .iter()
        .map(|x| bind_expr(ectx, outer_scope, x))
        .collect::<Result<Vec<_>, _>>()?;

    let (_func_impl, is_agg, typ) = resolve_func(ectx, name, &args)?;
    if is_agg {
        let agg = AggCall::new_unchecked(name.to_string(), args, distinct, typ);
        Ok(agg.into())
    } else {
        if distinct {
            return Err(SemanticError::distinct_not_allowed(name).into());
        }
        let func_call = FuncCall::new_unchecked(name.to_string(), args, typ);
        Ok(func_call.into())
    }
}

fn resolve_func(ectx: &ExprContext, name: &str, args: &[Expr]) -> Result<(FuncImpl, bool, DataType), PlanError> {
    let FunctionCatalog { name, func } = ectx
        .bctx
        .catalog()
        .get_function_by_name(name)
        .ok_or(PlanError::from(SemanticError::unknown_function(name, ectx.name)))?;

    let is_agg = func.is_agg;

    let args_types = args.iter().map(|x| x.typ()).collect_vec();
    // select function implementations
    for func_impl in func.impls.iter() {
        if let Some(ret) = func_impl.matches(&args_types) {
            return Ok((func_impl.clone(), is_agg, ret));
        }
    }

    // found no function matches the signature
    Err(SemanticError::invalid_function_arg_types(name, &args_types, ectx.name).into())
}

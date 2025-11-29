use mojito_common::IrToken;
use mojito_common::schema::{Name2ColumnMap, Schema};
use mojito_cypher::expr::{Constant, CreateMap, Expr, PropertyAccess, VariableRef};
use mojito_expr::impl_::constant::ConstantExpr;
use mojito_expr::impl_::create_map::CreateMapExpr;
use mojito_expr::impl_::property_access::PropertyAccessExpr;
use mojito_expr::impl_::variable_ref::VariableRefExpr;
use mojito_expr::impl_::{BoxedExpression, Expression};

use crate::builder::{BuildError, ExecutorBuildContext};

pub struct BuildExprContext<'a> {
    pub schema: &'a Schema,
    pub name2col: Name2ColumnMap,
    pub ctx: &'a ExecutorBuildContext,
}

impl<'a> BuildExprContext<'a> {
    pub fn new(schema: &'a Schema, ctx: &'a ExecutorBuildContext) -> Self {
        let name2col = schema.name_to_col_map();
        Self { schema, name2col, ctx }
    }
}

pub(crate) fn build_expression(ctx: &BuildExprContext<'_>, expr: &Expr) -> Result<BoxedExpression, BuildError> {
    match expr {
        Expr::VariableRef(variable_ref) => build_variable(ctx, variable_ref),
        Expr::PropertyAccess(property_access) => build_property_access(ctx, property_access),
        Expr::Constant(constant) => build_constant(ctx, constant),
        Expr::FuncCall(_func_call) => todo!(),
        Expr::AggCall(_agg_call) => todo!(),
        Expr::Subquery(_subquery) => todo!(),
        Expr::Label(_label_expr) => todo!(),
        Expr::CreateMap(create_map) => build_create_map(ctx, create_map),
    }
}

fn build_variable(ctx: &BuildExprContext<'_>, variable_ref: &VariableRef) -> Result<BoxedExpression, BuildError> {
    let col = ctx
        .name2col
        .get(&variable_ref.name)
        .ok_or_else(|| BuildError::VariableNotFound(variable_ref.name.clone()))?;
    let typ = ctx.schema.column(*col).typ.clone();
    Ok(VariableRefExpr::new(*col, typ).boxed())
}

fn build_property_access(
    ctx: &BuildExprContext<'_>,
    _property_access @ PropertyAccess { expr, property, .. }: &PropertyAccess,
) -> Result<BoxedExpression, BuildError> {
    let input = build_expression(ctx, expr)?;
    let token = match property {
        IrToken::Resolved(id) => id,
        IrToken::Unresolved(key) => {
            return Err(BuildError::UnresolvedToken(key.clone()));
        }
    };
    let expr = PropertyAccessExpr::new(input, *token);
    Ok(expr.boxed())
}

fn build_constant(_ctx: &BuildExprContext<'_>, constant: &Constant) -> Result<BoxedExpression, BuildError> {
    Ok(ConstantExpr {
        value: constant.data.clone(),
        typ: constant.typ.clone(),
    }
    .boxed())
}

fn build_create_map(ctx: &BuildExprContext<'_>, create_map: &CreateMap) -> Result<BoxedExpression, BuildError> {
    let properties = create_map
        .properties
        .iter()
        .map(|(token, expr)| build_expression(ctx, expr).map(|expr| (token.clone(), expr)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CreateMapExpr { properties }.boxed())
}

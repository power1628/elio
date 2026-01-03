use mojito_common::IrToken;
use mojito_common::data_type::DataType;
use mojito_common::schema::{Name2ColumnMap, Schema};
use mojito_cypher::expr;
use mojito_cypher::expr::{Constant, CreateStruct, Expr, ExprNode, PropertyAccess, VariableRef};
use mojito_expr::func::FUNCTION_REGISTRY;
use mojito_expr::impl_::constant::ConstantExpr;
use mojito_expr::impl_::create_struct::CreateStructExpr;
use mojito_expr::impl_::field_access::FieldAccessExpr;
use mojito_expr::impl_::func_call::FuncCallExpr;
use mojito_expr::impl_::label::HasLabelExpr;
use mojito_expr::impl_::project_path::ProjectPathExpr;
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
        Expr::FuncCall(func_call) => build_func_call(ctx, func_call),
        Expr::AggCall(_agg_call) => todo!(),
        Expr::Subquery(_subquery) => todo!(),
        Expr::HasLabel(has_label) => build_has_label(ctx, has_label),
        Expr::CreateStruct(create_map) => build_create_map(ctx, create_map),
        Expr::ProjectPath(project_path) => build_project_path(ctx, project_path),
    }
}

fn build_variable(ctx: &BuildExprContext<'_>, variable_ref: &VariableRef) -> Result<BoxedExpression, BuildError> {
    let col = ctx
        .name2col
        .get(&variable_ref.name)
        .ok_or_else(|| BuildError::variable_not_found(variable_ref.name.clone()))?;
    let typ = ctx.schema.column(*col).typ.clone();
    Ok(VariableRefExpr::new(*col, typ).boxed())
}

fn build_property_access(
    ctx: &BuildExprContext<'_>,
    property_access @ PropertyAccess { expr, property, .. }: &PropertyAccess,
) -> Result<BoxedExpression, BuildError> {
    let input = build_expression(ctx, expr)?;
    let _token = match property {
        IrToken::Resolved { name: _, token } => token,
        IrToken::Unresolved(key) => {
            return Err(BuildError::unresolved_token((*key).to_string()));
        }
    };
    let expr = FieldAccessExpr::new(input, property.clone(), property_access.typ());
    Ok(expr.boxed())
}

fn build_constant(_ctx: &BuildExprContext<'_>, constant: &Constant) -> Result<BoxedExpression, BuildError> {
    Ok(ConstantExpr {
        value: constant.data.clone(),
        // After null coercion in binder, all constants should have a type.
        // If not, fall back to Any type for runtime handling.
        typ: constant.typ.clone().unwrap_or(DataType::Any),
    }
    .boxed())
}

fn build_func_call(ctx: &BuildExprContext<'_>, func_call: &expr::FuncCall) -> Result<BoxedExpression, BuildError> {
    let args = func_call
        .args
        .iter()
        .map(|expr| build_expression(ctx, expr))
        .collect::<Result<Vec<_>, _>>()?;

    let func_impl = FUNCTION_REGISTRY.get_func_impl(&func_call.func_id);

    Ok(FuncCallExpr {
        inputs: args,
        func: func_impl.func,
        typ: func_call.typ(),
    }
    .boxed())
}

fn build_has_label(ctx: &BuildExprContext<'_>, has_label: &expr::HasLabel) -> Result<BoxedExpression, BuildError> {
    let entity = build_expression(ctx, &has_label.entity)?;
    Ok(HasLabelExpr {
        entity,
        label: has_label.label_or_rel.clone(),
    }
    .boxed())
}

fn build_create_map(ctx: &BuildExprContext<'_>, create_map: &CreateStruct) -> Result<BoxedExpression, BuildError> {
    let properties = create_map
        .properties
        .iter()
        .map(|(token, expr)| build_expression(ctx, expr).map(|expr| (token.clone(), expr)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CreateStructExpr {
        fields: properties,
        typ: create_map.typ().clone(),
        physical_type: create_map.typ().physical_type(),
    }
    .boxed())
}

fn build_project_path(
    ctx: &BuildExprContext<'_>,
    project_path: &expr::ProjectPath,
) -> Result<BoxedExpression, BuildError> {
    let inputs = project_path
        .step_variables()
        .into_iter()
        .map(|varref| build_expression(ctx, &varref.into()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ProjectPathExpr {
        inputs,
        typ: project_path.typ(),
    }
    .boxed())
}

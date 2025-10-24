use std::collections::HashSet;

use mojito_common::data_type::DataType;
use mojito_parser::ast::{self, NodePattern};
use mojito_storage::codec::TokenKind;

use crate::{
    binder::{
        BindContext,
        builder::IrSingleQueryBuilder,
        expr::bind_expr,
        label_expr::bind_label_expr,
        scope::{Scope, ScopeItem},
    },
    error::PlanError,
    expr::{Expr, FilterExprs, property_access::PropertyAccess},
    ir::node_connection::RelPattern,
    variable::{Variable, VariableName},
};

#[derive(Debug, Clone)]
pub struct ExprContext<'a> {
    pub bctx: &'a BindContext<'a>,
    // context name, used for error messages
    pub name: &'a str,
    // true on allow update in this contex
    pub allow_update: bool,
}

pub(crate) fn bind_pattern(pctx: &ExprContext, pattern: &[ast::PatternPart]) -> Result<(), PlanError> {
    todo!()
}

/// - SimplePattern: bind and pull the filter into WHERE clause
///   Return (Vec<NodeVar>, Vec<RelPattern>, Filter)
/// - QuantifiedPathPattern: generate selective path pattern
///   SemanticCheck: RejectNestedSelector
///   Return SelectivePathPattern
/// - Path variable, generate path expression
///
/// After binding, return
///   - PathPattern
///   - Filter(post filter)
///   - path variable
///
pub(crate) fn bind_pattern_part(pctx: &ExprContext, pattern: &ast::PatternPart) -> Result<(), PlanError> {
    let ast::PatternPart {
        variable,
        selector,
        factors,
    } = pattern;
    todo!()
}

struct BoundSimplePathPattern {
    pub nodes: Vec<VariableName>,
    pub rels: Vec<RelPattern>,
    pub post_filter: FilterExprs,
    pub arguments: HashSet<VariableName>,
}

fn bind_simple_pattern(
    pctx: &ExprContext,
    // current scope only record introduced scope items
    mut scope: Scope,
    simple: &ast::SimplePathPattern,
) -> Result<(BoundSimplePathPattern, Scope), PlanError> {
    let ast::SimplePathPattern { nodes, relationships } = simple;

    let mut arguments = HashSet::new();
    for NodePattern {
        variable,
        label_expr,
        properties,
        predicate,
    } in nodes
    {
        // bind variable
        let var = {
            if let Some(name) = variable {
                if let Some((variable, is_argument)) = resolve_node_variable(pctx, &scope, name)? {
                    if is_argument {
                        arguments.insert(variable.clone());
                    }
                    variable
                } else {
                    // introduce a new named variable
                    let var_name = pctx.bctx.variable_generator.next_name();
                    let item = ScopeItem::new_variable(var_name, None, DataType::Node);
                    let var = item.as_variable();
                    scope.add_item(item);
                    var
                }
            } else {
                // introduce an anonymous node variable
                let var_name = pctx.bctx.variable_generator.next_name();
                let item = ScopeItem::new_variable(var_name, None, DataType::Node);
                let var = item.as_variable();
                scope.add_item(item);
                var
            }
        };

        // pre-filter
        let mut filter = FilterExprs::empty();

        // bind label expr
        if let Some(label_expr) = label_expr {
            let label_expr = bind_label_expr(pctx, Expr::from_variable(&var).boxed(), label_expr)?;
            filter.push(*label_expr);
        }

        // bind properties
        if let Some(props) = properties {
            let props = bind_properties(pctx, &var, props)?;
            filter.and(props);
        }
        // TODO(pgao): bind predicate, in parser we do not support predicate right now
        if let Some(_) = predicate {
            return Err(PlanError::NotSupported(
                "predicate in pattern not supported".to_string(),
            ));
        }
    }

    todo!()
}

// If bound, return (variable, is_argument)
fn resolve_node_variable(pctx: &ExprContext, scope: &Scope, name: &str) -> Result<Option<(Variable, bool)>, PlanError> {
    // find if variable already defined in in_scope
    // check in pctx for imported variables
    if let Some(item) = scope.resolve_symbol(name) {
        if item.typ == DataType::Node {
            return Ok(Some((Variable::new(&item.variable, &item.typ), false)));
        } else {
            return Err(PlanError::semantic_err(format!("Expected node at {}", name)));
        }
    }
    // check if this is an outer reference
    for scope in pctx.bctx.outer_scopes.iter() {
        if let Some(item) = scope.resolve_symbol(name) {
            if item.typ == DataType::Node {
                return Ok(Some((Variable::new(&item.variable, &item.typ), true)));
            } else {
                return Err(PlanError::semantic_err(format!("Expected node at {}", name)));
            }
        }
    }
    Ok(None)
}

fn bind_properties(pctx: &ExprContext, var: &Variable, props: &ast::Expr) -> Result<FilterExprs, PlanError> {
    let mut filter = FilterExprs::empty();
    if let ast::Expr::MapExpression { keys, values } = props {
        for (key, value) in keys.iter().zip(values.iter()) {
            let token = pctx.bctx.catalog().get_token_id(key, TokenKind::PropertyKey).into();
            let value = bind_expr(pctx, value)?;
            // TODO(pgao): maybe we can inference the properties here
            let prop = Expr::PropertyAccess(PropertyAccess::new_unchecked(
                Box::new(Expr::from_variable(var.clone())),
                &token,
                &DataType::Any,
            ));
            let equal = prop.equal(value);
            filter.push(equal);
        }
        Ok(filter)
    } else {
        // this should be an parser error
        unreachable!()
    }
}

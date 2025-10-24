use std::{collections::HashSet, ops::Range};

use indexmap::IndexSet;
use mojito_common::data_type::DataType;
use mojito_parser::ast::{self, NodePattern, RelationshipPattern};
use mojito_storage::codec::TokenKind;

use crate::{
    binder::{
        BindContext,
        expr::{ExprContext, bind_expr},
        label_expr::bind_label_expr,
        scope::{Scope, ScopeItem},
    },
    error::PlanError,
    expr::{Expr, FilterExprs, IrToken, property_access::PropertyAccess},
    ir::node_connection::{PatternLength, RelPattern},
    variable::{Variable, VariableName},
};

#[derive(Debug, Clone)]
pub struct PatternContext<'a> {
    pub bctx: &'a BindContext<'a>,
    // context name, used for error messages
    pub name: &'a str,
    // true on allow update in this contex
    pub allow_update: bool,
}

impl<'a> PatternContext<'a> {
    pub fn derive_expr_context(&self, scope: &Scope, name: &str) -> ExprContext<'a> {
        todo!()
    }
}

pub(crate) fn bind_pattern(pctx: &PatternContext, pattern: &[ast::PatternPart]) -> Result<(), PlanError> {
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
pub(crate) fn bind_pattern_part(pctx: &PatternContext, pattern: &ast::PatternPart) -> Result<(), PlanError> {
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
    pub arguments: IndexSet<VariableName>,
}

impl BoundSimplePathPattern {
    /// Non repeated relationships in the path pattern
    /// TODO(pgao): use scope to recover the repated relationship
    pub fn semantic_check(&self, _scope: &Scope) -> Result<(), PlanError> {
        let rel_set: HashSet<_> = self.rels.iter().map(|x| x.variable.clone()).collect();
        if rel_set.len() != self.rels.len() {
            return Err(PlanError::semantic_err(
                " repeated relationships not allowed in the path pattern".to_string(),
            ));
        }
        Ok(())
    }
}

fn bind_simple_pattern(
    pctx: &PatternContext,
    // current scope only record introduced scope items
    mut scope: Scope,
    _simple @ ast::SimplePathPattern { nodes, relationships }: &ast::SimplePathPattern,
) -> Result<(BoundSimplePathPattern, Scope), PlanError> {
    let mut arguments = IndexSet::new();
    let mut ir_nodes = vec![];
    let mut ir_rels = vec![];
    let mut filter = FilterExprs::empty();

    for NodePattern {
        variable,
        label_expr,
        properties,
        predicate,
    } in nodes
    {
        // bind variable
        let (var, is_argument) = bind_variable(pctx, &mut scope, variable.as_deref(), &VariableKind::Node)?;
        if is_argument {
            arguments.insert(var.name.clone());
        }

        // pre-filter
        let mut node_filter = FilterExprs::empty();

        // bind label expr
        if let Some(label_expr) = label_expr {
            let label_expr = bind_label_expr(pctx, Expr::from_variable(&var).boxed(), label_expr)?;
            node_filter.push(*label_expr);
        }

        // bind properties
        if let Some(props) = properties {
            let props = bind_properties(pctx, &var, props)?;
            node_filter = node_filter.and(props);
        }
        // TODO(pgao): bind predicate, in parser we do not support predicate right now
        if let Some(_) = predicate {
            return Err(PlanError::NotSupported(
                "predicate in pattern not supported".to_string(),
            ));
        }

        filter = filter.and(node_filter);
        ir_nodes.push(var.name);
    }

    for (
        i,
        RelationshipPattern {
            variable,
            label_expr,
            properties,
            predicate,
            length,
            direction,
        },
    ) in relationships.iter().enumerate()
    {
        let (var, is_argument) = bind_variable(pctx, &mut scope, variable.as_deref(), &VariableKind::Rel)?;
        if is_argument {
            arguments.insert(var.name.clone());
        }
        // for relationship type, label expr should either be single reltype or reltype conjuncted with OR or NONE
        // bind label expr
        let mut reltypes: Vec<IrToken> = vec![];
        if let Some(label_expr) = label_expr {
            if !label_expr.contains_only_or() {
                return Err(PlanError::semantic_err(
                    "relationship type can only be conjuncted with OR",
                ));
            }
            let rel_types = label_expr.extract_relationship_types();
            for rtype in rel_types {
                let token = pctx
                    .bctx
                    .catalog()
                    .get_token_id(&rtype, TokenKind::RelationshipType)
                    .into();
                reltypes.push(token);
            }
        }

        let mut rel_filter = FilterExprs::empty();
        // bind properties
        if let Some(props) = properties {
            let props = bind_properties(pctx, &var, props)?;
            rel_filter = rel_filter.and(props);
        }
        // TODO(pgao): bind predicate, in parser we do not support predicate right now
        if let Some(_) = predicate {
            return Err(PlanError::NotSupported(
                "predicate in pattern not supported".to_string(),
            ));
        }

        // bind length
        let length = match length {
            None => PatternLength::Simple,
            Some(None) => PatternLength::Var { min: 0, max: None },
            Some(Some(Range { start, end })) => PatternLength::Var {
                min: *start as i64,
                max: if *end == usize::MAX { None } else { Some(*end as i64) },
            },
        };

        filter = filter.and(rel_filter);

        let rel = RelPattern {
            variable: var.name,
            endpoints: (ir_nodes[i].clone(), ir_nodes[i + 1].clone()),
            dir: *direction,
            types: reltypes,
            length,
        };
        ir_rels.push(rel);
    }

    let bound = BoundSimplePathPattern {
        nodes: ir_nodes,
        rels: ir_rels,
        post_filter: filter,
        arguments,
    };
    bound.semantic_check(&scope)?;

    Ok((bound, scope))
}

enum VariableKind {
    Node,
    Rel,
}

impl VariableKind {
    pub fn typ(&self) -> DataType {
        match self {
            VariableKind::Node => DataType::Node,
            VariableKind::Rel => DataType::Relationship,
        }
    }
}

struct PatternBuilder {
    pub arguments: IndexSet<Variable>,
    pub nodes: IndexSet<Variable>,
}

fn bind_variable(
    pctx: &PatternContext,
    scope: &mut Scope,
    name: Option<&str>, // None for anonymous variable
    kind: &VariableKind,
) -> Result<(Variable, bool), PlanError> {
    if let Some(name) = name {
        // named variable
        if let Some((variable, is_argument)) = resolve_variable(pctx, scope, name)? {
            return Ok((variable, is_argument));
        } else {
            // introduce a new named variable
            let var_name = pctx.bctx.variable_generator.named(name);
            let item = ScopeItem::new_variable(var_name, Some(name), kind.typ());
            let var = item.as_variable();
            scope.add_item(item);
            return Ok((var, false));
        }
    } else {
        // introduce an anonymous node variable
        let var_name = pctx.bctx.variable_generator.unnamed();
        let item = ScopeItem::new_variable(var_name, None, kind.typ());
        let var = item.as_variable();
        scope.add_item(item);
        return Ok((var, false));
    }
}

// If bound, return (variable, is_argument)
fn resolve_variable(pctx: &PatternContext, scope: &Scope, name: &str) -> Result<Option<(Variable, bool)>, PlanError> {
    // find if variable already defined in in_scope
    // check in pctx for imported variables
    if let Some(item) = scope.resolve_symbol(name) {
        return Ok(Some((Variable::new(&item.variable, &item.typ), false)));
    }
    // check if this is an outer reference
    for scope in pctx.bctx.outer_scopes.iter() {
        if let Some(item) = scope.resolve_symbol(name) {
            return Ok(Some((Variable::new(&item.variable, &item.typ), true)));
        }
    }
    Ok(None)
}

fn bind_properties(pctx: &PatternContext, var: &Variable, props: &ast::Expr) -> Result<FilterExprs, PlanError> {
    let mut filter = FilterExprs::empty();
    if let ast::Expr::MapExpression { keys, values } = props {
        for (key, value) in keys.iter().zip(values.iter()) {
            let token = pctx.bctx.catalog().get_token_id(key, TokenKind::PropertyKey).into();
            let value = bind_expr(pctx, value)?;
            // TODO(pgao): maybe we can inference the properties here
            let prop = Expr::PropertyAccess(PropertyAccess::new_unchecked(
                Box::new(Expr::from_variable(var)),
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

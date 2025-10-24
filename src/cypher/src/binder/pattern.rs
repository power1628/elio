use std::{collections::HashSet, i64, ops::Range};

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
    ir::{
        node_connection::{NodeBinding, PatternLength, QuantifiedPathPattern, RelPattern, Repetition},
        path_pattern::PathPattern,
    },
    variable::{Variable, VariableName},
};

#[derive(Debug, Clone)]
pub struct PatternContext<'a> {
    pub bctx: &'a BindContext<'a>,
    // context name, used for error messages
    pub name: &'a str,
    // true on allow update in this contex
    pub allow_update: bool,
    // true on quantified path pattern not allowed
    pub reject_qpp: bool,
    // true on reject named path pattern
    pub reject_named_path: bool,
    // true on reject selective path pattenr
    pub reject_selective: bool,
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

/// return (PathPattern, PathName)
pub(crate) fn bind_pattern_part(
    pctx: &PatternContext,
    scope: &mut Scope,
    pattern: &ast::PatternPart,
) -> Result<(PathPattern, Option<String>), PlanError> {
    let ast::PatternPart {
        variable,
        selector,
        factors,
    } = pattern;
    todo!()
}

struct NodeConnectionExtra {
    pub outer: IndexSet<VariableName>,
    pub post_filter: FilterExprs,
}

pub struct PathPatternExtra {
    pub name: Option<String>,          // named path or not
    pub outer: IndexSet<VariableName>, // outer references used by this pattern
    pub post_filter: FilterExprs,      // post filter after match this pattern
}

fn bind_simple_pattern(
    pctx: &PatternContext,
    // current scope only record introduced scope items
    mut scope: Scope,
    _simple @ ast::SimplePathPattern { nodes, relationships }: &ast::SimplePathPattern,
) -> Result<(Vec<VariableName>, Vec<RelPattern>, NodeConnectionExtra, Scope), PlanError> {
    let mut ir_nodes = vec![];
    let mut ir_rels = vec![];
    let mut filter = FilterExprs::empty();
    let mut outer = IndexSet::default();

    for NodePattern {
        variable,
        label_expr,
        properties,
        predicate,
    } in nodes
    {
        // bind variable
        let (var, is_outer) = bind_variable(pctx, &mut scope, variable.as_deref(), &VariableKind::Node)?;
        if is_outer {
            outer.insert(var.name.clone());
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
        let (var, is_outer) = bind_variable(pctx, &mut scope, variable.as_deref(), &VariableKind::Rel)?;

        if is_outer {
            outer.insert(var.name.clone());
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
            variable: var.name.clone(),
            endpoints: (ir_nodes[i].clone(), ir_nodes[i + 1].clone()),
            dir: *direction,
            types: reltypes,
            length,
        };
        ir_rels.push(rel);
    }

    // no repeated relationship check
    {
        let rel_set: HashSet<_> = ir_rels.iter().map(|x| x.variable.clone()).collect();
        if rel_set.len() != ir_rels.len() {
            return Err(PlanError::semantic_err(
                " repeated relationships not allowed in the path pattern".to_string(),
            ));
        }
    }

    Ok((
        ir_nodes,
        ir_rels,
        NodeConnectionExtra {
            outer,
            post_filter: filter,
        },
        scope,
    ))
}

fn bind_quantified_path_pattern(
    pctx: &PatternContext,
    scope: &mut Scope,
    left: &VariableName,
    right: &VariableName,
    _qpp @ ast::QuantifiedPathPattern {
        non_selective_part,
        quantifier,
        filter,
    }: &ast::QuantifiedPathPattern,
) -> Result<(QuantifiedPathPattern, NodeConnectionExtra, Scope), PlanError> {
    let mut inner_pctx = pctx.clone();
    // quantified path pattern not allowed to be nested
    inner_pctx.reject_qpp = true;
    // quantified path pattern not allowed to have named path pattern
    inner_pctx.reject_named_path = true;
    inner_pctx.reject_selective = true;

    let (bound, _) = bind_pattern_part(&inner_pctx, scope, non_selective_part)?;

    let rels = bound
        .as_node_connections()
        .ok_or(PlanError::semantic_err("Node connections expected in QPP."))?
        .as_rels()
        .ok_or(PlanError::semantic_err("Only simple relationships allowed in QPP."))?;

    let left_binding = NodeBinding {
        inner: rels.first().unwrap().endpoints.0.clone(),
        outer: left.clone(),
    };
    let right_binding = NodeBinding {
        inner: rels.last().unwrap().endpoints.1.clone(),
        outer: right.clone(),
    };
    let repetition = match quantifier {
        ast::PatternQuantifier::Plus => Repetition { min: 1, max: None },
        ast::PatternQuantifier::Star => Repetition { min: 0, max: None },
        ast::PatternQuantifier::Fixed(n) => Repetition {
            min: *n as i64,
            max: Some(*n as i64),
        },
        ast::PatternQuantifier::Interval { lower, upper } => Repetition {
            min: lower.unwrap_or_default() as i64,
            max: upper.map(|x| x as i64),
        },
    };
    let qpp = QuantifiedPathPattern {
        left_binding,
        right_binding,
        rels,
        repetition: repetition
        node_grouping: todo!(),
        rel_grouping: todo!(),
    };

    todo!()
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

/// Return (Variable, is_outer)
/// When is_outer is true, means the pattern works inside an subquery
fn bind_variable(
    pctx: &PatternContext,
    scope: &mut Scope,
    name: Option<&str>, // None for anonymous variable
    kind: &VariableKind,
) -> Result<(Variable, bool), PlanError> {
    if let Some(name) = name {
        // named variable
        if let Some((variable, is_outer)) = resolve_variable(pctx, scope, name)? {
            return Ok((variable, is_outer));
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

// If bound, return (variable, is_outer)
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

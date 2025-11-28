use mojito_common::data_type::DataType;
use mojito_common::schema::Variable;
use mojito_parser::ast;

use crate::error::PlanError;
use crate::expr::Expr;
use crate::plan_node::{CreateNode, CreateNodeInner, CreateRel, CreateRelInner, PlanExpr};
use crate::planner::PlannerContext;

pub(super) fn plan_create(
    ctx: &mut PlannerContext,
    mut root: Box<PlanExpr>,
    _create @ crate::ir::CreatePattern { nodes, rels }: &crate::ir::CreatePattern,
) -> Result<Box<PlanExpr>, PlanError> {
    for node in nodes.iter() {
        root = plan_create_node(ctx, root, node)?;
    }

    for rel in rels.iter() {
        root = plan_create_rel(ctx, root, rel)?;
    }
    Ok(root)
}

fn plan_create_node(
    _ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    _create_node @ crate::ir::CreateNode {
        variable,
        labels,
        properties,
    }: &crate::ir::CreateNode,
) -> Result<Box<PlanExpr>, PlanError> {
    let inner = CreateNodeInner {
        input: root,
        labels: labels.clone().into_iter().collect(),
        properties: Expr::from(properties.clone()).boxed(),
        variable: Variable::new(&variable.clone(), &DataType::Node),
    };

    Ok(PlanExpr::CreateNode(CreateNode::new(inner)).boxed())
}

fn plan_create_rel(
    _ctx: &mut PlannerContext,
    root: Box<PlanExpr>,
    _create_rel @ crate::ir::CreateRel {
        variable,
        left,
        right,
        reltype,
        direction,
        properties,
    }: &crate::ir::CreateRel,
) -> Result<Box<PlanExpr>, PlanError> {
    let (start, end) = {
        match direction {
            ast::SemanticDirection::Outgoing => (left.clone(), right.clone()),
            ast::SemanticDirection::Incoming => (right.clone(), left.clone()),
            ast::SemanticDirection::Both => unreachable!("create rel must be directed"),
        }
    };
    let inner = CreateRelInner {
        input: root,
        reltype: reltype.clone(),
        start_node: Expr::from_variable(&Variable::new(&start, &DataType::Node)).boxed(),
        end_node: Expr::from_variable(&Variable::new(&end, &DataType::Node)).boxed(),
        properties: Expr::from(properties.clone()).boxed(),
        variable: Variable::new(&variable.clone(), &DataType::Rel),
    };

    Ok(PlanExpr::CreateRel(CreateRel::new(inner)).boxed())
}
